use sea_orm::{
    prelude::Expr, ActiveModelBehavior, ActiveModelTrait, ActiveValue::Set, ColumnTrait, ConnectionTrait,
    DbErr, DeleteResult, DeriveEntityModel, DerivePrimaryKey, DeriveRelation, EntityTrait,
    EnumIter, PrimaryKeyTrait, QueryFilter, QuerySelect,
};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "ygg_affiliate__graph")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(indexed)]
    pub from: Uuid,
    #[sea_orm(indexed)]
    pub to: Uuid,
    pub reward: f32,
    pub rate: f32,
    #[sea_orm(default_expr = "Expr::current_timestamp()")]
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

pub type AffiliateGraphData = Model;
pub type AffiliateGraphEntity = Entity;

#[derive(Debug, Clone, PartialEq)]
pub struct AffiliateGraphDataBeforeCreate {
    pub from: Uuid,
    pub to: Uuid,
    pub reward: f32,
    pub rate: f32,
}

impl AffiliateGraphData {
    pub async fn create(
        db: &impl ConnectionTrait,
        data: &AffiliateGraphDataBeforeCreate,
    ) -> Result<Self, DbErr> {
        ActiveModel {
            from: Set(data.from),
            to: Set(data.to),
            rate: Set(data.rate),
            reward: Set(data.reward),
            ..Default::default()
        }.insert(db).await
    }

    pub async fn find_by_id(
        db: &impl ConnectionTrait,
        id: i32,
    ) -> Result<Option<Self>, DbErr> {
        Entity::find().filter(Column::Id.eq(id)).one(db).await
    }

    pub async fn find_by_from(
        db: &impl ConnectionTrait,
        from: Uuid,
        limit: u64,
        offset: u64,
    ) -> Result<Vec<Self>, DbErr> {
        Entity::find()
            .filter(Column::From.eq(from))
            .offset(offset)
            .limit(limit)
            .all(db).await
    }

    pub async fn delete_by_id(
        db: &impl ConnectionTrait,
        id: i32,
    ) -> Result<DeleteResult, DbErr> {
        Entity::delete_by_id(id).exec(db).await
    }

    pub async fn delete_by_from(
        db: &impl ConnectionTrait,
        from: Uuid,
    ) -> Result<DeleteResult, DbErr> {
        Entity::delete_many()
            .filter(Column::From.eq(from))
            .exec(db).await
    }

    pub async fn delete_by_to(
        db: &impl ConnectionTrait,
        to: Uuid,
    ) -> Result<DeleteResult, DbErr> {
        Entity::delete_many()
            .filter(Column::To.eq(to))
            .exec(db).await
    }
}