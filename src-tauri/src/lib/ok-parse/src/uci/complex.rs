use chumsky::prelude::*;

use super::{
    Bound, EngineResponse, InfoParams, OptionDefinition, OptionType, Score,
    UciParseError,
};

/**
 * These are the parsers for the complex responses from the engine.
 *
 * The complex responses are:
 * - info
 * - option
 *
 * First we capture the token and the rest of the line, and then we process it separately.
 * That's why we have two parsers for each command here.
 */

/// Parser for the "info" command that captures the token and the rest of the line
pub fn info_token_parser() -> impl Parser<char, String, Error = Simple<char>> {
    just("info")
        .padded()
        .ignore_then(
            take_until(end())
                .map(|(chars, _)| chars.into_iter().collect::<String>()),
        )
        .labelled("info response")
}

/// Parser for the "option" command that captures the token and the rest of the line
pub fn option_token_parser() -> impl Parser<char, String, Error = Simple<char>>
{
    just("option")
        .padded()
        .ignore_then(
            take_until(end())
                .map(|(chars, _)| chars.into_iter().collect::<String>()),
        )
        .labelled("option response")
}

/// Parse the parameters of an "info" command from a string
pub fn parse_info_params(
    input: String
) -> Result<EngineResponse, UciParseError> {
    let tokens: Vec<&str> = input.split_whitespace().collect();
    let mut params = InfoParams::default();
    let mut i = 0;

    while i < tokens.len() {
        match tokens[i] {
            "depth" => {
                if i + 1 < tokens.len() {
                    match tokens[i + 1].parse::<u32>() {
                        Ok(depth) => {
                            params.depth = Some(depth);
                            i += 2;
                        }
                        Err(_) => {
                            return Err(UciParseError::InvalidValue {
                                param: "depth".to_string(),
                                value: tokens[i + 1].to_string(),
                            });
                        }
                    }
                } else {
                    return Err(UciParseError::MissingValue {
                        param: "depth".to_string(),
                    });
                }
            }
            "seldepth" => {
                if i + 1 < tokens.len() {
                    match tokens[i + 1].parse::<u32>() {
                        Ok(seldepth) => {
                            params.seldepth = Some(seldepth);
                            i += 2;
                        }
                        Err(_) => {
                            return Err(UciParseError::InvalidValue {
                                param: "seldepth".to_string(),
                                value: tokens[i + 1].to_string(),
                            });
                        }
                    }
                } else {
                    return Err(UciParseError::MissingValue {
                        param: "seldepth".to_string(),
                    });
                }
            }
            "time" => {
                if i + 1 < tokens.len() {
                    match tokens[i + 1].parse::<u64>() {
                        Ok(time) => {
                            params.time = Some(time);
                            i += 2;
                        }
                        Err(_) => {
                            return Err(UciParseError::InvalidValue {
                                param: "time".to_string(),
                                value: tokens[i + 1].to_string(),
                            });
                        }
                    }
                } else {
                    return Err(UciParseError::MissingValue {
                        param: "time".to_string(),
                    });
                }
            }
            "nodes" => {
                if i + 1 < tokens.len() {
                    match tokens[i + 1].parse::<u64>() {
                        Ok(nodes) => {
                            params.nodes = Some(nodes);
                            i += 2;
                        }
                        Err(_) => {
                            return Err(UciParseError::InvalidValue {
                                param: "nodes".to_string(),
                                value: tokens[i + 1].to_string(),
                            });
                        }
                    }
                } else {
                    return Err(UciParseError::MissingValue {
                        param: "nodes".to_string(),
                    });
                }
            }
            "multipv" => {
                if i + 1 < tokens.len() {
                    match tokens[i + 1].parse::<u32>() {
                        Ok(multipv) => {
                            params.multipv = Some(multipv);
                            i += 2;
                        }
                        Err(_) => {
                            return Err(UciParseError::InvalidValue {
                                param: "multipv".to_string(),
                                value: tokens[i + 1].to_string(),
                            });
                        }
                    }
                } else {
                    return Err(UciParseError::MissingValue {
                        param: "multipv".to_string(),
                    });
                }
            }
            "score" => {
                // Parse the score parameter (more complex)
                i += 1; // Skip "score"

                if i >= tokens.len() {
                    return Err(UciParseError::MissingValue {
                        param: "score".to_string(),
                    });
                }

                match tokens[i] {
                    "cp" => {
                        i += 1;
                        if i >= tokens.len() {
                            return Err(UciParseError::MissingValue {
                                param: "score cp".to_string(),
                            });
                        }

                        match tokens[i].parse::<i32>() {
                            Ok(value) => {
                                let mut bound = None;
                                i += 1;

                                // Check for bound
                                if i < tokens.len() {
                                    match tokens[i] {
                                        "lowerbound" => {
                                            bound = Some(Bound::Lower);
                                            i += 1;
                                        }
                                        "upperbound" => {
                                            bound = Some(Bound::Upper);
                                            i += 1;
                                        }
                                        _ => {} // Not a bound
                                    }
                                }

                                params.score =
                                    Some(Score::Centipawns { value, bound });
                            }
                            Err(_) => {
                                return Err(UciParseError::InvalidValue {
                                    param: "score cp".to_string(),
                                    value: tokens[i].to_string(),
                                });
                            }
                        }
                    }
                    "mate" => {
                        i += 1;
                        if i >= tokens.len() {
                            return Err(UciParseError::MissingValue {
                                param: "score mate".to_string(),
                            });
                        }

                        match tokens[i].parse::<i32>() {
                            Ok(value) => {
                                params.score = Some(Score::Mate(value));
                                i += 1;
                            }
                            Err(_) => {
                                return Err(UciParseError::InvalidValue {
                                    param: "score mate".to_string(),
                                    value: tokens[i].to_string(),
                                });
                            }
                        }
                    }
                    _ => {
                        return Err(UciParseError::InvalidValue {
                            param: "score".to_string(),
                            value: tokens[i].to_string(),
                        });
                    }
                }
            }
            "currmove" => {
                if i + 1 < tokens.len() {
                    params.currmove = Some(tokens[i + 1].to_string());
                    i += 2;
                } else {
                    return Err(UciParseError::MissingValue {
                        param: "currmove".to_string(),
                    });
                }
            }
            "currmovenumber" => {
                if i + 1 < tokens.len() {
                    match tokens[i + 1].parse::<u32>() {
                        Ok(currmovenumber) => {
                            params.currmovenumber = Some(currmovenumber);
                            i += 2;
                        }
                        Err(_) => {
                            return Err(UciParseError::InvalidValue {
                                param: "currmovenumber".to_string(),
                                value: tokens[i + 1].to_string(),
                            });
                        }
                    }
                } else {
                    return Err(UciParseError::MissingValue {
                        param: "currmovenumber".to_string(),
                    });
                }
            }
            "hashfull" => {
                if i + 1 < tokens.len() {
                    match tokens[i + 1].parse::<u32>() {
                        Ok(hashfull) => {
                            params.hashfull = Some(hashfull);
                            i += 2;
                        }
                        Err(_) => {
                            return Err(UciParseError::InvalidValue {
                                param: "hashfull".to_string(),
                                value: tokens[i + 1].to_string(),
                            });
                        }
                    }
                } else {
                    return Err(UciParseError::MissingValue {
                        param: "hashfull".to_string(),
                    });
                }
            }
            "nps" => {
                if i + 1 < tokens.len() {
                    match tokens[i + 1].parse::<u64>() {
                        Ok(nps) => {
                            params.nps = Some(nps);
                            i += 2;
                        }
                        Err(_) => {
                            return Err(UciParseError::InvalidValue {
                                param: "nps".to_string(),
                                value: tokens[i + 1].to_string(),
                            });
                        }
                    }
                } else {
                    return Err(UciParseError::MissingValue {
                        param: "nps".to_string(),
                    });
                }
            }
            "tbhits" => {
                if i + 1 < tokens.len() {
                    match tokens[i + 1].parse::<u64>() {
                        Ok(tbhits) => {
                            params.tbhits = Some(tbhits);
                            i += 2;
                        }
                        Err(_) => {
                            return Err(UciParseError::InvalidValue {
                                param: "tbhits".to_string(),
                                value: tokens[i + 1].to_string(),
                            });
                        }
                    }
                } else {
                    return Err(UciParseError::MissingValue {
                        param: "tbhits".to_string(),
                    });
                }
            }
            "sbhits" => {
                if i + 1 < tokens.len() {
                    match tokens[i + 1].parse::<u64>() {
                        Ok(sbhits) => {
                            params.sbhits = Some(sbhits);
                            i += 2;
                        }
                        Err(_) => {
                            return Err(UciParseError::InvalidValue {
                                param: "sbhits".to_string(),
                                value: tokens[i + 1].to_string(),
                            });
                        }
                    }
                } else {
                    return Err(UciParseError::MissingValue {
                        param: "sbhits".to_string(),
                    });
                }
            }
            "cpuload" => {
                if i + 1 < tokens.len() {
                    match tokens[i + 1].parse::<u32>() {
                        Ok(cpuload) => {
                            params.cpuload = Some(cpuload);
                            i += 2;
                        }
                        Err(_) => {
                            return Err(UciParseError::InvalidValue {
                                param: "cpuload".to_string(),
                                value: tokens[i + 1].to_string(),
                            });
                        }
                    }
                } else {
                    return Err(UciParseError::MissingValue {
                        param: "cpuload".to_string(),
                    });
                }
            }
            "pv" => {
                // Parse the principal variation - a sequence of moves
                i += 1; // Skip "pv"
                let mut moves = Vec::new();

                while i < tokens.len() {
                    // Simple check for a move: 4-5 characters and alphanumeric
                    let token = tokens[i];
                    if (token.len() == 4 || token.len() == 5)
                        && token.chars().all(|c| c.is_alphanumeric())
                    {
                        moves.push(token.to_string());
                        i += 1;
                    } else {
                        // Not a move, break
                        break;
                    }
                }

                params.pv = Some(moves);
            }
            "refutation" => {
                // Parse refutation line - a sequence of moves
                i += 1; // Skip "refutation"
                let mut moves = Vec::new();

                while i < tokens.len() {
                    // Simple check for a move: 4-5 characters and alphanumeric
                    let token = tokens[i];
                    if (token.len() == 4 || token.len() == 5)
                        && token.chars().all(|c| c.is_alphanumeric())
                    {
                        moves.push(token.to_string());
                        i += 1;
                    } else {
                        // Not a move, break
                        break;
                    }
                }

                params.refutation = Some(moves);
            }
            "currline" => {
                // Parse current line - optional CPU number followed by a sequence of moves
                i += 1; // Skip "currline"

                let mut cpu_num = None;
                let mut moves = Vec::new();

                // Check if the first token is a CPU number
                if i < tokens.len() {
                    if let Ok(num) = tokens[i].parse::<u32>() {
                        cpu_num = Some(num);
                        i += 1;
                    }
                }

                // Parse moves
                while i < tokens.len() {
                    let token = tokens[i];
                    if (token.len() == 4 || token.len() == 5)
                        && token.chars().all(|c| c.is_alphanumeric())
                    {
                        moves.push(token.to_string());
                        i += 1;
                    } else {
                        // Not a move, break
                        break;
                    }
                }

                params.currline = Some((cpu_num, moves));
            }
            "string" => {
                // String parameter consumes all remaining tokens
                i += 1; // Skip "string"
                if i < tokens.len() {
                    let string_content = tokens[i..].join(" ");
                    params.string = Some(string_content);
                    break; // End parsing as string consumes everything
                }
            }
            _ => {
                // Unknown parameter, skip
                i += 1;
            }
        }
    }

    Ok(EngineResponse::Info(params))
}

