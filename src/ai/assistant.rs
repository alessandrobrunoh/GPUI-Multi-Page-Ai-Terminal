use crate::ai::{AiService, QuestionRequest};
use std::sync::Arc;

pub struct AiAssistant {
    ai_service: Arc<AiService>,
}

impl AiAssistant {
    pub fn new(ai_service: Arc<AiService>) -> Self {
        Self { ai_service }
    }
    
    pub async fn ask_question(&self, question: &str, context: Option<&str>) -> anyhow::Result<String> {
        let response = self.ai_service.ask_question(question).await?;
        
        Ok(format!(
            "AI Assistant: {}\n\nContext: {}",
            response,
            context.unwrap_or("No additional context")
        ))
    }
    
    pub fn get_help_topics(&self) -> Vec<(&'static str, &'static str)> {
        vec![
            ("commands", "Get help with terminal commands"),
            ("programming", "Ask programming questions"),
            ("system", "System administration help"),
            ("git", "Git version control help"),
            ("docker", "Docker containerization help"),
            ("general", "General questions"),
        ]
    }
    
    pub fn format_help(&self) -> String {
        let topics = self.get_help_topics();
        let mut help_text = String::from("AI Assistant Help Topics:\n\n");
        
        for (topic, description) in topics {
            help_text.push_str(&format!("  {} - {}\n", topic, description));
        }
        
        help_text.push_str("\nUsage: ai <your question>\n");
        help_text.push_str("Example: ai how do I list files in a directory?\n");
        
        help_text
    }
}