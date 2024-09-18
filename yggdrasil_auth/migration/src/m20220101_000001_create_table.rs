use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
enum UserAuthPair {
    #[sea_orm(iden = "ygg_auth__user_auth_pair")]
    Table,
    IdNumber,
    AuthProvider,
    AuthKey,
    UserId,
    IsVerified,
    VerifiedAt,
}

#[derive(DeriveIden)]
enum EmailProvider {
    #[sea_orm(iden = "ygg_auth__email_provider")]
    Table,
    Email,
    PasswordHash,
    AuthKey,
    VerifyCode,
    CodeSentAt,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_table(
            Table::create()
                .table(UserAuthPair::Table)
                .if_not_exists()
                .col(ColumnDef::new(UserAuthPair::IdNumber).integer().not_null().auto_increment().primary_key())
                .col(ColumnDef::new(UserAuthPair::AuthProvider).string().not_null())
                .col(ColumnDef::new(UserAuthPair::AuthKey).string().not_null())
                .col(ColumnDef::new(UserAuthPair::UserId).uuid().not_null())
                .col(ColumnDef::new(UserAuthPair::IsVerified).boolean().not_null().default(false))
                .col(ColumnDef::new(UserAuthPair::VerifiedAt).timestamp().nullable())
                .to_owned()
        ).await?;
        manager.create_index(
            Index::create()
                .table(UserAuthPair::Table)
                .name("ygg_auth__pair_provider_index")
                .col(UserAuthPair::AuthProvider)
                .col(UserAuthPair::AuthKey)
                .to_owned()
        ).await?;
        manager.create_index(
            Index::create()
                .table(UserAuthPair::Table)
                .name("ygg_auth__pair_user_id_index")
                .col(UserAuthPair::UserId)
                .to_owned()
        ).await?;
        manager.create_table(
            Table::create()
                .table(EmailProvider::Table)
                .if_not_exists()
                .col(ColumnDef::new(EmailProvider::Email).string().not_null().primary_key())
                .col(ColumnDef::new(EmailProvider::PasswordHash).text().not_null())
                .col(ColumnDef::new(EmailProvider::AuthKey).uuid().not_null().unique())
                .col(ColumnDef::new(EmailProvider::VerifyCode).string().nullable())
                .col(ColumnDef::new(EmailProvider::CodeSentAt).timestamp().nullable())
                .to_owned()
        ).await?;
        manager.create_index(
            Index::create()
                .table(EmailProvider::Table)
                .name("ygg_auth__email_provider_auth_key_index")
                .col(EmailProvider::AuthKey)
                .unique()
                .to_owned()
        ).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_index(
            Index::drop()
                .name("ygg_auth__email_provider_auth_key_index")
                .table(EmailProvider::AuthKey)
                .to_owned()
        ).await?;
        manager.drop_table(Table::drop().table(EmailProvider::Table).to_owned()).await?;
        manager.drop_index(
            Index::drop()
                .name("ygg_auth__pair_user_id_index")
                .table(UserAuthPair::Table)
                .to_owned()
        ).await?;
        manager.drop_index(
            Index::drop()
                .name("ygg_auth__pair_provider_index")
                .table(UserAuthPair::Table)
                .to_owned()
        ).await?;
        manager.drop_table(Table::drop().table(UserAuthPair::Table).to_owned()).await?;
        Ok(())
    }
}
