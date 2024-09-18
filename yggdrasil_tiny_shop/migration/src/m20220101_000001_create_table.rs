use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
enum Production {
    #[sea_orm(iden = "ygg_tiny_shop__production")]
    Table,
    Id,
    Name,
    Price,
    Stock,
    LockedStock,
    ProductionType,
    InfinityStock,
    Description,
    Content,
    Labels,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_table(
            Table::create()
                .table(Production::Table)
                .if_not_exists()
                .col(ColumnDef::new(Production::Id).integer().not_null().auto_increment().primary_key())
                .col(ColumnDef::new(Production::Name).string().not_null())
                .col(ColumnDef::new(Production::Price).float().not_null())
                .col(ColumnDef::new(Production::Stock).integer().not_null())
                .col(ColumnDef::new(Production::LockedStock).integer().not_null().default(0))
                .col(ColumnDef::new(Production::ProductionType).string().not_null())
                .col(ColumnDef::new(Production::InfinityStock).boolean().not_null().default(false))
                .col(ColumnDef::new(Production::Description).text().not_null())
                .col(ColumnDef::new(Production::Content).text().not_null())
                .col(ColumnDef::new(Production::Labels).array(ColumnType::Text).not_null())
                .to_owned()
        ).await?;
        manager.create_index(
            Index::create()
                .table(Production::Table)
                .name("ygg_tiny_shop__production_name_index")
                .col(Production::Name)
                .to_owned()
        ).await?;
        manager.create_index(
            Index::create()
                .table(Production::Table)
                .name("ygg_tiny_shop__production_production_type_index")
                .col(Production::ProductionType)
                .to_owned()
        ).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Production::Table).to_owned()).await?;
        Ok(())
    }
}