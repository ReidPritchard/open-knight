use super::structs::ChessGame;
use crate::parse::pgn::PgnToken;

impl std::fmt::Display for PgnToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PgnToken::MoveNumber(n) => write!(f, "{}.", n),
            PgnToken::Move(m) => write!(f, "{} ", m),
            PgnToken::Result(r) => write!(f, "{}", r),
            PgnToken::Tag(k, v) => write!(f, "[{} \"{}\"]", k, v),
            PgnToken::Comment(c) => write!(f, "{{{}}}", c),
            PgnToken::Variation(v) => write!(
                f,
                "({})",
                v.iter().map(|t| t.to_string()).collect::<String>()
            ),
            PgnToken::NAG(n) => write!(f, "${}", n),
            PgnToken::MoveSuffixNotation(m) => write!(f, "{}", m),
        }
    }
}

impl ChessGame {
    /// Converts the game to PGN format
    pub fn to_pgn(&self) -> String {
        let mut pgn = String::new();

        // Add standard tags
        pgn.push_str(&format!(
            "[Event \"{}\"]\n",
            self.tournament
                .as_ref()
                .map_or("Casual Game".to_string(), |t| t.name.clone())
        ));
        pgn.push_str(&format!(
            "[Site \"{}\"]\n",
            self.tournament
                .as_ref()
                .and_then(|t| t.location.as_ref())
                .map(String::as_str)
                .unwrap_or("?")
        ));
        pgn.push_str(&format!("[Date \"{}\"]\n", self.date));
        pgn.push_str(&format!(
            "[Round \"{}\"]\n",
            self.round.map_or("?".to_string(), |r| r.to_string())
        ));
        pgn.push_str(&format!("[White \"{}\"]\n", self.white_player.name));
        pgn.push_str(&format!("[Black \"{}\"]\n", self.black_player.name));
        pgn.push_str(&format!("[Result \"{}\"]\n", self.result));

        if let Some(ref opening) = self.opening {
            if let Some(ref eco) = opening.eco {
                pgn.push_str(&format!("[ECO \"{}\"]\n", eco));
            }
            if let Some(ref name) = opening.name {
                pgn.push_str(&format!("[Opening \"{}\"]\n", name));
            }
            if let Some(ref variation) = opening.variation {
                pgn.push_str(&format!("[Variation \"{}\"]\n", variation));
            }
        }

        if let Some(ref elo) = self.white_player.elo {
            pgn.push_str(&format!("[WhiteElo \"{}\"]\n", elo));
        }
        if let Some(ref elo) = self.black_player.elo {
            pgn.push_str(&format!("[BlackElo \"{}\"]\n", elo));
        }

        // Add any custom tags
        for tag in &self.tags {
            if !tag.starts_with("[Event ")
                && !tag.starts_with("[Site ")
                && !tag.starts_with("[Date ")
                && !tag.starts_with("[Round ")
                && !tag.starts_with("[White ")
                && !tag.starts_with("[Black ")
                && !tag.starts_with("[Result ")
                && !tag.starts_with("[ECO ")
                && !tag.starts_with("[Opening ")
                && !tag.starts_with("[Variation ")
                && !tag.starts_with("[WhiteElo ")
                && !tag.starts_with("[BlackElo ")
            {
                pgn.push_str(&format!("{}\n", tag));
            }
        }

        pgn.push('\n');

        // Add moves
        pgn.push_str(&self.move_tree.to_pgn_moves());
        pgn.push_str(&format!("{}", self.result));
        pgn
    }
}
