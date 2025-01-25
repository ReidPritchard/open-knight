use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Read and execute the SQL file
        manager
            .create_table(
                Table::create()
                    .table(Player::Table)
                    .col(
                        ColumnDef::new(Player::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Player::Name).string().not_null())
                    .col(ColumnDef::new(Player::LastKnownElo).integer())
                    .col(ColumnDef::new(Player::Country).string())
                    .col(ColumnDef::new(Player::CreatedAt).string().not_null())
                    .col(ColumnDef::new(Player::UpdatedAt).string().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Tournament::Table)
                    .col(
                        ColumnDef::new(Tournament::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Tournament::Name).string().not_null())
                    .col(ColumnDef::new(Tournament::Type_).string())
                    .col(ColumnDef::new(Tournament::StartDate).string())
                    .col(ColumnDef::new(Tournament::EndDate).string())
                    .col(ColumnDef::new(Tournament::Location).string())
                    .col(ColumnDef::new(Tournament::CreatedAt).string().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Opening::Table)
                    .col(
                        ColumnDef::new(Opening::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Opening::EcoCode).string())
                    .col(ColumnDef::new(Opening::Name).string())
                    .col(ColumnDef::new(Opening::Variation).string())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Game::Table)
                    .col(
                        ColumnDef::new(Game::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Game::WhitePlayerId).integer().not_null())
                    .col(ColumnDef::new(Game::WhitePlayerElo).integer())
                    .col(ColumnDef::new(Game::BlackPlayerId).integer().not_null())
                    .col(ColumnDef::new(Game::BlackPlayerElo).integer())
                    .col(ColumnDef::new(Game::TournamentId).integer())
                    .col(ColumnDef::new(Game::OpeningId).integer())
                    .col(ColumnDef::new(Game::Result).string())
                    .col(ColumnDef::new(Game::RoundNumber).integer())
                    .col(ColumnDef::new(Game::DatePlayed).string())
                    .col(ColumnDef::new(Game::Fen).string())
                    .col(ColumnDef::new(Game::Pgn).string())
                    .col(ColumnDef::new(Game::CreatedAt).string().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_game_white_player")
                            .from(Game::Table, Game::WhitePlayerId)
                            .to(Player::Table, Player::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_game_black_player")
                            .from(Game::Table, Game::BlackPlayerId)
                            .to(Player::Table, Player::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_game_tournament")
                            .from(Game::Table, Game::TournamentId)
                            .to(Tournament::Table, Tournament::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_game_opening")
                            .from(Game::Table, Game::OpeningId)
                            .to(Opening::Table, Opening::Id),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Position::Table)
                    .col(
                        ColumnDef::new(Position::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Position::Fen).string().not_null())
                    .col(ColumnDef::new(Position::FenHash).string())
                    .col(ColumnDef::new(Position::CreatedAt).string().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Move::Table)
                    .col(
                        ColumnDef::new(Move::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Move::GameId).integer().not_null())
                    .col(ColumnDef::new(Move::MoveNumber).integer())
                    .col(ColumnDef::new(Move::PlayerColor).string())
                    .col(ColumnDef::new(Move::MoveNotation).string().not_null())
                    .col(ColumnDef::new(Move::ParentMoveId).integer())
                    .col(ColumnDef::new(Move::PositionId).integer())
                    .col(ColumnDef::new(Move::CreatedAt).string().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_move_game")
                            .from(Move::Table, Move::GameId)
                            .to(Game::Table, Game::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_move_parent")
                            .from(Move::Table, Move::ParentMoveId)
                            .to(Move::Table, Move::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_move_position")
                            .from(Move::Table, Move::PositionId)
                            .to(Position::Table, Position::Id),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Annotation::Table)
                    .col(
                        ColumnDef::new(Annotation::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Annotation::MoveId).integer().not_null())
                    .col(ColumnDef::new(Annotation::UserId).integer())
                    .col(ColumnDef::new(Annotation::Comment).string())
                    .col(ColumnDef::new(Annotation::Arrows).string())
                    .col(ColumnDef::new(Annotation::Highlights).string())
                    .col(ColumnDef::new(Annotation::CreatedAt).string().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_annotation_move")
                            .from(Annotation::Table, Annotation::MoveId)
                            .to(Move::Table, Move::Id),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Evaluation::Table)
                    .col(
                        ColumnDef::new(Evaluation::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Evaluation::PositionId).integer().not_null())
                    .col(ColumnDef::new(Evaluation::EvaluationScore).float())
                    .col(ColumnDef::new(Evaluation::EvaluationType).string())
                    .col(ColumnDef::new(Evaluation::Depth).integer())
                    .col(ColumnDef::new(Evaluation::EngineName).string())
                    .col(ColumnDef::new(Evaluation::CreatedAt).string().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_evaluation_position")
                            .from(Evaluation::Table, Evaluation::PositionId)
                            .to(Position::Table, Position::Id),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Tag::Table)
                    .col(
                        ColumnDef::new(Tag::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Tag::Name).string().not_null())
                    .col(ColumnDef::new(Tag::Description).string())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(GameTag::Table)
                    .col(
                        ColumnDef::new(GameTag::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(GameTag::GameId).integer().not_null())
                    .col(ColumnDef::new(GameTag::TagId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_gametag_game")
                            .from(GameTag::Table, GameTag::GameId)
                            .to(Game::Table, Game::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_gametag_tag")
                            .from(GameTag::Table, GameTag::TagId)
                            .to(Tag::Table, Tag::Id),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(MoveTag::Table)
                    .col(
                        ColumnDef::new(MoveTag::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(MoveTag::MoveId).integer().not_null())
                    .col(ColumnDef::new(MoveTag::TagId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_movetag_move")
                            .from(MoveTag::Table, MoveTag::MoveId)
                            .to(Move::Table, Move::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_movetag_tag")
                            .from(MoveTag::Table, MoveTag::TagId)
                            .to(Tag::Table, Tag::Id),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(MoveTimeTracking::Table)
                    .col(
                        ColumnDef::new(MoveTimeTracking::Id)
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
                    .col(ColumnDef::new(MoveTimeTracking::TimeSpentMs).integer())
                    .col(ColumnDef::new(MoveTimeTracking::TimeLeftMs).integer())
                    .col(
                        ColumnDef::new(MoveTimeTracking::CreatedAt)
                            .string()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_movetimetracking_move")
                            .from(MoveTimeTracking::Table, MoveTimeTracking::MoveId)
                            .to(Move::Table, Move::Id),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(MoveTimeTracking::Table).to_owned())
            .await?;
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
            .drop_table(Table::drop().table(Evaluation::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Annotation::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Move::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Position::Table).to_owned())
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
            .drop_table(Table::drop().table(Player::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum Player {
    Table,
    Id,
    Name,
    LastKnownElo,
    Country,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Tournament {
    Table,
    Id,
    Name,
    Type_,
    StartDate,
    EndDate,
    Location,
    CreatedAt,
}

#[derive(DeriveIden)]
enum Opening {
    Table,
    Id,
    EcoCode,
    Name,
    Variation,
}

#[derive(DeriveIden)]
enum Game {
    Table,
    Id,
    WhitePlayerId,
    WhitePlayerElo,
    BlackPlayerId,
    BlackPlayerElo,
    TournamentId,
    OpeningId,
    Result,
    RoundNumber,
    DatePlayed,
    Fen,
    Pgn,
    CreatedAt,
}

#[derive(DeriveIden)]
enum Position {
    Table,
    Id,
    Fen,
    FenHash,
    CreatedAt,
}

#[derive(DeriveIden)]
enum Move {
    Table,
    Id,
    GameId,
    MoveNumber,
    PlayerColor,
    MoveNotation,
    ParentMoveId,
    PositionId,
    CreatedAt,
}

#[derive(DeriveIden)]
enum Annotation {
    Table,
    Id,
    MoveId,
    UserId,
    Comment,
    Arrows,
    Highlights,
    CreatedAt,
}

#[derive(DeriveIden)]
enum Evaluation {
    Table,
    Id,
    PositionId,
    EvaluationScore,
    EvaluationType,
    Depth,
    EngineName,
    CreatedAt,
}

#[derive(DeriveIden)]
enum Tag {
    Table,
    Id,
    Name,
    Description,
}

#[derive(DeriveIden)]
enum GameTag {
    Table,
    Id,
    GameId,
    TagId,
}

#[derive(DeriveIden)]
enum MoveTag {
    Table,
    Id,
    MoveId,
    TagId,
}

#[derive(DeriveIden)]
enum MoveTimeTracking {
    Table,
    Id,
    MoveId,
    TimeSpentMs,
    TimeLeftMs,
    CreatedAt,
}
