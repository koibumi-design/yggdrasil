use sea_orm::{
    ActiveModelBehavior, ActiveModelTrait, ActiveValue::Set, ConnectionTrait, DbErr,
    DeleteResult, DeriveEntityModel, DerivePrimaryKey, DeriveRelation, EntityTrait, EnumIter, PrimaryKeyTrait,
};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "ygg_affiliate__statistics")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub user_id: Uuid,
    #[sea_orm(default_value = 0.0)]
    pub total: f32,
    #[sea_orm(default_value = 0.0)]
    pub withdrawn: f32,
    #[sea_orm(default_value = 0)]
    pub count_referrals: i32,
    pub rate: f32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

pub type AffiliateStatisticsData = Model;
pub type AffiliateStatisticsEntity = Entity;

#[derive(Debug, Clone, PartialEq)]
pub struct AffiliateStatisticsDataBeforeCreate {
    user_id: Uuid,
    rate: f32,
}

impl AffiliateStatisticsData {
    pub async fn create(
        db: &impl ConnectionTrait,
        data: AffiliateStatisticsDataBeforeCreate,
    ) -> Result<AffiliateStatisticsData, DbErr> {
        ActiveModel {
            user_id: Set(data.user_id),
            rate: Set(data.rate),
            ..Default::default()
        }.insert(db).await
    }

    pub async fn update_total(
        db: &impl ConnectionTrait,
        before: &AffiliateStatisticsData,
        new_total: f32,
    ) -> Result<AffiliateStatisticsData, DbErr> {
        let mut active: ActiveModel = before.clone().into();
        active.total = Set(new_total);
        active.update(db).await
    }

    pub async fn update_withdrawn(
        db: &impl ConnectionTrait,
        before: &AffiliateStatisticsData,
        new_withdrawn: f32,
    ) -> Result<AffiliateStatisticsData, DbErr> {
        let mut active: ActiveModel = before.clone().into();
        active.withdrawn = Set(new_withdrawn);
        active.update(db).await
    }

    pub async fn update_count_referrals(
        db: &impl ConnectionTrait,
        before: &AffiliateStatisticsData,
        new_count_referrals: i32,
    ) -> Result<AffiliateStatisticsData, DbErr> {
        let mut active: ActiveModel = before.clone().into();
        active.count_referrals = Set(new_count_referrals);
        active.update(db).await
    }

    pub async fn update_rate(
        db: &impl ConnectionTrait,
        before: &AffiliateStatisticsData,
        new_rate: f32,
    ) -> Result<AffiliateStatisticsData, DbErr> {
        let mut active: ActiveModel = before.clone().into();
        active.rate = Set(new_rate);
        active.update(db).await
    }

    pub async fn on_invite(
        db: &impl ConnectionTrait,
        before: &AffiliateStatisticsData,
        raw_amount: f32,
    ) -> Result<AffiliateStatisticsData, DbErr> {
        let rate = before.rate;
        let before_total = before.total;
        let before_count = before.count_referrals;
        let mut active: ActiveModel = before.clone().into();
        active.total = Set(before_total + rate * raw_amount);
        active.count_referrals = Set(before_count + 1);
        active.update(db).await
    }

    pub async fn find_by_id(
        db: &impl ConnectionTrait,
        id: Uuid,
    ) -> Result<Option<AffiliateStatisticsData>, DbErr> {
        Entity::find_by_id(id).one(db).await
    }

    pub async fn delete_by_id(
        db: &impl ConnectionTrait,
        id: Uuid,
    ) -> Result<DeleteResult, DbErr> {
        Entity::delete_by_id(id).exec(db).await
    }
}