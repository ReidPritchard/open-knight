use sea_orm_migration::prelude::*;

#[derive(Iden)]
pub enum Player {
    Table,
    Id,
    Name,
    LastKnownElo,
    Country,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
pub enum Tournament {
    Table,
    Id,
    Name,
    #[iden = "type"]
    Type,
    StartDate,
    EndDate,
    Location,
}

#[derive(Iden)]
pub enum Opening {
    Table,
    Id,
    EcoCode,
    Name,
    Variation,
}

#[derive(Iden)]
pub enum Game {
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

#[derive(Iden)]
pub enum Position {
    Table,
    Id,
    Fen,
    FenHash,
    CreatedAt,
}

#[derive(Iden)]
pub enum Move {
    Table,
    Id,
    GameId,
    ParentMoveId,
    MoveNumber,
    PlayerColor,
    MoveNotation,
    PositionId,
    CreatedAt,
}

#[derive(Iden)]
pub enum Annotation {
    Table,
    Id,
    MoveId,
    UserId,
    Comment,
    Arrows,
    Highlights,
    CreatedAt,
}

#[derive(Iden)]
pub enum Evaluation {
    Table,
    Id,
    PositionId,
    EvaluationScore,
    EvaluationType,
    Depth,
    EngineName,
    CreatedAt,
}

#[derive(Iden)]
pub enum MoveTimeTracking {
    Table,
    Id,
    MoveId,
    TimeSpentMs,
    TimeLeftMs,
    CreatedAt,
}

#[derive(Iden)]
pub enum Tag {
    Table,
    Id,
    Name,
    Description,
}

#[derive(Iden)]
pub enum GameTag {
    Table,
    Id,
    GameId,
    TagId,
}

#[derive(Iden)]
pub enum MoveTag {
    Table,
    Id,
    MoveId,
    TagId,
}
