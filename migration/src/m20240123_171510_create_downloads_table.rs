use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Download::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Download::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Download::Url).string().not_null())
                    .col(ColumnDef::new(Download::Name).string().not_null())
                    .col(ColumnDef::new(Download::Host).string().not_null())
                    .col(ColumnDef::new(Download::Size).integer().not_null())
                    .col(ColumnDef::new(Download::Status).string().not_null())
                    .col(ColumnDef::new(Download::CreatedAt).date_time().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Download::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Download {
    Table,
    Id,
    Url,
    Name,
    Host,
    Size,
    Status,
    CreatedAt,
}
