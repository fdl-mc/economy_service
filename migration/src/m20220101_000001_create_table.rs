use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                sea_query::Table::create()
                    .table(EconomyStates::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(EconomyStates::Id)
                            .integer()
                            .primary_key()
                            .not_null()
                            .auto_increment(),
                    )
                    .col(
                        ColumnDef::new(EconomyStates::UserId)
                            .integer()
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(EconomyStates::Balance)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(EconomyStates::Banker)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                sea_query::Table::drop()
                    .table(EconomyStates::Table)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

#[derive(Iden)]
enum EconomyStates {
    Table,
    Id,
    UserId,
    Balance,
    Banker,
}
