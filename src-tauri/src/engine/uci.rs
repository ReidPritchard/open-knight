// src-tauri/src/engine/uci.rs
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;
use tauri::{AppHandle, Emitter};

pub struct UciEngine {
    process: Child,
    options: Vec<UciOption>,
    name: String,
    author: String,
    _listener_thread: thread::JoinHandle<()>,
    app_handle: AppHandle,
    stdin: Arc<Mutex<std::process::ChildStdin>>,
}

#[derive(Debug)]
pub struct UciOption {
    name: String,
    option_type: String,
    default: Option<String>,
    min: Option<String>,
    max: Option<String>,
    var: Vec<String>,
}

impl UciEngine {
    pub fn new(path: PathBuf, app_handle: AppHandle) -> Result<Self, String> {
        // Start the engine process
        let mut process = Command::new(path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| format!("Failed to start engine process: {}", e))?;

        let stdout = BufReader::new(process.stdout.take().unwrap());
        let stdin = process.stdin.take().unwrap();
        let stdin = Arc::new(Mutex::new(stdin));

        // Setup communication channels
        let (tx, rx) = mpsc::channel();

        // Setup listener thread to process engine output
        let app_handle_clone = app_handle.clone();
        let listener_thread = thread::spawn(move || {
            for line in stdout.lines() {
                if let Ok(line) = line {
                    // Parse UCI output and send events to the app
                    tx.send(line.clone()).unwrap_or_default();

                    

                    // Emit events to the frontend
                    app_handle_clone
                        .emit("engine-output", line)
                        .unwrap_or_else(|e| eprintln!("Failed to emit engine output: {}", e));
                }
            }
        });

        // Initialize the engine
        Self::send_command(&stdin, "uci")?;

        // Wait for and process "uciok" response
        let mut name = String::new();
        let mut author = String::new();
        let mut options = Vec::new();

        // Process initial UCI responses with timeout
        let timeout = Duration::from_secs(5);
        let start = std::time::Instant::now();

        while start.elapsed() < timeout {
            if let Ok(line) = rx.recv_timeout(Duration::from_millis(100)) {
                if line.starts_with("id name") {
                    name = line.trim_start_matches("id name").trim().to_string();
                } else if line.starts_with("id author") {
                    author = line.trim_start_matches("id author").trim().to_string();
                } else if line.starts_with("option") {
                    // Parse option
                    let option = Self::parse_option(&line);
                    options.push(option);
                } else if line == "uciok" {
                    // UCI initialization completed

                    println!("Engine initialized: {}", name);
                    println!("Author: {}", author);
                    println!("Options: {:?}", options);

                    break;
                }
            }
        }

        if name.is_empty() {
            return Err("Engine did not provide a name".to_string());
        }

        Ok(Self {
            process,
            options,
            name,
            author,
            _listener_thread: listener_thread,
            app_handle,
            stdin,
        })
    }

    fn parse_option(line: &str) -> UciOption {
        // Basic option parsing
        let mut name = String::new();
        let mut option_type = String::new();
        let mut default = None;
        let mut min = None;
        let mut max = None;
        let mut var = Vec::new();

        // Parse the option line
        let parts: Vec<&str> = line.split_whitespace().collect();
        for i in 0..parts.len() {
            match parts[i] {
                "name" if i + 1 < parts.len() => {
                    let mut j = i + 1;
                    while j < parts.len() && parts[j] != "type" {
                        name.push_str(parts[j]);
                        name.push(' ');
                        j += 1;
                    }
                    name = name.trim().to_string();
                }
                "type" if i + 1 < parts.len() => {
                    option_type = parts[i + 1].to_string();
                }
                "default" if i + 1 < parts.len() => {
                    default = Some(parts[i + 1].to_string());
                }
                "min" if i + 1 < parts.len() => {
                    min = Some(parts[i + 1].to_string());
                }
                "max" if i + 1 < parts.len() => {
                    max = Some(parts[i + 1].to_string());
                }
                "var" if i + 1 < parts.len() => {
                    var.push(parts[i + 1].to_string());
                }
                _ => {}
            }
        }

        UciOption {
            name,
            option_type,
            default,
            min,
            max,
            var,
        }
    }

    pub fn set_option(&mut self, name: &str, value: &str) -> Result<(), String> {
        let command = format!("setoption name {} value {}", name, value);
        Self::send_command(&self.stdin, &command)?;
        Ok(())
    }

    pub fn position_from_fen(&mut self, fen: &str) -> Result<(), String> {
        println!("Setting position from FEN: {}", fen);
        let command = format!("position fen {}", fen);
        Self::send_command(&self.stdin, &command)?;
        Ok(())
    }

    pub fn position_from_moves(&mut self, moves: &[&str]) -> Result<(), String> {
        let moves_str = moves.join(" ");
        let command = format!("position startpos moves {}", moves_str);
        Self::send_command(&self.stdin, &command)?;
        Ok(())
    }

    pub fn go_infinite(&mut self) -> Result<(), String> {
        Self::send_command(&self.stdin, "go infinite")?;
        Ok(())
    }

    pub fn go_depth(&mut self, depth: usize) -> Result<(), String> {
        let command = format!("go depth {}", depth);
        Self::send_command(&self.stdin, &command)?;
        Ok(())
    }

    pub fn go_movetime(&mut self, time_ms: usize) -> Result<(), String> {
        let command = format!("go movetime {}", time_ms);
        Self::send_command(&self.stdin, &command)?;
        Ok(())
    }

    pub fn stop(&mut self) -> Result<(), String> {
        Self::send_command(&self.stdin, "stop")?;
        Ok(())
    }

    pub fn quit(&mut self) -> Result<(), String> {
        Self::send_command(&self.stdin, "quit")?;
        Ok(())
    }

    fn send_command(
        stdin: &Arc<Mutex<std::process::ChildStdin>>,
        command: &str,
    ) -> Result<(), String> {
        let mut stdin = stdin
            .lock()
            .map_err(|_| "Failed to lock stdin".to_string())?;
        writeln!(stdin, "{}", command).map_err(|e| format!("Failed to send command: {}", e))?;
        Ok(())
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_author(&self) -> &str {
        &self.author
    }

    pub fn get_options(&self) -> &[UciOption] {
        &self.options
    }
}

impl Drop for UciEngine {
    fn drop(&mut self) {
        // Make sure to stop and quit the engine when the struct is dropped
        let _ = self.stop();
        let _ = self.quit();
    }
}
