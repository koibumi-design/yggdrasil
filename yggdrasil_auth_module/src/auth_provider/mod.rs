pub mod inner_email_provider;

use crate::repository::UserAuthPairData;
use sea_orm::DbErr;
use std::fmt::{Display, Formatter};
use uuid::Uuid;

#[derive(Debug, Eq, PartialEq)]
pub enum AuthError {
    ConnectionError(String),
    DatabaseError(DbErr),
    ConflictingAccount,
    VerifySendError(String),
    VerifyAlgorithmError(String),
}

#[derive(Debug, Clone)]
pub struct VerifyInfo {
    pub verify_code: String,
    pub service_name: String,
    pub user_account_description: String,
}

impl Display for AuthError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AuthError::ConnectionError(msg) => write!(f, "Connection error: {}", msg),
            AuthError::DatabaseError(err) => write!(f, "Database error: {:?}", err),
            AuthError::VerifySendError(msg) => write!(f, "Verify send error: {}", msg),
            AuthError::VerifyAlgorithmError(msg) => write!(f, "Verify algorithm error: {}", msg),
            AuthError::ConflictingAccount => write!(f, "Conflicting account"),
        }
    }
}

impl std::error::Error for AuthError {}

#[async_trait::async_trait]
pub trait AuthProvider<Account: Send + Sync + Sized + Clone>: Send + Sync + Sized {
    async fn try_login(&self, account: &Account) -> Result<Option<UserAuthPairData>, AuthError>;
    async fn try_register(
        &self,
        account: &Account,
        user_id: Uuid,
    ) -> Result<UserAuthPairData, AuthError>;
    async fn send_verify(
        &self,
        account: &Account,
        verify_info: &VerifyInfo,
    ) -> Result<(), AuthError>;
    async fn check_verify_response(
        &self,
        account: &Account,
        verify_code: &str,
    ) -> Result<bool, AuthError>;
}
