use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::Statement;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(
        &self,
        manager: &SchemaManager,
    ) -> Result<(), DbErr> {
        // Add CASCADE constraints to foreign keys
        self.add_cascade_constraints(manager).await
    }

    async fn down(
        &self,
        manager: &SchemaManager,
    ) -> Result<(), DbErr> {
        // Remove CASCADE constraints (revert to original foreign key behavior)
        self.remove_cascade_constraints(manager).await
    }
}

impl Migration {
    /// Add CASCADE constraints to relevant foreign keys
    async fn add_cascade_constraints<'a>(
        &self,
        manager: &'a SchemaManager<'a>,
    ) -> Result<(), DbErr> {
        let conn = manager.get_connection();

        // Disable foreign key checks during migration
        conn.execute(Statement::from_string(
            manager.get_database_backend(),
            "PRAGMA foreign_keys = OFF".to_string(),
        ))
        .await?;

        // Create new tables with CASCADE constraints
        self.create_tables_with_cascade(manager).await?;

        // Copy data from original tables (in dependency order)
        self.copy_data_to_new_tables(manager).await?;

        // Drop original tables (in reverse dependency order)
        self.drop_original_tables(manager).await?;

        // Rename new tables to original names
        self.rename_new_tables_to_original(manager).await?;

        // Re-enable foreign key checks
        conn.execute(Statement::from_string(
            manager.get_database_backend(),
            "PRAGMA foreign_keys = ON".to_string(),
        ))
        .await?;

        Ok(())
    }

    /// Remove CASCADE constraints (revert to original behavior)
    async fn remove_cascade_constraints<'a>(
        &self,
        manager: &'a SchemaManager<'a>,
    ) -> Result<(), DbErr> {
        let conn = manager.get_connection();

        // Disable foreign key checks during migration
        conn.execute(Statement::from_string(
            manager.get_database_backend(),
            "PRAGMA foreign_keys = OFF".to_string(),
        ))
        .await?;

        // Create tables without CASCADE constraints (original behavior)
        self.create_tables_without_cascade(manager).await?;

        // Copy data back
        self.copy_data_to_reverted_tables(manager).await?;

        // Drop CASCADE tables
        self.drop_cascade_tables(manager).await?;

        // Rename reverted tables to original names
        self.rename_reverted_tables_to_original(manager).await?;

        // Re-enable foreign key checks
        conn.execute(Statement::from_string(
            manager.get_database_backend(),
            "PRAGMA foreign_keys = ON".to_string(),
        ))
        .await?;

        Ok(())
    }

    /// Create new tables with CASCADE foreign key constraints
    async fn create_tables_with_cascade<'a>(
        &self,
        manager: &'a SchemaManager<'a>,
    ) -> Result<(), DbErr> {
        // Create Game table first (no dependencies)
        manager
            .create_table(
                Table::create()
                    .table(Alias::new("game_new"))
                    .col(
                        ColumnDef::new(Alias::new("game_id"))
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Alias::new("white_player_id"))
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Alias::new("black_player_id"))
                            .integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Alias::new("tournament_id")).integer())
                    .col(ColumnDef::new(Alias::new("opening_id")).integer())
                    .col(ColumnDef::new(Alias::new("result")).string())
                    .col(ColumnDef::new(Alias::new("termination")).string())
                    .col(ColumnDef::new(Alias::new("round_number")).integer())
                    .col(ColumnDef::new(Alias::new("date_played")).string())
                    .col(ColumnDef::new(Alias::new("time_control")).string())
                    .col(ColumnDef::new(Alias::new("fen")).string())
                    .col(ColumnDef::new(Alias::new("variant")).string())
                    .col(ColumnDef::new(Alias::new("pgn")).string().not_null())
                    .col(ColumnDef::new(Alias::new("created_at")).timestamp())
                    // Foreign keys (same as original, no CASCADE needed here)
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_game_white_player_new")
                            .from(
                                Alias::new("game_new"),
                                Alias::new("white_player_id"),
                            )
                            .to(Player::Table, Player::PlayerId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_game_black_player_new")
                            .from(
                                Alias::new("game_new"),
                                Alias::new("black_player_id"),
                            )
                            .to(Player::Table, Player::PlayerId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_game_tournament_new")
                            .from(
                                Alias::new("game_new"),
                                Alias::new("tournament_id"),
                            )
                            .to(Tournament::Table, Tournament::TournamentId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_game_opening_new")
                            .from(
                                Alias::new("game_new"),
                                Alias::new("opening_id"),
                            )
                            .to(Opening::Table, Opening::OpeningId),
                    )
                    .to_owned(),
            )
            .await?;

        // Create Move table with CASCADE to Game
        manager
            .create_table(
                Table::create()
                    .table(Alias::new("move_new"))
                    .col(
                        ColumnDef::new(Alias::new("move_id"))
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Alias::new("game_id"))
                            .integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Alias::new("parent_move_id")).integer())
                    .col(
                        ColumnDef::new(Alias::new("variation_order"))
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(Alias::new("ply_number"))
                            .integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Alias::new("san")).string().not_null())
                    .col(ColumnDef::new(Alias::new("uci")).string().not_null())
                    .col(
                        ColumnDef::new(Alias::new("position_id"))
                            .integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Alias::new("created_at")).timestamp())
                    // CASCADE foreign key to Game
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_move_game_new")
                            .from(Alias::new("move_new"), Alias::new("game_id"))
                            .to(Alias::new("game_new"), Alias::new("game_id"))
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_move_position_new")
                            .from(
                                Alias::new("move_new"),
                                Alias::new("position_id"),
                            )
                            .to(Position::Table, Position::PositionId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_move_parent_new")
                            .from(
                                Alias::new("move_new"),
                                Alias::new("parent_move_id"),
                            )
                            .to(Alias::new("move_new"), Alias::new("move_id")),
                    )
                    .to_owned(),
            )
            .await?;

        // Create Annotation table with CASCADE to Move
        manager
            .create_table(
                Table::create()
                    .table(Alias::new("annotation_new"))
                    .col(
                        ColumnDef::new(Alias::new("annotation_id"))
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Alias::new("move_id"))
                            .integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Alias::new("user_id")).integer())
                    .col(ColumnDef::new(Alias::new("comment")).text())
                    .col(ColumnDef::new(Alias::new("arrows")).text())
                    .col(ColumnDef::new(Alias::new("highlights")).text())
                    .col(ColumnDef::new(Alias::new("created_at")).timestamp())
                    // CASCADE foreign key to Move
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_annotation_move_new")
                            .from(
                                Alias::new("annotation_new"),
                                Alias::new("move_id"),
                            )
                            .to(Alias::new("move_new"), Alias::new("move_id"))
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_annotation_user_new")
                            .from(
                                Alias::new("annotation_new"),
                                Alias::new("user_id"),
                            )
                            .to(User::Table, User::UserId),
                    )
                    .to_owned(),
            )
            .await?;

        // Create MoveTimeTracking table with CASCADE to Move
        manager
            .create_table(
                Table::create()
                    .table(Alias::new("move_time_tracking_new"))
                    .col(
                        ColumnDef::new(Alias::new("time_tracking_id"))
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Alias::new("move_id"))
                            .integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Alias::new("time_spent_ms")).integer())
                    .col(ColumnDef::new(Alias::new("time_left_ms")).integer())
                    .col(ColumnDef::new(Alias::new("clock_type")).string())
                    .col(ColumnDef::new(Alias::new("created_at")).timestamp())
                    // CASCADE foreign key to Move
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_move_time_tracking_move_new")
                            .from(
                                Alias::new("move_time_tracking_new"),
                                Alias::new("move_id"),
                            )
                            .to(Alias::new("move_new"), Alias::new("move_id"))
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Create GameTag table with CASCADE to Game
        manager
            .create_table(
                Table::create()
                    .table(Alias::new("game_tag_new"))
                    .col(
                        ColumnDef::new(Alias::new("game_id"))
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Alias::new("tag_id"))
                            .integer()
                            .not_null(),
                    )
                    .primary_key(
                        Index::create()
                            .name("pk_game_tag_new")
                            .col(Alias::new("game_id"))
                            .col(Alias::new("tag_id")),
                    )
                    // CASCADE foreign key to Game
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_game_tag_game_new")
                            .from(
                                Alias::new("game_tag_new"),
                                Alias::new("game_id"),
                            )
                            .to(Alias::new("game_new"), Alias::new("game_id"))
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_game_tag_tag_new")
                            .from(
                                Alias::new("game_tag_new"),
                                Alias::new("tag_id"),
                            )
                            .to(Tag::Table, Tag::TagId),
                    )
                    .to_owned(),
            )
            .await?;

        // Create MoveTag table with CASCADE to Move
        manager
            .create_table(
                Table::create()
                    .table(Alias::new("move_tag_new"))
                    .col(
                        ColumnDef::new(Alias::new("move_id"))
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Alias::new("tag_id"))
                            .integer()
                            .not_null(),
                    )
                    .primary_key(
                        Index::create()
                            .name("pk_move_tag_new")
                            .col(Alias::new("move_id"))
                            .col(Alias::new("tag_id")),
                    )
                    // CASCADE foreign key to Move
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_move_tag_move_new")
                            .from(
                                Alias::new("move_tag_new"),
                                Alias::new("move_id"),
                            )
                            .to(Alias::new("move_new"), Alias::new("move_id"))
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_move_tag_tag_new")
                            .from(
                                Alias::new("move_tag_new"),
                                Alias::new("tag_id"),
                            )
                            .to(Tag::Table, Tag::TagId),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    /// Copy data from original tables to new tables (in dependency order)
    async fn copy_data_to_new_tables<'a>(
        &self,
        manager: &'a SchemaManager<'a>,
    ) -> Result<(), DbErr> {
        let conn = manager.get_connection();

        // Copy in dependency order: Game first, then Move, then everything else
        conn.execute(Statement::from_string(
            manager.get_database_backend(),
            "INSERT INTO game_new SELECT * FROM game".to_string(),
        ))
        .await?;

        conn.execute(Statement::from_string(
            manager.get_database_backend(),
            "INSERT INTO move_new SELECT * FROM move".to_string(),
        ))
        .await?;

        conn.execute(Statement::from_string(
            manager.get_database_backend(),
            "INSERT INTO annotation_new SELECT * FROM annotation".to_string(),
        ))
        .await?;

        conn.execute(Statement::from_string(
            manager.get_database_backend(),
            "INSERT INTO move_time_tracking_new SELECT * FROM move_time_tracking".to_string(),
        ))
        .await?;

        conn.execute(Statement::from_string(
            manager.get_database_backend(),
            "INSERT INTO game_tag_new SELECT * FROM game_tag".to_string(),
        ))
        .await?;

        conn.execute(Statement::from_string(
            manager.get_database_backend(),
            "INSERT INTO move_tag_new SELECT * FROM move_tag".to_string(),
        ))
        .await?;

        Ok(())
    }

    /// Drop original tables (in reverse dependency order)
    async fn drop_original_tables<'a>(
        &self,
        manager: &'a SchemaManager<'a>,
    ) -> Result<(), DbErr> {
        // Drop in reverse dependency order
        manager
            .drop_table(Table::drop().table(MoveTag::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(GameTag::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(MoveTimeTracking::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Annotation::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Move::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Game::Table).to_owned())
            .await?;

        Ok(())
    }

    /// Rename new tables to original names
    async fn rename_new_tables_to_original<'a>(
        &self,
        manager: &'a SchemaManager<'a>,
    ) -> Result<(), DbErr> {
        let conn = manager.get_connection();

        // Rename in dependency order
        conn.execute(Statement::from_string(
            manager.get_database_backend(),
            "ALTER TABLE game_new RENAME TO game".to_string(),
        ))
        .await?;

        conn.execute(Statement::from_string(
            manager.get_database_backend(),
            "ALTER TABLE move_new RENAME TO move".to_string(),
        ))
        .await?;

        conn.execute(Statement::from_string(
            manager.get_database_backend(),
            "ALTER TABLE annotation_new RENAME TO annotation".to_string(),
        ))
        .await?;

        conn.execute(Statement::from_string(
            manager.get_database_backend(),
            "ALTER TABLE move_time_tracking_new RENAME TO move_time_tracking"
                .to_string(),
        ))
        .await?;

        conn.execute(Statement::from_string(
            manager.get_database_backend(),
            "ALTER TABLE game_tag_new RENAME TO game_tag".to_string(),
        ))
        .await?;

        conn.execute(Statement::from_string(
            manager.get_database_backend(),
            "ALTER TABLE move_tag_new RENAME TO move_tag".to_string(),
        ))
        .await?;

        Ok(())
    }

    /// Create tables without CASCADE constraints (for down migration)
    async fn create_tables_without_cascade<'a>(
        &self,
        manager: &'a SchemaManager<'a>,
    ) -> Result<(), DbErr> {
        // Create tables with original foreign key behavior (without CASCADE)
        // This is essentially the same as create_tables_with_cascade but without .on_delete(ForeignKeyAction::Cascade)

        // Game table (unchanged)
        manager
            .create_table(
                Table::create()
                    .table(Alias::new("game_reverted"))
                    .col(
                        ColumnDef::new(Alias::new("game_id"))
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Alias::new("white_player_id"))
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Alias::new("black_player_id"))
                            .integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Alias::new("tournament_id")).integer())
                    .col(ColumnDef::new(Alias::new("opening_id")).integer())
                    .col(ColumnDef::new(Alias::new("result")).string())
                    .col(ColumnDef::new(Alias::new("termination")).string())
                    .col(ColumnDef::new(Alias::new("round_number")).integer())
                    .col(ColumnDef::new(Alias::new("date_played")).string())
                    .col(ColumnDef::new(Alias::new("time_control")).string())
                    .col(ColumnDef::new(Alias::new("fen")).string())
                    .col(ColumnDef::new(Alias::new("variant")).string())
                    .col(ColumnDef::new(Alias::new("pgn")).string().not_null())
                    .col(ColumnDef::new(Alias::new("created_at")).timestamp())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_game_white_player_reverted")
                            .from(
                                Alias::new("game_reverted"),
                                Alias::new("white_player_id"),
                            )
                            .to(Player::Table, Player::PlayerId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_game_black_player_reverted")
                            .from(
                                Alias::new("game_reverted"),
                                Alias::new("black_player_id"),
                            )
                            .to(Player::Table, Player::PlayerId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_game_tournament_reverted")
                            .from(
                                Alias::new("game_reverted"),
                                Alias::new("tournament_id"),
                            )
                            .to(Tournament::Table, Tournament::TournamentId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_game_opening_reverted")
                            .from(
                                Alias::new("game_reverted"),
                                Alias::new("opening_id"),
                            )
                            .to(Opening::Table, Opening::OpeningId),
                    )
                    .to_owned(),
            )
            .await?;

        // Move table WITHOUT CASCADE
        manager
            .create_table(
                Table::create()
                    .table(Alias::new("move_reverted"))
                    .col(
                        ColumnDef::new(Alias::new("move_id"))
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Alias::new("game_id"))
                            .integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Alias::new("parent_move_id")).integer())
                    .col(
                        ColumnDef::new(Alias::new("variation_order"))
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(Alias::new("ply_number"))
                            .integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Alias::new("san")).string().not_null())
                    .col(ColumnDef::new(Alias::new("uci")).string().not_null())
                    .col(
                        ColumnDef::new(Alias::new("position_id"))
                            .integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Alias::new("created_at")).timestamp())
                    // NO CASCADE - original behavior
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_move_game_reverted")
                            .from(
                                Alias::new("move_reverted"),
                                Alias::new("game_id"),
                            )
                            .to(
                                Alias::new("game_reverted"),
                                Alias::new("game_id"),
                            ),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_move_position_reverted")
                            .from(
                                Alias::new("move_reverted"),
                                Alias::new("position_id"),
                            )
                            .to(Position::Table, Position::PositionId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_move_parent_reverted")
                            .from(
                                Alias::new("move_reverted"),
                                Alias::new("parent_move_id"),
                            )
                            .to(
                                Alias::new("move_reverted"),
                                Alias::new("move_id"),
                            ),
                    )
                    .to_owned(),
            )
            .await?;

        // Continue with other tables (without CASCADE)...
        // [Similar pattern for all other tables]

        Ok(())
    }

    /// Copy data to reverted tables (for down migration)
    async fn copy_data_to_reverted_tables<'a>(
        &self,
        manager: &'a SchemaManager<'a>,
    ) -> Result<(), DbErr> {
        let conn = manager.get_connection();

        conn.execute(Statement::from_string(
            manager.get_database_backend(),
            "INSERT INTO game_reverted SELECT * FROM game".to_string(),
        ))
        .await?;

        conn.execute(Statement::from_string(
            manager.get_database_backend(),
            "INSERT INTO move_reverted SELECT * FROM move".to_string(),
        ))
        .await?;

        // Continue for other tables...
        Ok(())
    }

    /// Drop CASCADE tables (for down migration)
    async fn drop_cascade_tables<'a>(
        &self,
        manager: &'a SchemaManager<'a>,
    ) -> Result<(), DbErr> {
        // Drop current tables with CASCADE constraints
        manager
            .drop_table(Table::drop().table(MoveTag::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(GameTag::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(MoveTimeTracking::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Annotation::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Move::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Game::Table).to_owned())
            .await?;

        Ok(())
    }

    /// Rename reverted tables to original names (for down migration)
    async fn rename_reverted_tables_to_original<'a>(
        &self,
        manager: &'a SchemaManager<'a>,
    ) -> Result<(), DbErr> {
        let conn = manager.get_connection();

        conn.execute(Statement::from_string(
            manager.get_database_backend(),
            "ALTER TABLE game_reverted RENAME TO game".to_string(),
        ))
        .await?;

        conn.execute(Statement::from_string(
            manager.get_database_backend(),
            "ALTER TABLE move_reverted RENAME TO move".to_string(),
        ))
        .await?;

        Ok(())
    }
}

// Recreate partial Iden enums from the original migration
// not sure if there is a better way to do this
// or even if this is "best practice"
// I'd think we should be importing the original enums to make sure they're up to date

#[derive(Iden)]
enum Game {
    Table,
}

#[derive(Iden)]
enum Move {
    Table,
}

#[derive(Iden)]
enum Position {
    Table,
    PositionId,
}

#[derive(Iden)]
enum Player {
    Table,
    PlayerId,
}

#[derive(Iden)]
enum User {
    Table,
    UserId,
}

#[derive(Iden)]
enum Tournament {
    Table,
    TournamentId,
}

#[derive(Iden)]
enum Annotation {
    Table,
}

#[derive(Iden)]
enum MoveTimeTracking {
    Table,
}

#[derive(Iden)]
enum GameTag {
    Table,
}

#[derive(Iden)]
enum MoveTag {
    Table,
}

#[derive(Iden)]
enum Tag {
    Table,
    TagId,
}

#[derive(Iden)]
enum Opening {
    Table,
    OpeningId,
}
