use super::{AuthError, AuthProvider, VerifyInfo};
pub use crate::password_hash::{
    HashError, HashFunction, IntoHashedFunction, VerifyHashedFunction, ARGON2_HASH_FUNCTION,
    BCRYPT_HASH_FUNCTION,
};
use crate::repository::{
    InnerEmailProviderBeforeInsert, InnerEmailProviderData, UserAuthPairBeforeInsert,
    UserAuthPairData,
};
use lettre::message::header::ContentType;
use lettre::{Message, SmtpTransport, Transport};
use sea_orm::{DatabaseConnection, TransactionError, TransactionTrait};
use std::sync::Arc;
use uuid::Uuid;

/// `auth_provider` in [UserAuthPairData]
const INNER_EMAIL_PROVIDER_NAME: &str = "inner_email_provider";

pub struct InnerEmailProvider {
    hash_function: &'static HashFunction,
    database_connection: Arc<DatabaseConnection>,
    template: EmailTemplateFunction,
    mailer: Arc<SmtpTransport>,
}

pub struct EmailContent {
    pub subject: String,
    pub from: String,
    pub to: String,
    pub content: String,
    pub content_type: ContentType,
}

pub type EmailTemplateFunction = fn(&VerifyInfo, &EmailAccount) -> EmailContent;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmailAccount {
    pub email: String,
    pub password: String,
}

impl InnerEmailProvider {
    pub fn new(
        hash_function: &'static HashFunction,
        database_connection: Arc<DatabaseConnection>,
        template: EmailTemplateFunction,
        mailer: Arc<SmtpTransport>,
    ) -> Self {
        Self {
            hash_function,
            database_connection,
            template,
            mailer,
        }
    }
}

#[async_trait::async_trait]
impl AuthProvider<EmailAccount> for InnerEmailProvider {
    async fn try_login(
        &self,
        account: &EmailAccount,
    ) -> Result<Option<UserAuthPairData>, AuthError> {
        let EmailAccount { email, password } = account;
        let maybe_user_record =
            InnerEmailProviderData::find_by_email(self.database_connection.as_ref(), email)
                .await
                .map_err(AuthError::DatabaseError)?;

        if maybe_user_record.is_none() {
            return Ok(None);
        }
        let user_record = maybe_user_record.unwrap();
        let is_password_correct =
            (self.hash_function.verify)(&password, &user_record.password_hash)
                .map_err(|e| e.into())?;
        if !is_password_correct {
            return Ok(None);
        }
        let auth_key = user_record.auth_key;

        UserAuthPairData::find_by_key(
            self.database_connection.as_ref(),
            INNER_EMAIL_PROVIDER_NAME,
            &auth_key.to_string(),
        )
        .await
        .map_err(AuthError::DatabaseError)
    }

    async fn try_register(
        &self,
        account: &EmailAccount,
        user_id: Uuid,
    ) -> Result<UserAuthPairData, AuthError> {
        let EmailAccount { email, password } = account;
        let maybe_user_record =
            InnerEmailProviderData::find_by_email(self.database_connection.as_ref(), email)
                .await
                .map_err(AuthError::DatabaseError)?;
        if maybe_user_record.is_some() {
            return Err(AuthError::ConflictingAccount);
        }

        let random_auth_key = uuid::Uuid::new_v4();
        let hashed_password = (self.hash_function.into_hashed)(password).map_err(|e| e.into())?;
        let provider_record = InnerEmailProviderBeforeInsert {
            email: email.clone(),
            password_hash: hashed_password,
            auth_key: random_auth_key,
        };
        let pair = UserAuthPairBeforeInsert {
            auth_provider: INNER_EMAIL_PROVIDER_NAME.to_owned(),
            auth_key: random_auth_key.to_string(),
            user_id,
        };
        let tr = self
            .database_connection
            .transaction(|tx| {
                Box::pin(async move {
                    InnerEmailProviderData::create(tx, provider_record)
                        .await
                        .map_err(AuthError::DatabaseError)?;
                    UserAuthPairData::create(tx, pair)
                        .await
                        .map_err(AuthError::DatabaseError)
                })
            })
            .await;
        match tr {
            Ok(pair) => Ok(pair),
            Err(e) => match e {
                TransactionError::Transaction(AuthError::DatabaseError(err)) => {
                    Err(AuthError::DatabaseError(err))
                }
                TransactionError::Connection(err) => Err(AuthError::DatabaseError(err)),
                _ => unreachable!(),
            },
        }
    }

    async fn send_verify(
        &self,
        account: &EmailAccount,
        verify_info: &VerifyInfo,
    ) -> Result<(), AuthError> {
        let email_content = (self.template)(verify_info, account);
        let email = Message::builder()
            .from(
                email_content
                    .from
                    .parse()
                    .map_err(|_| AuthError::VerifySendError("Invalid from email".to_owned()))?,
            )
            .to(email_content
                .to
                .parse()
                .map_err(|_| AuthError::VerifySendError("Invalid to email".to_owned()))?)
            .subject(email_content.subject)
            .header(email_content.content_type)
            .body(email_content.content)
            .map_err(|_| AuthError::VerifySendError("Failed to build email".to_owned()))?;
        match self.mailer.send(&email) {
            Ok(_) => Ok(()),
            Err(_) => Err(AuthError::VerifySendError(
                "Failed to send email".to_owned(),
            )),
        }
    }

    async fn check_verify_response(
        &self,
        account: &EmailAccount,
        verify_code: &str,
    ) -> Result<bool, AuthError> {
        let EmailAccount { email, .. } = account;
        let maybe_user_record =
            InnerEmailProviderData::find_by_email(self.database_connection.as_ref(), email)
                .await
                .map_err(AuthError::DatabaseError)?;
        if maybe_user_record.is_none() {
            return Ok(false);
        }
        let user_record = maybe_user_record.unwrap();
        if user_record.verify_code.is_none() {
            return Ok(false);
        }
        let correct_code = user_record.verify_code.unwrap();
        Ok(correct_code == verify_code)
    }
}
