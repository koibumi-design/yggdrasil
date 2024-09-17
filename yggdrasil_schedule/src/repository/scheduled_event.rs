use chrono::NaiveTime;
use sea_orm::{
    ActiveModelBehavior, ActiveModelTrait, ActiveValue::Set, ColumnTrait, ConnectionTrait, DbErr,
    DeleteResult, DeriveEntityModel, DerivePrimaryKey, DeriveRelation, EntityTrait, EnumIter,
    PrimaryKeyTrait, QueryFilter,
};

#[derive(Debug, Clone, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "ygg_schedule__scheduled_event")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u64,
    #[sea_orm(index)]
    pub time: NaiveTime,
    #[sea_orm(column_type = "Text")]
    pub payload: String,
    #[sea_orm(default=false)]
    pub have_been_executed: bool,
    pub consumer: String,
    pub created_at: NaiveTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

pub type ScheduledEventData = Model;
pub type ScheduledEventEntity = Entity;

pub struct ScheduledEventBeforeInsert {
    pub time: NaiveTime,
    pub payload: String,
    pub consumer: String,
}

impl ScheduledEventData {
    pub async fn create(
        db: &impl ConnectionTrait,
        data: ScheduledEventBeforeInsert,
    ) -> Result<ScheduledEventData, DbErr> {
        ActiveModel {
            time: Set(data.time),
            payload: Set(data.payload),
            consumer: Set(data.consumer),
            created_at: Set(chrono::Utc::now().time()),
            ..Default::default()
        }
        .insert(db)
        .await
    }

    pub async fn set_have_been_executed(
        db: &impl ConnectionTrait,
        before: &ScheduledEventData,
        have_been_executed: bool,
    ) -> Result<ScheduledEventData, DbErr> {
        let mut active: ActiveModel = before.clone().into();
        active.have_been_executed = Set(have_been_executed);
        active.update(db).await
    }

    pub async fn delete_by_id(
        db: &impl ConnectionTrait,
        id: u64,
    ) -> Result<DeleteResult, DbErr> {
        ScheduledEventEntity::delete_by_id(id).exec(db).await
    }

    pub async fn find_by_id(
        db: &impl ConnectionTrait,
        id: u64,
    ) -> Result<Option<ScheduledEventData>, DbErr> {
        ScheduledEventEntity::find_by_id(id).one(db).await
    }

    pub async fn get_all_on_time(
        db: &impl ConnectionTrait,
        time: NaiveTime,
    ) -> Result<Vec<ScheduledEventData>, DbErr> {
        ScheduledEventEntity::find()
            .filter(Column::Time.lte(time))
            .filter(Column::HaveBeenExecuted.eq(false))
            .all(db)
            .await
    }
}