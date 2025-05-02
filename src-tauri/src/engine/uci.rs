use crate::engine::engine::{EngineConfig, EngineOption};
use crate::parse::uci::{parse_engine_response, EngineResponse, IdInfo};
use serde::{Deserialize, Serialize};
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;
use tauri::{AppHandle, Emitter};

use super::engine::EngineError;

/// A UCI engine instance/state
pub struct UciEngine {
    /// The engine configuration
    engine: EngineConfig,
    /// The engine process
    process: Child,
    /// The listener thread
    _listener_thread: thread::JoinHandle<()>,
    /// The stdin handle
    stdin: Arc<Mutex<std::process::ChildStdin>>,
    /// The state of the engine
    state: Arc<Mutex<UciEngineState>>,
}

/// The state of the engine
/// Mostly needed for full-game analysis
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UciEngineState {
    /// The current position of the engine in FEN format
    pub current_position_fen: String,
    /// The current position of the engine in move notation (list of moves)
    pub current_position_moves: Vec<String>,
    /// Most recent engine score for the current position
    pub current_position_score: Option<f64>,
    /// The top N lines of the engine's output for the current position
    pub current_position_lines: Vec<String>,
    /// Most recent engine output
    pub last_engine_output: String,
    /// A set of all the engine's output for the current position
    pub all_engine_output: Vec<String>,
}

impl UciEngine {
    /// Creates a new UCI engine instance
    ///
    /// # Parameters
    /// * `path` - Path to the engine executable
    /// * `app_handle` - Tauri app handle for event emission
    /// * `timeout_secs` - Optional timeout for engine initialization (default: 10 seconds)
    pub fn new(
        path: PathBuf,
        app_handle: AppHandle,
        timeout_secs: Option<u64>,
    ) -> Result<Self, EngineError> {
        let engine_path = path.to_string_lossy().to_string();
        let timeout = Duration::from_secs(timeout_secs.unwrap_or(10));

        // Create state
        let state = Arc::new(Mutex::new(UciEngineState::default()));

        // Start the engine process
        let mut process = Command::new(&path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| {
                EngineError::EngineProcessError(format!(
                    "Failed to start engine at {}: {}",
                    engine_path, e
                ))
            })?;

        // Get stdin and stdout handles
        let stdout = process.stdout.take().ok_or_else(|| {
            EngineError::EngineProcessError("Failed to capture engine stdout".to_string())
        })?;
        let stdout = BufReader::new(stdout);

        let stdin = process.stdin.take().ok_or_else(|| {
            EngineError::EngineProcessError("Failed to capture engine stdin".to_string())
        })?;
        let stdin = Arc::new(Mutex::new(stdin));

        // Setup communication channels
        let (init_tx, init_rx) = mpsc::channel();

        // Create engine configuration
        let engine_config = Arc::new(Mutex::new(EngineConfig::new(engine_path.clone())));
        let engine_config_for_thread = Arc::clone(&engine_config);

        // Setup listener thread
        let app_handle_clone = app_handle.clone();
        let state_clone = Arc::clone(&state);

        let listener_thread = thread::spawn(move || {
            let mut received_uciok = false;
            for line in stdout.lines() {
                match line {
                    Ok(line) => {
                        if !received_uciok {
                            // Process initialization responses
                            received_uciok =
                                Self::process_init_line(&line, &engine_config_for_thread, &init_tx);

                            if received_uciok {
                                // Just received uciok
                                // the options are now all set
                                // TODO: emit options to frontend (event: 'engine-options')
                                println!(
                                    "Options: {:?}",
                                    engine_config_for_thread.lock().unwrap().options
                                );
                            }
                        } else {
                            // Parse and handle engine response
                            Self::handle_uci_output(
                                &line,
                                &engine_config_for_thread,
                                &app_handle_clone,
                                &state_clone,
                            );
                        }
                    }
                    Err(e) => {
                        eprintln!("Error reading from engine: {}", e);
                        break;
                    }
                }
            }
            eprintln!("Engine stdout closed");

            // TODO: Check process status
        });

        // Initialize engine with UCI protocol
        Self::send_command(&stdin, "uci")?;

        // Wait for initialization with timeout
        if init_rx.recv_timeout(timeout).is_err() {
            // If the engine doesn't respond in time, kill the process
            let _ = process.kill();
            return Err(EngineError::EngineProcessError(format!(
                "Engine initialization timed out after {} seconds",
                timeout_secs.unwrap_or(10)
            )));
        }

        // Get the final engine configuration
        let engine_config = match engine_config.lock() {
            Ok(config) => config,
            Err(_) => {
                return Err(EngineError::EngineProcessError(
                    "Failed to lock engine config".to_string(),
                ))
            }
        };

        if engine_config.name.is_empty() {
            return Err(EngineError::EngineProcessError(
                "Engine did not provide a name".to_string(),
            ));
        }

        // Create the engine instance
        Ok(Self {
            process,
            engine: EngineConfig {
                name: engine_config.name.clone(),
                author: engine_config.author.clone(),
                path: engine_config.path.clone(),
                options: engine_config.options.clone(),
            },
            _listener_thread: listener_thread,
            stdin,
            state,
        })
    }