/// Parse the parameters of an "option" command from a string
pub fn parse_option_params(
    input: String
) -> Result<EngineResponse, UciParseError> {
    let tokens: Vec<&str> = input.split_whitespace().collect();

    // Find positions of key tokens
    let name_pos = tokens.iter().position(|&t| t == "name");
    let type_pos = tokens.iter().position(|&t| t == "type");

    if name_pos.is_none()
        || type_pos.is_none()
        || name_pos.unwrap() >= type_pos.unwrap()
    {
        return Err(UciParseError::ParseFailure {
            input,
            message: "Invalid option format, expected 'name' before 'type'"
                .to_string(),
        });
    }

    // Extract option name (between "name" and "type")
    let name_start = name_pos.unwrap() + 1;
    let name = tokens[name_start..type_pos.unwrap()].join(" ");

    // Extract option type
    let type_pos = type_pos.unwrap();
    if type_pos + 1 >= tokens.len() {
        return Err(UciParseError::MissingValue {
            param: "type".to_string(),
        });
    }

    let option_type = match tokens[type_pos + 1] {
        "check" => OptionType::Check,
        "spin" => OptionType::Spin,
        "combo" => OptionType::Combo,
        "button" => OptionType::Button,
        "string" => OptionType::String,
        _ => {
            return Err(UciParseError::InvalidValue {
                param: "type".to_string(),
                value: tokens[type_pos + 1].to_string(),
            });
        }
    };

    // Extract default value, min, max, and var
    let mut default = None;
    let mut min = None;
    let mut max = None;
    let mut var = Vec::new();

    let mut i = type_pos + 2;
    while i < tokens.len() {
        match tokens[i] {
            "default" => {
                i += 1;
                if i < tokens.len() {
                    // For default, find the next parameter or end
                    let next_param = tokens[i..]
                        .iter()
                        .position(|&t| t == "min" || t == "max" || t == "var")
                        .map(|pos| pos + i)
                        .unwrap_or(tokens.len());

                    default = Some(tokens[i..next_param].join(" "));
                    i = next_param;
                }
            }
            "min" => {
                i += 1;
                if i < tokens.len() {
                    if let Ok(value) = tokens[i].parse::<i32>() {
                        min = Some(value);
                    }
                    i += 1;
                }
            }
            "max" => {
                i += 1;
                if i < tokens.len() {
                    if let Ok(value) = tokens[i].parse::<i32>() {
                        max = Some(value);
                    }
                    i += 1;
                }
            }
            "var" => {
                i += 1;
                if i < tokens.len() {
                    // For var, find the next "var" or end
                    let next_var = tokens[i..]
                        .iter()
                        .position(|&t| t == "var")
                        .map(|pos| pos + i)
                        .unwrap_or(tokens.len());

                    var.push(tokens[i..next_var].join(" "));
                    i = next_var;
                }
            }
            _ => {
                // Unknown parameter, skip
                i += 1;
            }
        }
    }

    Ok(EngineResponse::Option(OptionDefinition {
        name,
        option_type,
        default,
        min,
        max,
        var,
    }))
}
