use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::Statement;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .col(
                        ColumnDef::new(User::UserId)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(User::Username)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(User::Email).string().not_null().unique_key())
                    .col(
                        ColumnDef::new(User::CreatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Player::Table)
                    .col(
                        ColumnDef::new(Player::PlayerId)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Player::Name).string().not_null())
                    .col(
                        ColumnDef::new(Player::EloRating)
                            .integer()
                            .check(Expr::cust("elo_rating BETWEEN 0 AND 5000")),
                    )
                    .col(ColumnDef::new(Player::Title).string().check(Expr::cust(
                        "title IN ('GM', 'IM', 'FM', 'CM', 'WGM', 'WIM', 'WFM', 'WCM', '')",
                    )))
                    .col(
                        ColumnDef::new(Player::CountryCode)
                            .string()
                            .check(Expr::cust("country_code GLOB '[A-Z][A-Z]'")),
                    )
                    .col(
                        ColumnDef::new(Player::CreatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp()),
                    )
                    .col(ColumnDef::new(Player::UpdatedAt).timestamp())
                    .to_owned(),
            )
            .await?;

        manager
            .get_connection()
            .execute(
                Statement::from_string(
                    manager.get_database_backend(),
                    r#"CREATE TRIGGER Player_Update
                    AFTER UPDATE ON Player
                    BEGIN
                        UPDATE Player SET updated_at = CURRENT_TIMESTAMP WHERE player_id = OLD.player_id;
                    END;"#.to_owned(),
                ),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Tournament::Table)
                    .col(
                        ColumnDef::new(Tournament::TournamentId)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Tournament::Name).string().not_null())
                    .col(ColumnDef::new(Tournament::Type).string().check(Expr::cust(
                        "type IN ('Swiss', 'Round Robin', 'Knockout', 'Match')",
                    )))
                    .col(ColumnDef::new(Tournament::TimeControl).string())
                    .col(
                        ColumnDef::new(Tournament::StartDate)
                            .string()
                            .check(Expr::cust(
                                "start_date IS NULL OR date(start_date) IS NOT NULL",
                            )),
                    )
                    .col(
                        ColumnDef::new(Tournament::EndDate)
                            .string()
                            .check(Expr::cust("end_date IS NULL OR date(end_date) IS NOT NULL")),
                    )
                    .col(ColumnDef::new(Tournament::Location).string())
                    .check(Expr::cust(
                        "((end_date IS NULL AND start_date IS NULL) OR (JULIANDAY(end_date) >= JULIANDAY(start_date)))",
                    ))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Opening::Table)
                    .col(
                        ColumnDef::new(Opening::OpeningId)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Opening::EcoCode)
                            .string()
                            .check(Expr::cust("eco_code GLOB '[A-E][0-9][0-9]'")),
                    )
                    .col(ColumnDef::new(Opening::Name).string().not_null())
                    .col(ColumnDef::new(Opening::Variation).string())
                    .index(
                        Index::create()
                            .unique()
                            .name("opening_unique")
                            .col(Opening::EcoCode)
                            .col(Opening::Name)
                            .col(Opening::Variation),
                    )
                    .to_owned(),
            )
            .await?;

        manager.create_table(
            Table::create()
                .table(Game::Table)
                .col(
                    ColumnDef::new(Game::GameId)
                        .integer()
                        .not_null()
                        .auto_increment()
                        .primary_key(),
                )
                .col(ColumnDef::new(Game::WhitePlayerId).integer().not_null())
                .col(ColumnDef::new(Game::BlackPlayerId).integer().not_null())
                .col(ColumnDef::new(Game::TournamentId).integer())
                .col(ColumnDef::new(Game::OpeningId).integer())
                .col(
                    ColumnDef::new(Game::Result).string().check(Expr::cust(
                        "result IN ('1-0', '0-1', '1/2-1/2', '*')",
                    )),
                )
                .col(
                    ColumnDef::new(Game::Termination).string().check(Expr::cust(
                        "termination IN ('Normal', 'Time forfeit', 'Abandoned', 'Rules infraction')",
                    )),
                )
                .col(ColumnDef::new(Game::RoundNumber).integer())
                .col(
                    ColumnDef::new(Game::DatePlayed)
                        .string()
                        .check(Expr::cust(
                            "date_played IS NULL OR date(date_played) IS NOT NULL",
                        )),
                )
                .col(ColumnDef::new(Game::TimeControl).string())
                .col(ColumnDef::new(Game::Fen).string())
                .col(ColumnDef::new(Game::Variant).string())
                .col(ColumnDef::new(Game::Pgn).string().not_null())
                .col(
                    ColumnDef::new(Game::CreatedAt)
                        .timestamp()
                        .default(Expr::current_timestamp()),
                )
                .foreign_key(
                    ForeignKey::create()
                        .from(Game::Table, Game::WhitePlayerId)
                        .to(Player::Table, Player::PlayerId),
                )
                .foreign_key(
                    ForeignKey::create()
                        .from(Game::Table, Game::BlackPlayerId)
                        .to(Player::Table, Player::PlayerId),
                )
                .foreign_key(
                    ForeignKey::create()
                        .from(Game::Table, Game::TournamentId)
                        .to(Tournament::Table, Tournament::TournamentId),
                )
                .foreign_key(
                    ForeignKey::create()
                        .from(Game::Table, Game::OpeningId)
                        .to(Opening::Table, Opening::OpeningId),
                )
                .check(Expr::cust("white_player_id != black_player_id"))
                .to_owned(),
        ).await?;

        manager
            .create_table(
                Table::create()
                    .table(Position::Table)
                    .col(
                        ColumnDef::new(Position::PositionId)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Position::Fen)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(Position::FenHash).string().not_null())
                    .col(
                        ColumnDef::new(Position::CreatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Move::Table)
                    .col(
                        ColumnDef::new(Move::MoveId)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Move::GameId).integer().not_null())
                    .col(ColumnDef::new(Move::ParentMoveId).integer())
                    .col(
                        ColumnDef::new(Move::VariationOrder)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(Move::PlyNumber)
                            .integer()
                            .not_null()
                            .check(Expr::cust("ply_number >= 0")),
                    )
                    .col(ColumnDef::new(Move::San).string().not_null())
                    .col(ColumnDef::new(Move::Uci).string().not_null())
                    .col(ColumnDef::new(Move::PositionId).integer().not_null())
                    .col(
                        ColumnDef::new(Move::CreatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Move::Table, Move::GameId)
                            .to(Game::Table, Game::GameId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Move::Table, Move::PositionId)
                            .to(Position::Table, Position::PositionId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Move::Table, Move::ParentMoveId)
                            .to(Move::Table, Move::MoveId),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Annotation::Table)
                    .col(
                        ColumnDef::new(Annotation::AnnotationId)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Annotation::MoveId).integer().not_null())
                    .col(ColumnDef::new(Annotation::UserId).integer()) // TODO: add .not_null() once user table is active
                    .col(ColumnDef::new(Annotation::Comment).text())
                    .col(ColumnDef::new(Annotation::Arrows).text())
                    .col(ColumnDef::new(Annotation::Highlights).text())
                    .col(
                        ColumnDef::new(Annotation::CreatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Annotation::Table, Annotation::MoveId)
                            .to(Move::Table, Move::MoveId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Annotation::Table, Annotation::UserId)
                            .to(User::Table, User::UserId),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Evaluation::Table)
                    .col(
                        ColumnDef::new(Evaluation::EvaluationId)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Evaluation::PositionId).integer().not_null())
                    .col(ColumnDef::new(Evaluation::Score).double())
                    .col(
                        ColumnDef::new(Evaluation::Type)
                            .string()
                            .check(Expr::cust("type IN ('cp', 'mate')")),
                    )
                    .col(ColumnDef::new(Evaluation::BestLine).text())
                    .col(
                        ColumnDef::new(Evaluation::Depth)
                            .integer()
                            .check(Expr::cust("depth > 0")),
                    )
                    .col(ColumnDef::new(Evaluation::EngineName).string().not_null())
                    .col(
                        ColumnDef::new(Evaluation::EngineVersion)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Evaluation::CreatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Evaluation::Table, Evaluation::PositionId)
                            .to(Position::Table, Position::PositionId),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(MoveTimeTracking::Table)
                    .col(
                        ColumnDef::new(MoveTimeTracking::TimeTrackingId)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(MoveTimeTracking::MoveId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(MoveTimeTracking::TimeSpentMs)
                            .integer()
                            .check(Expr::cust("time_spent_ms > 0")),
                    )
                    .col(
                        ColumnDef::new(MoveTimeTracking::TimeLeftMs)
                            .integer()
                            .check(Expr::cust("time_left_ms >= 0")),
                    )
                    .col(
                        ColumnDef::new(MoveTimeTracking::ClockType)
                            .string()
                            .check(Expr::cust(
                                "clock_type IN ('bronstein', 'fischer', 'simple')",
                            )),
                    )
                    .col(
                        ColumnDef::new(MoveTimeTracking::CreatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(MoveTimeTracking::Table, MoveTimeTracking::MoveId)
                            .to(Move::Table, Move::MoveId),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Tag::Table)
                    .col(
                        ColumnDef::new(Tag::TagId)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Tag::Name).string().not_null().unique_key())
                    .col(ColumnDef::new(Tag::Description).text())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(GameTag::Table)
                    .col(ColumnDef::new(GameTag::GameId).integer().not_null())
                    .col(ColumnDef::new(GameTag::TagId).integer().not_null())
                    .primary_key(
                        Index::create()
                            .name("pk_game_tag")
                            .col(GameTag::GameId)
                            .col(GameTag::TagId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(GameTag::Table, GameTag::GameId)
                            .to(Game::Table, Game::GameId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(GameTag::Table, GameTag::TagId)
                            .to(Tag::Table, Tag::TagId),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(MoveTag::Table)
                    .col(ColumnDef::new(MoveTag::MoveId).integer().not_null())
                    .col(ColumnDef::new(MoveTag::TagId).integer().not_null())
                    .primary_key(
                        Index::create()
                            .name("pk_move_tag")
                            .col(MoveTag::MoveId)
                            .col(MoveTag::TagId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(MoveTag::Table, MoveTag::MoveId)
                            .to(Move::Table, Move::MoveId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(MoveTag::Table, MoveTag::TagId)
                            .to(Tag::Table, Tag::TagId),
                    )
                    .to_owned(),
            )
            .await?;

        // Indexes
        manager
            .create_index(
                Index::create()
                    .name("idx_game_players")
                    .table(Game::Table)
                    .col(Game::WhitePlayerId)
                    .col(Game::BlackPlayerId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_game_date")
                    .table(Game::Table)
                    .col(Game::DatePlayed)
                    .to_owned(),
            )
            .await?;

        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                "CREATE INDEX idx_position_fen_hash ON Position(fen_hash);".to_owned(),
            ))
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_move_game_ply")
                    .table(Move::Table)
                    .col(Move::GameId)
                    .col(Move::PlyNumber)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_evaluation_position")
                    .table(Evaluation::Table)
                    .col(Evaluation::PositionId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(MoveTag::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(GameTag::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Tag::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(MoveTimeTracking::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Evaluation::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Annotation::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Move::Table).to_owned())
            .await?;

        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                "DROP TABLE Position".to_owned(),
            ))
            .await?;

        manager
            .drop_table(Table::drop().table(Game::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Opening::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Tournament::Table).to_owned())
            .await?;

        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                "DROP TRIGGER IF EXISTS Player_Update".to_owned(),
            ))
            .await?;
        manager
            .drop_table(Table::drop().table(Player::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await?;

        Ok(())
    }
}

// Iden enums for each table
#[derive(Iden)]
enum User {
    Table,
    UserId,
    Username,
    Email,
    CreatedAt,
}

#[derive(Iden)]
enum Player {
    Table,
    PlayerId,
    Name,
    EloRating,
    Title,
    CountryCode,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum Tournament {
    Table,
    TournamentId,
    Name,
    Type,
    TimeControl,
    StartDate,
    EndDate,
    Location,
}

#[derive(Iden)]
enum Opening {
    Table,
    OpeningId,
    EcoCode,
    Name,
    Variation,
}

#[derive(Iden)]
enum Game {
    Table,
    GameId,
    WhitePlayerId,
    BlackPlayerId,
    TournamentId,
    OpeningId,
    Result,
    Termination,
    RoundNumber,
    DatePlayed,
    TimeControl,
    Fen,
    Variant,
    Pgn,
    CreatedAt,
}

#[derive(Iden)]
enum Position {
    Table,
    PositionId,
    Fen,
    FenHash,
    CreatedAt,
}

#[derive(Iden)]
enum Move {
    Table,
    MoveId,
    GameId,
    ParentMoveId,
    VariationOrder,
    PlyNumber,
    San,
    Uci,
    PositionId, // The resulting position after the move is played
    CreatedAt,
}

#[derive(Iden)]
enum Annotation {
    Table,
    AnnotationId,
    MoveId,
    UserId,
    Comment,
    Arrows,
    Highlights,
    CreatedAt,
}

#[derive(Iden)]
enum Evaluation {
    Table,
    EvaluationId,
    PositionId,
    Score,
    Type,
    BestLine,
    Depth,
    EngineName,
    EngineVersion,
    CreatedAt,
}

#[derive(Iden)]
enum MoveTimeTracking {
    Table,
    TimeTrackingId,
    MoveId,
    TimeSpentMs,
    TimeLeftMs,
    ClockType,
    CreatedAt,
}

#[derive(Iden)]
enum Tag {
    Table,
    TagId,
    Name,
    Description,
}

#[derive(Iden)]
enum GameTag {
    Table,
    GameId,
    TagId,
}

#[derive(Iden)]
enum MoveTag {
    Table,
    MoveId,
    TagId,
}
