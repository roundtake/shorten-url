use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ShortenUrl::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ShortenUrl::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key()
                    )
                    .col(
                        ColumnDef::new(ShortenUrl::ShortenId)
                            .string()
                            .not_null()
                            .unique_key()
                    )
                    .col(
                        ColumnDef::new(ShortenUrl::OriginalUrl)
                            .string()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ShortenUrl::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum ShortenUrl {
    Table,
    Id,
    ShortenId,
    OriginalUrl,
}
