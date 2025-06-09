use sea_orm::Statement;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(Iden)]
enum GameHeader {
    Table,
    HeaderId,
    GameId,
    HeaderName,
    HeaderValue,
}

#[derive(Iden)]
enum Game {
    Table,
    GameId,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create a new table for game headers
        manager
            .create_table(
                table_auto(GameHeader::Table)
                    .col(pk_auto(GameHeader::HeaderId))
                    .col(integer(GameHeader::GameId))
                    .col(string(GameHeader::HeaderName))
                    .col(string(GameHeader::HeaderValue))
                    // Auto-generated from `table_auto`
                    // .col(timestamp(GameHeader::CreatedAt).default(Expr::current_timestamp()))
                    // .col(timestamp(GameHeader::UpdatedAt).default(Expr::current_timestamp()))
                    .foreign_key(
                        ForeignKey::create()
                            .from(GameHeader::Table, GameHeader::GameId)
                            .to(Game::Table, Game::GameId)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // manager
        //     .get_connection()
        //     .execute(
        //         Statement::from_string(
        //             manager.get_database_backend(),
        //             r#"CREATE TRIGGER GameHeader_Update
        //             AFTER UPDATE ON GameHeader
        //             BEGIN
        //                 UPDATE GameHeader SET updated_at = CURRENT_TIMESTAMP WHERE header_id = OLD.header_id;
        //             END;"#.to_owned(),
        //         ),
        //     )
        //     .await?;

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
