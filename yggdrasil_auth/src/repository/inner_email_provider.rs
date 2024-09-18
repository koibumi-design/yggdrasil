use sea_orm::{
    ActiveModelBehavior, ActiveModelTrait, ActiveValue::Set, ColumnTrait, ConnectionTrait, DbErr,
    DeleteResult, DeriveEntityModel, DerivePrimaryKey, DeriveRelation, EntityTrait, EnumIter,
    PrimaryKeyTrait, QueryFilter,
};
use std::option::Option;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "ygg_auth__email_provider")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub email: String,

    #[sea_orm(column_type="Text")]
    pub password_hash: String,

    #[sea_orm(index, unique)]
    pub auth_key: Uuid,

    pub verify_code: Option<String>,
    pub code_sent_at: Option<chrono::NaiveDateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

pub type InnerEmailProviderData = Model;
pub type InnerEmailProviderEntity = Entity;

#[derive(Debug, Clone, PartialEq)]
pub struct InnerEmailProviderBeforeInsert {
    pub email: String,
    pub password_hash: String,
    pub auth_key: Uuid,
}

impl InnerEmailProviderData {
    pub async fn create(
        db: &impl ConnectionTrait,
        data: InnerEmailProviderBeforeInsert,
    ) -> Result<InnerEmailProviderData, DbErr> {
        ActiveModel {
            email: Set(data.email),
            password_hash: Set(data.password_hash),
            auth_key: Set(data.auth_key),
            ..Default::default()
        }
        .insert(db)
        .await
    }

    pub async fn set_verify_code(
        db: &impl ConnectionTrait,
        before: &InnerEmailProviderData,
        verify_code: String,
    ) -> Result<InnerEmailProviderData, DbErr> {
        let mut active: ActiveModel = before.clone().into();
        active.verify_code = Set(Some(verify_code));
        active.code_sent_at = Set(Some(chrono::Utc::now().naive_utc()));
        active.update(db).await
    }

    pub async fn find_by_auth_key(
        db: &impl ConnectionTrait,
        auth_key: Uuid,
    ) -> Result<Option<InnerEmailProviderData>, DbErr> {
        Entity::find()
            .filter(Column::AuthKey.eq(auth_key))
            .one(db)
            .await
    }

    pub async fn find_by_email(
        db: &impl ConnectionTrait,
        email: &str,
    ) -> Result<Option<InnerEmailProviderData>, DbErr> {
        Entity::find().filter(Column::Email.eq(email)).one(db).await
    }

    pub async fn update_password_hash(
        db: &impl ConnectionTrait,
        before: &InnerEmailProviderData,
        new_password_hash: &str,
    ) -> Result<InnerEmailProviderData, DbErr> {
        let mut active: ActiveModel = before.clone().into();
        active.password_hash = Set(new_password_hash.to_owned());
        active.update(db).await
    }

    pub async fn delete(
        db: &impl ConnectionTrait,
        before: InnerEmailProviderData,
    ) -> Result<DeleteResult, DbErr> {
        let active: ActiveModel = before.into();
        Entity::delete(active).exec(db).await
    }

    pub async fn update_email(
        db: &impl ConnectionTrait,
        before: &InnerEmailProviderData,
        new_email: &str,
    ) -> Result<InnerEmailProviderData, DbErr> {
        let mut active: ActiveModel = before.clone().into();
        active.email = Set(new_email.to_owned());
        active.update(db).await
    }
}
