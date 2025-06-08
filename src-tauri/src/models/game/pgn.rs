use crate::models::parse::parse_pgn_tokens;
use crate::models::{ChessGame, ChessOpening, ChessPlayer, ChessPosition, ChessTournament};
use crate::parse::pgn::PgnToken;
use std::error::Error;

impl ChessGame {
    /// Parses multiple PGN games into ChessGame objects.
    ///
    /// This function takes a string containing one or more chess games in PGN format
    /// and converts them into a vector of ChessGame objects. The conversion process:
    ///
    /// 1. Parses the PGN string into tokens using the PGN parser
    /// 2. Processes game metadata from PGN tags (players, tournament, opening, etc.)
    /// 3. Processes moves, including any attached comments and variations
    /// 4. Creates temporary game objects (without database IDs)
    ///
    /// # Arguments
    /// * `pgn` - A string containing one or more chess games in PGN format
    ///
    /// # Returns
    /// * `Result<Vec<ChessGame>, Box<dyn Error>>` - A vector of parsed chess games or an error
    pub fn from_pgn_games(pgn: &str) -> Result<Vec<Self>, Box<dyn Error>> {
        let games = crate::parse::pgn::parse_pgn_games(pgn)?;
        let mut result = Vec::new();

        for game_tokens in games {
            let mut tags = Vec::new();
            let mut result_str = String::from("*");
            let mut move_tokens = Vec::new();

            // Process all PGN tokens for the current game
            for token in &game_tokens {
                match token {
                    PgnToken::Tag(key, value) => tags.push((key.clone(), value.clone())),
                    PgnToken::Result(r) => result_str = r.clone(),
                    // Collect all move-related tokens for later processing
                    PgnToken::Move(_)
                    | PgnToken::MoveNumber(_)
                    | PgnToken::Comment(_)
                    | PgnToken::Variation(_)
                    | PgnToken::NAG(_)
                    | PgnToken::MoveSuffixNotation(_) => {
                        move_tokens.push(token.clone());
                    }
                }
            }

            // Helper function to extract tag values
            let get_tag = |name: &str| -> Option<String> {
                tags.iter()
                    .find(|(key, _)| key == name)
                    .map(|(_, value)| value.clone())
            };

            let root_position = ChessPosition::from_fen(get_tag("FEN"), None)?;

            let move_tree = parse_pgn_tokens(0, root_position.clone(), &move_tokens)?;

            // Create a temporary game object
            let game = ChessGame {
                id: 0, // Database ID will be set when saving
                white_player: ChessPlayer {
                    id: 0,
                    name: get_tag("White").unwrap_or_else(|| "Unknown".to_string()),
                    elo: get_tag("WhiteElo").and_then(|e| e.parse().ok()),
                    country: None,
                },
                black_player: ChessPlayer {
                    id: 0,
                    name: get_tag("Black").unwrap_or_else(|| "Unknown".to_string()),
                    elo: get_tag("BlackElo").and_then(|e| e.parse().ok()),
                    country: None,
                },
                tournament: get_tag("Event").map(|name| ChessTournament {
                    id: 0,
                    name,
                    tournament_type: None,
                    start_date: get_tag("EventDate"),
                    end_date: None,
                    location: get_tag("Site"),
                    time_control: get_tag("TimeControl"),
                }),
                opening: get_tag("ECO").map(|eco| ChessOpening {
                    id: 0,
                    eco: Some(eco),
                    name: get_tag("Opening"),
                    variation: get_tag("Variation"),
                }),
                result: result_str,
                round: get_tag("Round").and_then(|r| r.parse().ok()),
                date: get_tag("Date")
                    .map(|d| d.replace('.', "-"))
                    .unwrap_or_else(|| "????-??-??".to_string()),
                move_tree,
                tags: tags
                    .iter()
                    .map(|(k, v)| format!("[{} \"{}\"]", k, v))
                    .collect(),
                fen: Some(root_position.fen),
                pgn: Some(game_tokens.iter().map(|t| t.to_string()).collect()),
            };

            result.push(game);
        }

        Ok(result)
    }
}
