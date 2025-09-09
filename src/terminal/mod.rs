pub mod emulator;
pub mod tab;

use std::sync::Arc;
use std::collections::HashMap;
use uuid::Uuid;
use crate::ai::AiService;

pub use emulator::TerminalEmulator;
pub use tab::TerminalTab;

pub struct TerminalManager {
    tabs: HashMap<Uuid, TerminalTab>,
    tab_order: Vec<Uuid>,
    active_tab_index: usize,
    ai_service: Arc<AiService>,
}

impl TerminalManager {
    pub fn new(ai_service: Arc<AiService>) -> Self {
        Self {
            tabs: HashMap::new(),
            tab_order: Vec::new(),
            active_tab_index: 0,
            ai_service,
        }
    }
    
    pub async fn create_new_tab(&mut self) -> Uuid {
        let id = Uuid::new_v4();
        let tab = TerminalTab::new(id, self.ai_service.clone()).await;
        
        self.tabs.insert(id, tab);
        self.tab_order.push(id);
        self.active_tab_index = self.tab_order.len() - 1;
        
        id
    }
    
    pub fn close_current_tab(&mut self) {
        if self.tab_order.is_empty() {
            return;
        }
        
        let id = self.tab_order[self.active_tab_index];
        self.tabs.remove(&id);
        self.tab_order.remove(self.active_tab_index);
        
        if self.tab_order.is_empty() {
            self.active_tab_index = 0;
        } else if self.active_tab_index >= self.tab_order.len() {
            self.active_tab_index = self.tab_order.len() - 1;
        }
    }
    
    pub fn next_tab(&mut self) {
        if !self.tab_order.is_empty() {
            self.active_tab_index = (self.active_tab_index + 1) % self.tab_order.len();
        }
    }
    
    pub fn get_tabs(&self) -> Vec<(Uuid, String, bool)> {
        self.tab_order
            .iter()
            .enumerate()
            .map(|(i, &id)| {
                let title = self.tabs.get(&id)
                    .map(|tab| tab.get_title())
                    .unwrap_or_else(|| "Unknown".to_string());
                let is_active = i == self.active_tab_index;
                (id, title, is_active)
            })
            .collect()
    }
    
    pub fn get_current_content(&self) -> Vec<String> {
        if let Some(&active_id) = self.tab_order.get(self.active_tab_index) {
            if let Some(tab) = self.tabs.get(&active_id) {
                return tab.get_content();
            }
        }
        vec!["No active terminal".to_string()]
    }
    
    pub async fn execute_command(&mut self, command: &str) {
        if let Some(&active_id) = self.tab_order.get(self.active_tab_index) {
            if let Some(tab) = self.tabs.get_mut(&active_id) {
                tab.execute_command(command).await;
            }
        }
    }
}