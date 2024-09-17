use sea_orm_migration::{prelude::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
enum AffGraph {
    #[sea_orm(iden = "ygg_affiliate__graph")]
    Table,
    Id,
    From,
    To,
    Reward,
    Rate,
    CreatedAt,
}

#[derive(DeriveIden)]
enum AffStat {
    #[sea_orm(iden = "ygg_affiliate__statistics")]
    Table,
    UserId,
    Total,
    Withdrawn,
    CountReferrals,
    Rate,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
            Table::create()
                .table(AffGraph::Table)
                .if_not_exists()
                .col(ColumnDef::new(AffGraph::Id).integer().not_null().auto_increment().primary_key())
                .col(ColumnDef::new(AffGraph::From).uuid().not_null())
                .col(ColumnDef::new(AffGraph::To).uuid().not_null())
                .col(ColumnDef::new(AffGraph::Reward).float().not_null())
                .col(ColumnDef::new(AffGraph::Rate).float().not_null())
                .col(ColumnDef::new(AffGraph::CreatedAt).timestamp().not_null().default("CURRENT_TIMESTAMP"))
                .to_owned()
        ).await?;
        manager
            .create_table(
            Table::create()
                .table(AffStat::Table)
                .if_not_exists()
                .col(ColumnDef::new(AffStat::UserId).uuid().not_null().primary_key())
                .col(ColumnDef::new(AffStat::Total).float().not_null())
                .col(ColumnDef::new(AffStat::Withdrawn).float().not_null())
                .col(ColumnDef::new(AffStat::CountReferrals).integer().not_null())
                .col(ColumnDef::new(AffStat::Rate).float().not_null())
                .to_owned()
        ).await?;
        manager.create_index(
            Index::create()
                .table(AffGraph::Table)
                .name("ygg_affiliate__from_index")
                .col(AffGraph::From)
                .to_owned()
        ).await?;
        manager.create_index(
            Index::create()
                .table(AffGraph::Table)
                .name("ygg_affiliate__to_index")
                .col(AffGraph::To)
                .to_owned()
        ).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_index(
            Index::drop()
                .name("ygg_affiliate__from_index")
                .table(AffGraph::Table)
                .to_owned()
        ).await?;
        manager.drop_index(
            Index::drop()
                .name("ygg_affiliate__to_index")
                .table(AffGraph::Table)
                .to_owned()
        ).await?;
        manager.drop_table(
            Table::drop()
                .table(AffGraph::Table)
                .to_owned()
        ).await?;
        manager.drop_table(
            Table::drop()
                .table(AffStat::Table)
                .to_owned()
        ).await?;
        Ok(())
    }
}