    /// Process initialization lines and signal when complete
    fn process_init_line(
        line: &str,
        engine_config: &Arc<Mutex<EngineConfig>>,
        init_tx: &mpsc::Sender<()>,
    ) -> bool {
        let parsed_line = match parse_engine_response(line) {
            Ok(response) => response,
            Err(e) => {
                eprintln!("Error parsing engine response: {}", e);
                return false;
            }
        };

        let mut config = match engine_config.lock() {
            Ok(guard) => guard,
            Err(e) => {
                eprintln!("Failed to lock engine config: {}", e);
                return false;
            }
        };

        match parsed_line {
            EngineResponse::Id(id) => match id {
                IdInfo::Name(name) => {
                    config.name = name;
                    false
                }
                IdInfo::Author(author) => {
                    config.author = author;
                    false
                }
            },
            EngineResponse::UciOk => {
                // Signal that the engine has received the uciok command
                let _ = init_tx.send(());
                true
            }
            EngineResponse::Option(option) => {
                // TODO: Add option to engine config
                // println!("Option: {:?}", serde_json::to_string(&option).unwrap());
                config.options.push(EngineOption {
                    name: option.name,
                    option_type: serde_json::to_string(&option.option_type).unwrap(),
                    value: option.default.clone(),
                    default: option.default,
                    min: option.min.map(|min| min.to_string()),
                    max: option.max.map(|max| max.to_string()),
                    description: None,
                    var: Some(option.var),
                });

                false
            }
            _ => false,
        }
    }

    /// Parse and handle an engine response
    fn handle_uci_output(
        line: &str,
        engine_config: &Arc<Mutex<EngineConfig>>,
        app_handle: &AppHandle,
        state: &Arc<Mutex<UciEngineState>>,
    ) {
        // Try to parse the line
        let parse_result = parse_engine_response(line);

        // Get engine name
        let engine_name = match engine_config.lock() {
            Ok(config) => config.name.clone(),
            Err(_) => String::new(),
        };

        // Get the state
        let mut state = state.lock().unwrap();

        // Determine what to do with the line
        match parse_result {
            Ok(response) => {
                // Update state and handle response
                match response {
                    EngineResponse::Info(info) => {
                        // Update engine state with latest info
                        state.last_engine_output = line.to_string();
                        state.all_engine_output.push(line.to_string());

                        // Update score if available
                        if let Some(score) = info.score {
                            match score {
                                crate::parse::uci::Score::Centipawns { value, .. } => {
                                    state.current_position_score = Some(value as f64 / 100.0);
                                }
                                crate::parse::uci::Score::Mate(mate) => {
                                    // Convert mate score to a large centipawn value
                                    state.current_position_score =
                                        Some(if mate > 0 { 1000.0 } else { -1000.0 });
                                }
                            }
                        }

                        // Update current lines if available
                        if let Some(pv) = info.pv {
                            state.current_position_lines = pv;
                        }
                    }
                    EngineResponse::BestMove { .. } => {
                        // Analysis is complete, emit final state
                        let engine_message = serde_json::to_string(&*state).unwrap();
                        if let Err(e) = app_handle.emit("engine-analysis-complete", engine_message)
                        {
                            eprintln!("Failed to emit analysis complete: {}", e);
                        }
                    }
                    EngineResponse::ReadyOk => {
                        // Engine is ready for new commands
                        if let Err(e) = app_handle.emit("engine-ready", "") {
                            eprintln!("Failed to emit engine ready: {}", e);
                        }
                    }
                    _ => {} // Ignore other response types
                }
            }
            Err(e) => {
                eprintln!("Error parsing engine response: {}", e);
            }
        }

        // Emit raw engine output for debugging
        if let Err(e) = app_handle.emit("engine-output", line) {
            eprintln!("Failed to emit engine output: {}", e);
        }
    }

