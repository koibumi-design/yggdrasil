use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
enum ScheduledEvent {
    #[sea_orm(iden = "ygg_schedule__scheduled_event")]
    Table,
    Id,
    Time,
    Payload,
    HaveBeenExecuted,
    Consumer,
    CreatedAt,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_table(
            Table::create()
                .table(ScheduledEvent::Table)
                .if_not_exists()
                .col(ColumnDef::new(ScheduledEvent::Id).integer().not_null().auto_increment().primary_key())
                .col(ColumnDef::new(ScheduledEvent::Time).timestamp().not_null())
                .col(ColumnDef::new(ScheduledEvent::Payload).text().not_null())
                .col(ColumnDef::new(ScheduledEvent::HaveBeenExecuted).boolean().not_null().default(false))
                .col(ColumnDef::new(ScheduledEvent::Consumer).string().not_null())
                .col(ColumnDef::new(ScheduledEvent::CreatedAt).timestamp().not_null())
                .to_owned()
        ).await?;
        manager.create_index(
            Index::create()
                .table(ScheduledEvent::Table)
                .name("ygg_schedule__scheduled_event_time_index")
                .col(ScheduledEvent::Time)
                .to_owned()
        ).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(ScheduledEvent::Table).to_owned()).await?;
        Ok(())
    }
}
