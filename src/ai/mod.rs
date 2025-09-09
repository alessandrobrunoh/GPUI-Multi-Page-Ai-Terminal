pub mod completion;
pub mod assistant;

use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct AiService {
    client: Client,
    api_key: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct CompletionRequest {
    pub prompt: String,
    pub context: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct CompletionResponse {
    pub completion: String,
    pub confidence: f32,
}

#[derive(Serialize, Deserialize)]
pub struct QuestionRequest {
    pub question: String,
    pub context: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct QuestionResponse {
    pub answer: String,
    pub sources: Vec<String>,
}

impl AiService {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            api_key: std::env::var("AI_API_KEY").ok(),
        }
    }
    
    pub async fn get_completion(&self, request: CompletionRequest) -> anyhow::Result<CompletionResponse> {
        // For now, return a mock completion
        // In a real implementation, this would call an AI API like OpenAI or Claude
        Ok(CompletionResponse {
            completion: format!("Suggested completion for: {}", request.prompt),
            confidence: 0.85,
        })
    }
    
    pub async fn ask_question(&self, question: &str) -> anyhow::Result<String> {
        // Mock AI responses for demonstration
        let response = match question.to_lowercase().as_str() {
            q if q.contains("list files") || q.contains("ls") => {
                "To list files in a directory, use the 'ls' command. Options include:\n\
                 - ls -l (long format with details)\n\
                 - ls -la (include hidden files)\n\
                 - ls -lh (human readable file sizes)"
            }
            q if q.contains("git") => {
                "Git is a distributed version control system. Common commands:\n\
                 - git status (check repository status)\n\
                 - git add <file> (stage changes)\n\
                 - git commit -m 'message' (commit changes)\n\
                 - git push (push to remote repository)"
            }
            q if q.contains("docker") => {
                "Docker is a containerization platform. Basic commands:\n\
                 - docker build -t <name> . (build image)\n\
                 - docker run <image> (run container)\n\
                 - docker ps (list running containers)\n\
                 - docker stop <container> (stop container)"
            }
            q if q.contains("rust") || q.contains("cargo") => {
                "Rust is a systems programming language. Cargo commands:\n\
                 - cargo new <project> (create new project)\n\
                 - cargo build (build project)\n\
                 - cargo run (run project)\n\
                 - cargo test (run tests)"
            }
            _ => {
                "I'm an AI assistant that can help with terminal commands, programming, \
                 and system administration. Try asking about specific tools like git, \
                 docker, rust, or common shell commands!"
            }
        };
        
        Ok(response.to_string())
    }
    
    pub async fn get_completions(&self, input: &str) -> Vec<String> {
        // Provide common shell command completions
        let commands = vec![
            "ls", "cd", "pwd", "mkdir", "rmdir", "rm", "cp", "mv", "cat", "grep",
            "find", "which", "man", "help", "clear", "exit", "history", "echo",
            "git", "npm", "cargo", "python", "node", "docker", "kubectl",
            "ai", "date", "whoami",
        ];
        
        commands
            .into_iter()
            .filter(|cmd| cmd.starts_with(input))
            .map(|cmd| cmd.to_string())
            .take(5) // Limit to 5 suggestions
            .collect()
    }
    
    pub fn is_configured(&self) -> bool {
        self.api_key.is_some()
    }
}