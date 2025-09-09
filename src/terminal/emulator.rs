use std::collections::VecDeque;
use crate::ai::AiService;
use std::sync::Arc;

pub struct TerminalEmulator {
    content: VecDeque<String>,
    history: Vec<String>,
    ai_service: Arc<AiService>,
    max_lines: usize,
}

impl TerminalEmulator {
    pub async fn new(ai_service: Arc<AiService>) -> Self {
        let mut emulator = Self {
            content: VecDeque::new(),
            history: Vec::new(),
            ai_service,
            max_lines: 1000,
        };
        
        emulator.add_line("Welcome to GPUI AI Terminal v0.1.0".to_string());
        emulator.add_line("Features: AI Auto Completion, AI Assistant, Multi-Tab Support".to_string());
        emulator.add_line("Commands: help, clear, ai <question>, history".to_string());
        emulator.add_line("Shortcuts: Ctrl+T (new tab), Ctrl+W (close tab), Ctrl+Tab (next tab)".to_string());
        emulator.add_line("".to_string());
        
        emulator
    }
    
    pub fn add_line(&mut self, line: String) {
        self.content.push_back(line);
        if self.content.len() > self.max_lines {
            self.content.pop_front();
        }
    }
    
    pub fn get_content(&self) -> Vec<String> {
        self.content.iter().cloned().collect()
    }
    
    pub async fn execute_command(&mut self, command: &str) {
        let trimmed = command.trim();
        
        // Add command to history
        if !trimmed.is_empty() {
            self.history.push(trimmed.to_string());
            self.add_line(format!("$ {}", trimmed));
        } else {
            self.add_line("$ ".to_string());
            return;
        }
        
        match trimmed {
            "help" => {
                self.add_line("Available commands:".to_string());
                self.add_line("  help          - Show this help".to_string());
                self.add_line("  clear         - Clear terminal".to_string());
                self.add_line("  history       - Show command history".to_string());
                self.add_line("  echo <text>   - Echo text".to_string());
                self.add_line("  ai <question> - Ask AI assistant a question".to_string());
                self.add_line("  ls            - List files".to_string());
                self.add_line("  pwd           - Print working directory".to_string());
                self.add_line("  whoami        - Show current user".to_string());
                self.add_line("  date          - Show current date and time".to_string());
            }
            "clear" => {
                self.content.clear();
            }
            "history" => {
                self.add_line("Command history:".to_string());
                let history_clone = self.history.clone();
                for (i, cmd) in history_clone.iter().enumerate() {
                    self.add_line(format!("  {} {}", i + 1, cmd));
                }
            }
            cmd if cmd.starts_with("echo ") => {
                let text = &cmd[5..];
                self.add_line(text.to_string());
            }
            cmd if cmd.starts_with("ai ") => {
                let question = &cmd[3..];
                self.add_line("AI Assistant: Processing your question...".to_string());
                
                match self.ai_service.ask_question(question).await {
                    Ok(response) => {
                        for line in response.lines() {
                            self.add_line(format!("AI: {}", line));
                        }
                    }
                    Err(e) => {
                        self.add_line(format!("AI Error: {}", e));
                    }
                }
            }
            "ls" => {
                self.add_line("Cargo.toml".to_string());
                self.add_line("README.md".to_string());
                self.add_line("src/".to_string());
                self.add_line("target/".to_string());
            }
            "pwd" => {
                self.add_line("/home/user/gpui-terminal".to_string());
            }
            "whoami" => {
                self.add_line("terminal-user".to_string());
            }
            "date" => {
                let now = chrono::Utc::now();
                self.add_line(now.format("%Y-%m-%d %H:%M:%S UTC").to_string());
            }
            _ => {
                self.add_line(format!("Command not found: {}", trimmed));
                self.add_line("Type 'help' for available commands".to_string());
            }
        }
    }
}