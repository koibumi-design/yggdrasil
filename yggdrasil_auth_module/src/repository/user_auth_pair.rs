use sea_orm::{
    ActiveModelBehavior, ActiveModelTrait, ActiveValue::Set, ColumnTrait, ConnectionTrait, DbErr,
    DeleteResult, DeriveEntityModel, DerivePrimaryKey, DeriveRelation, EntityTrait, EnumIter,
    PrimaryKeyTrait, QueryFilter,
};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "ygg_auth__user_auth_pair")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id_number: i32,

    #[sea_orm(index)]
    pub auth_provider: String,

    #[sea_orm(index)]
    pub auth_key: String,

    #[sea_orm(index)]
    pub user_id: Uuid,

    #[sea_orm(default_value = false)]
    pub is_verified: bool,
    pub verified_at: Option<chrono::NaiveDateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

pub type UserAuthPairData = Model;
pub type UserAuthPairEntity = Entity;

#[derive(Debug, Clone, PartialEq)]
pub struct UserAuthPairBeforeInsert {
    pub auth_provider: String,
    pub auth_key: String,
    pub user_id: Uuid,
}

impl UserAuthPairData {
    pub async fn create(
        db: &impl ConnectionTrait,
        data: UserAuthPairBeforeInsert,
    ) -> Result<UserAuthPairData, DbErr> {
        ActiveModel {
            auth_provider: Set(data.auth_provider),
            auth_key: Set(data.auth_key),
            user_id: Set(data.user_id),
            ..Default::default()
        }
        .insert(db)
        .await
    }

    pub async fn update_is_verified(
        db: &impl ConnectionTrait,
        before: &UserAuthPairData,
        is_verified: bool,
    ) -> Result<UserAuthPairData, DbErr> {
        let mut active: ActiveModel = before.clone().into();
        active.is_verified = Set(is_verified);
        if is_verified {
            active.verified_at = Set(Some(chrono::Utc::now().naive_utc()));
        } else {
            active.verified_at = Set(None);
        }
        active.update(db).await
    }

    pub async fn find_by_user_id(
        db: &impl ConnectionTrait,
        user_id: Uuid,
    ) -> Result<Vec<UserAuthPairData>, DbErr> {
        Entity::find()
            .filter(Column::UserId.eq(user_id))
            .all(db)
            .await
    }

    pub async fn find_by_key(
        db: &impl ConnectionTrait,
        auth_provider: &str,
        auth_key: &str,
    ) -> Result<Option<UserAuthPairData>, DbErr> {
        Entity::find()
            .filter(Column::AuthProvider.eq(auth_provider))
            .filter(Column::AuthKey.eq(auth_key))
            .one(db)
            .await
    }

    pub async fn delete(
        db: &impl ConnectionTrait,
        before: UserAuthPairData,
    ) -> Result<DeleteResult, DbErr> {
        let active: ActiveModel = before.into();
        Entity::delete(active).exec(db).await
    }

    pub async fn update_auth_key(
        db: &impl ConnectionTrait,
        before: UserAuthPairData,
        new_auth_key: &str,
    ) -> Result<UserAuthPairData, DbErr> {
        let mut active: ActiveModel = before.into();
        active.auth_key = Set(new_auth_key.to_string());
        active.update(db).await
    }
}
