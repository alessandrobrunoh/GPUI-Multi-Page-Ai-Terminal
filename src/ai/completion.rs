use crate::ai::{AiService, CompletionRequest};
use std::sync::Arc;

pub struct AiCompletion {
    ai_service: Arc<AiService>,
}

impl AiCompletion {
    pub fn new(ai_service: Arc<AiService>) -> Self {
        Self { ai_service }
    }
    
    pub async fn suggest_completion(&self, current_input: &str, context: Option<&str>) -> Option<String> {
        if current_input.is_empty() {
            return None;
        }
        
        let request = CompletionRequest {
            prompt: current_input.to_string(),
            context: context.map(|s| s.to_string()),
        };
        
        match self.ai_service.get_completion(request).await {
            Ok(response) => {
                if response.confidence > 0.5 {
                    Some(response.completion)
                } else {
                    None
                }
            }
            Err(_) => None,
        }
    }
    
    pub fn get_common_completions(&self, input: &str) -> Vec<String> {
        // Provide common shell command completions
        let commands = vec![
            "ls", "cd", "pwd", "mkdir", "rmdir", "rm", "cp", "mv", "cat", "grep",
            "find", "which", "man", "help", "clear", "exit", "history", "echo",
            "git", "npm", "cargo", "python", "node", "docker", "kubectl",
            "ai", // Our custom AI command
        ];
        
        commands
            .into_iter()
            .filter(|cmd| cmd.starts_with(input))
            .map(|cmd| cmd.to_string())
            .collect()
    }
}