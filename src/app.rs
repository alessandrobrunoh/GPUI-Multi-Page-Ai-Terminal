use std::sync::Arc;
use uuid::Uuid;
use crate::terminal::TerminalManager;
use crate::ai::AiService;

pub struct App {
    terminal_manager: TerminalManager,
    ai_service: Arc<AiService>,
    current_input: String,
    show_completions: bool,
    completions: Vec<String>,
}

impl App {
    pub async fn new() -> Self {
        let ai_service = Arc::new(AiService::new());
        let mut terminal_manager = TerminalManager::new(ai_service.clone());
        
        // Create initial tab
        terminal_manager.create_new_tab().await;
        
        Self {
            terminal_manager,
            ai_service,
            current_input: String::new(),
            show_completions: false,
            completions: Vec::new(),
        }
    }
    
    pub fn get_tabs(&self) -> Vec<(Uuid, String, bool)> {
        self.terminal_manager.get_tabs()
    }
    
    pub fn get_current_terminal_content(&self) -> Vec<String> {
        self.terminal_manager.get_current_content()
    }
    
    pub fn get_current_input(&self) -> &str {
        &self.current_input
    }
    
    pub fn get_completions(&self) -> &[String] {
        &self.completions
    }
    
    pub fn show_completions(&self) -> bool {
        self.show_completions
    }
    
    pub async fn new_tab(&mut self) {
        self.terminal_manager.create_new_tab().await;
    }
    
    pub fn next_tab(&mut self) {
        self.terminal_manager.next_tab();
    }
    
    pub fn close_current_tab(&mut self) {
        self.terminal_manager.close_current_tab();
    }
    
    pub fn handle_char_input(&mut self, c: char) {
        self.current_input.push(c);
        self.hide_completions();
    }
    
    pub fn handle_backspace(&mut self) {
        self.current_input.pop();
        self.hide_completions();
    }
    
    pub async fn handle_tab_completion(&mut self) {
        if self.current_input.is_empty() {
            return;
        }
        
        self.completions = self.ai_service.get_completions(&self.current_input).await;
        self.show_completions = !self.completions.is_empty();
    }
    
    pub async fn execute_current_input(&mut self) {
        if self.current_input.is_empty() {
            return;
        }
        
        let input = self.current_input.clone();
        self.current_input.clear();
        self.hide_completions();
        
        self.terminal_manager.execute_command(&input).await;
    }
    
    fn hide_completions(&mut self) {
        self.show_completions = false;
        self.completions.clear();
    }
}