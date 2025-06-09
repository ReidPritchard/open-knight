use sea_orm::Statement;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create a new table for game headers
        manager
            .create_table(
                Table::create()
                    .table(GameHeader::Table)
                    .col(
                        ColumnDef::new(GameHeader::HeaderId)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(GameHeader::GameId).integer().not_null())
                    .col(ColumnDef::new(GameHeader::HeaderName).string().not_null())
                    .col(ColumnDef::new(GameHeader::HeaderValue).string().not_null())
                    .col(
                        ColumnDef::new(GameHeader::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(GameHeader::UpdatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(GameHeader::Table, GameHeader::GameId)
                            .to(Game::Table, Game::GameId)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .if_not_exists()
                    .to_owned(),
            )
            .await?;

        manager
            .get_connection()
            .execute(
                Statement::from_string(
                    manager.get_database_backend(),
                    r#"CREATE TRIGGER GameHeader_Update
                    AFTER UPDATE ON GameHeader
                    BEGIN
                        UPDATE GameHeader SET updated_at = CURRENT_TIMESTAMP WHERE header_id = OLD.header_id;
                    END;"#.to_owned(),
                ),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop the game headers table
        manager
            .drop_table(Table::drop().table(GameHeader::Table).to_owned())
            .await?;

        Ok(())
    }
}

// Recreate partial Iden enums from the original migration
// not sure if there is a better way to do this
// or even if this is "best practice"
// I'd think we should be importing the original enums to make sure they're up to date

#[derive(Iden)]
enum GameHeader {
    Table,
    HeaderId,
    GameId,
    HeaderName,
    HeaderValue,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum Game {
    Table,
    GameId,
}
