use uuid::Uuid;
use std::sync::Arc;
use crate::terminal::TerminalEmulator;
use crate::ai::AiService;

pub struct TerminalTab {
    id: Uuid,
    title: String,
    emulator: TerminalEmulator,
}

impl TerminalTab {
    pub async fn new(id: Uuid, ai_service: Arc<AiService>) -> Self {
        let emulator = TerminalEmulator::new(ai_service).await;
        
        Self {
            id,
            title: format!("Terminal-{}", &id.to_string()[..8]),
            emulator,
        }
    }
    
    pub fn get_id(&self) -> Uuid {
        self.id
    }
    
    pub fn get_title(&self) -> String {
        self.title.clone()
    }
    
    pub fn get_content(&self) -> Vec<String> {
        self.emulator.get_content()
    }
    
    pub async fn execute_command(&mut self, command: &str) {
        self.emulator.execute_command(command).await;
    }
}