    // Send a command to the engine
    fn send_command(
        stdin: &Arc<Mutex<std::process::ChildStdin>>,
        command: &str,
    ) -> Result<(), EngineError> {
        let mut stdin = stdin
            .lock()
            .map_err(|_| EngineError::EngineProcessError("Failed to lock stdin".to_string()))?;

        writeln!(stdin, "{}", command).map_err(|e| {
            EngineError::EngineProcessError(format!("Failed to send command: {}", e))
        })?;

        Ok(())
    }

    // Public interface methods

    pub fn set_option(&mut self, name: &str, value: &str) -> Result<(), EngineError> {
        let command = format!("setoption name {} value {}", name, value);
        Self::send_command(&self.stdin, &command)
    }

    pub fn position_from_fen(&mut self, fen: &str) -> Result<(), EngineError> {
        println!("Setting position from FEN: {}", fen);
        // Reset the engine state for the new position
        {
            let mut state = self.state.lock().unwrap();
            state.current_position_fen = fen.to_string();
            state.current_position_moves.clear();
            state.current_position_score = None;
            state.current_position_lines.clear();
            state.last_engine_output.clear();
            state.all_engine_output.clear();
        }

        // Send the position command
        let command = format!("position fen {}", fen);
        Self::send_command(&self.stdin, &command)?;

        // Ensure the engine is ready for the new position
        Self::send_command(&self.stdin, "isready")
    }

    pub fn position_from_moves(&mut self, moves: &[&str]) -> Result<(), EngineError> {
        let moves_str = moves.join(" ");
        let command = format!("position startpos moves {}", moves_str);
        Self::send_command(&self.stdin, &command)
    }

    pub fn go_infinite(&mut self) -> Result<(), EngineError> {
        Self::send_command(&self.stdin, "go infinite")
    }

    pub fn go_depth(&mut self, depth: usize) -> Result<(), EngineError> {
        println!("Going depth: {}", depth);
        let command = format!("go depth {}", depth);
        Self::send_command(&self.stdin, &command)
    }

    pub fn go_movetime(&mut self, time_ms: usize) -> Result<(), EngineError> {
        println!("Going movetime: {}", time_ms);
        let command = format!("go movetime {}", time_ms);
        Self::send_command(&self.stdin, &command)
    }

    pub fn stop(&mut self) -> Result<(), EngineError> {
        println!("Stopping engine");
        Self::send_command(&self.stdin, "stop")
    }

    pub fn quit(&mut self) -> Result<(), EngineError> {
        println!("Quitting engine");
        Self::send_command(&self.stdin, "quit")
    }

    // Getters

    pub fn get_name(&self) -> &str {
        &self.engine.name
    }

    pub fn get_author(&self) -> &str {
        &self.engine.author
    }

    pub fn get_options(&self) -> &[EngineOption] {
        &self.engine.options
    }
}

impl Drop for UciEngine {
    fn drop(&mut self) {
        // Make sure to stop and quit the engine when the struct is dropped
        let _ = self.stop();
        let _ = self.quit();
    }
}
