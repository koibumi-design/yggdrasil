use sea_orm::entity::prelude::*;
use sea_orm::ActiveValue::Set;
use sea_orm::QuerySelect;
use std::default::Default;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "ygg_shop__production")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(indexed)]
    pub name: String,
    pub price: f32,
    pub stock: i32,
    #[sea_orm(default_value = 0)]
    pub locked_stock: i32,
    #[sea_orm(indexed)]
    pub production_type: String,
    #[sea_orm(default_value = false)]
    pub infinity_stock: bool,
    #[sea_orm(column_type = "Text")]
    pub description: String,
    pub content: String,
    #[sea_orm(indexed)]
    pub labels: Vec<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Debug, Clone, PartialEq)]
pub struct ProductionDataBeforeCreate {
    pub name: String,
    pub price: f32,
    pub stock: i32,
    pub production_type: String,
    pub description: String,
    pub content: String,
    pub labels: Vec<String>,
}

pub type ProductionEntity = Entity;
pub type ProductionData = Model;

impl ProductionData {
    pub async fn list_all_productions(
        db: &impl ConnectionTrait,
        limit: u64,
        offset: u64,
    ) -> Result<Vec<ProductionData>, DbErr> {
        Entity::find().cursor_by(Column::Id).offset(offset).limit(limit).all(db).await
    }

    pub async fn find_by_id(
        db: &impl ConnectionTrait,
        id: i32,
    ) -> Result<Option<ProductionData>, DbErr> {
        Entity::find().filter(Column::Id.eq(id)).one(db).await
    }

    pub async fn find_by_exact_name(
        db: &impl ConnectionTrait,
        name: &str,
        limit: u64,
        offset: u64,
    ) -> Result<Vec<ProductionData>, DbErr> {
        Entity::find().filter(Column::Name.eq(name)).offset(offset).limit(limit).all(db).await
    }

    pub async fn find_by_fused_name(
        db: &impl ConnectionTrait,
        name: &str,
        limit: u64,
        offset: u64,
    ) -> Result<Vec<ProductionData>, DbErr> {
        Entity::find().filter(Column::Name.like(format!("%{}%", name))).offset(offset).limit(limit).all(db).await
    }

    pub async fn update_full(
        db: &impl ConnectionTrait,
        before: &ProductionData,
        new_data: ProductionData,
    ) -> Result<ProductionData, DbErr> {
        let mut active: ActiveModel = before.clone().into();
        active.content = Set(new_data.content);
        active.description = Set(new_data.description);
        active.infinity_stock = Set(new_data.infinity_stock);
        active.labels = Set(new_data.labels);
        active.locked_stock = Set(new_data.locked_stock);
        active.name = Set(new_data.name);
        active.price = Set(new_data.price);
        active.production_type = Set(new_data.production_type);
        active.stock = Set(new_data.stock);
        active.update(db).await
    }

    pub async fn update_stock(
        db: &impl ConnectionTrait,
        before: &ProductionData,
        new_stock: i32,
    ) -> Result<ProductionData, DbErr> {
        let mut active: ActiveModel = before.clone().into();
        active.stock = Set(new_stock);
        active.update(db).await
    }

    pub async fn update_locked_stock(
        db: &impl ConnectionTrait,
        before: &ProductionData,
        new_locked_stock: i32,
    ) -> Result<ProductionData, DbErr> {
        let mut active: ActiveModel = before.clone().into();
        active.locked_stock = Set(new_locked_stock);
        active.update(db).await
    }

    pub async fn create(
        db: &impl ConnectionTrait,
        data: ProductionDataBeforeCreate,
    ) -> Result<ProductionData, DbErr> {
        ActiveModel {
            name: Set(data.name),
            price: Set(data.price),
            stock: Set(data.stock),
            production_type: Set(data.production_type),
            description: Set(data.description),
            content: Set(data.content),
            labels: Set(data.labels),
            ..Default::default()
        }.insert(db).await
    }

    pub async fn lock_stock(
        db: &impl ConnectionTrait,
        before: &ProductionData,
        change_amount: i32,
    ) -> Result<ProductionData, DbErr> {
        let current_stock = before.stock;
        let current_locked_stock = before.locked_stock;
        let mut active: ActiveModel = before.clone().into();
        active.stock = Set(current_stock - change_amount);
        active.locked_stock = Set(current_locked_stock + change_amount);
        active.update(db).await
    }

    pub async fn unlock_stock(
        db: &impl ConnectionTrait,
        before: &ProductionData,
        change_amount: i32,
    ) -> Result<ProductionData, DbErr> {
        let current_stock = before.stock;
        let current_locked_stock = before.locked_stock;
        let mut active: ActiveModel = before.clone().into();
        active.stock = Set(current_stock + change_amount);
        active.locked_stock = Set(current_locked_stock - change_amount);
        active.update(db).await
    }
}
