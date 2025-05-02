use crate::parse::uci::util::*;
use chumsky::prelude::*;

use super::{EngineResponse, IdInfo, ProtectionStatus, RegistrationStatus};

/**
 * These are the parsers for the simple responses from the engine.
 *
 * The simple responses are:
 * - uciok
 * - readyok
 * - id
 * - bestmove
 * - copyprotection
 * - registration
 */

/// Parse "uciok" response
pub fn uciok_parser() -> impl Parser<char, EngineResponse, Error = Simple<char>> {
    just("uciok")
        .padded()
        .map(|_| EngineResponse::UciOk)
        .labelled("uciok response")
}

/// Parse "readyok" response
pub fn readyok_parser() -> impl Parser<char, EngineResponse, Error = Simple<char>> {
    just("readyok")
        .padded()
        .map(|_| EngineResponse::ReadyOk)
        .labelled("readyok response")
}

/// Parse "id" response
pub fn id_parser() -> impl Parser<char, EngineResponse, Error = Simple<char>> {
    just("id")
        .padded()
        .ignore_then(
            just("name")
                .padded()
                .ignore_then(
                    take_until(end())
                        .map(|(chars, _)| chars.into_iter().collect::<String>().trim().to_string())
                        .map(IdInfo::Name),
                )
                .or(just("author").padded().ignore_then(
                    take_until(end())
                        .map(|(chars, _)| chars.into_iter().collect::<String>().trim().to_string())
                        .map(IdInfo::Author),
                )),
        )
        .map(EngineResponse::Id)
        .labelled("id response")
}

/// Parse "bestmove" response
pub fn bestmove_parser() -> impl Parser<char, EngineResponse, Error = Simple<char>> {
    just("bestmove")
        .padded()
        .ignore_then(chess_move())
        .then(just("ponder").padded().ignore_then(chess_move()).or_not())
        .map(|(best_move, ponder)| EngineResponse::BestMove { best_move, ponder })
        .labelled("bestmove response")
}

/// Parse "copyprotection" response
pub fn copyprotection_parser() -> impl Parser<char, EngineResponse, Error = Simple<char>> {
    just("copyprotection")
        .padded()
        .ignore_then(
            just("checking")
                .padded()
                .map(|_| ProtectionStatus::Checking)
                .or(just("ok").padded().map(|_| ProtectionStatus::Ok))
                .or(just("error").padded().map(|_| ProtectionStatus::Error)),
        )
        .map(EngineResponse::CopyProtection)
        .labelled("copyprotection response")
}

/// Parse "registration" response
pub fn registration_parser() -> impl Parser<char, EngineResponse, Error = Simple<char>> {
    just("registration")
        .padded()
        .ignore_then(
            just("checking")
                .padded()
                .map(|_| RegistrationStatus::Checking)
                .or(just("ok").padded().map(|_| RegistrationStatus::Ok))
                .or(just("error").padded().map(|_| RegistrationStatus::Error)),
        )
        .map(EngineResponse::Registration)
        .labelled("registration response")
}
