use eframe::egui;
use anyhow::Result;
use uuid::Uuid;
use std::time::Instant;

mod terminal;
mod ai;

use terminal::TerminalSession;
use ai::GeminiClient;

fn main() -> Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_title("GPUI Multi-Page AI Terminal")
            .with_min_inner_size([800.0, 600.0]),
        ..Default::default()
    };

    let result = eframe::run_native(
        "GPUI Multi-Page AI Terminal",
        options,
        Box::new(|_cc| Ok(Box::new(TerminalApp::new()))),
    );
    
    match result {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("Failed to run application: {}", e);
            std::process::exit(1);
        }
    }
}

#[derive(Clone)]
pub enum AppMode {
    Terminal,
    AiAsist,
}

pub struct TerminalTab {
    id: Uuid,
    title: String,
    content: String,
    input: String,
    mode: AppMode,
    history: Vec<String>,
    ai_conversation: Vec<AiMessage>,
    terminal_session: Option<TerminalSession>,
    last_update: Instant,
}

#[derive(Clone)]
pub struct AiMessage {
    pub role: String, // "user" or "assistant"
    pub content: String,
}

pub struct TerminalApp {
    tabs: Vec<TerminalTab>,
    active_tab: usize,
    show_ai_input: bool,
    gemini_client: Option<GeminiClient>,
    rt: tokio::runtime::Runtime,
}

impl TerminalApp {
    fn new() -> Self {
        let rt = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");
        
        let gemini_client = std::env::var("GEMINI_API_KEY")
            .ok()
            .filter(|key| !key.is_empty())
            .map(GeminiClient::new);
        
        let mut app = Self {
            tabs: Vec::new(),
            active_tab: 0,
            show_ai_input: false,
            gemini_client,
            rt,
        };
        
        // Create initial tab
        app.add_new_tab();
        app
    }

    fn add_new_tab(&mut self) {
        let terminal_session = TerminalSession::new().ok();
        
        let tab = TerminalTab {
            id: Uuid::new_v4(),
            title: format!("Terminal {}", self.tabs.len() + 1),
            content: if terminal_session.is_some() {
                "Real terminal session started...\n".to_string()
            } else {
                "Welcome to GPUI Multi-Page AI Terminal (Demo Mode)\n$ ".to_string()
            },
            input: String::new(),
            mode: AppMode::Terminal,
            history: Vec::new(),
            ai_conversation: Vec::new(),
            terminal_session,
            last_update: Instant::now(),
        };
        self.tabs.push(tab);
        self.active_tab = self.tabs.len() - 1;
    }

    fn close_tab(&mut self, index: usize) {
        if self.tabs.len() > 1 && index < self.tabs.len() {
            self.tabs.remove(index);
            if self.active_tab >= self.tabs.len() {
                self.active_tab = self.tabs.len() - 1;
            }
        }
    }

    fn switch_mode(&mut self) {
        if let Some(tab) = self.tabs.get_mut(self.active_tab) {
            tab.mode = match tab.mode {
                AppMode::Terminal => AppMode::AiAsist,
                AppMode::AiAsist => AppMode::Terminal,
            };
        }
    }

    fn execute_command(&mut self, command: String) {
        if let Some(tab) = self.tabs.get_mut(self.active_tab) {
            match tab.mode {
                AppMode::Terminal => {
                    if let Some(ref session) = tab.terminal_session {
                        // Real terminal session
                        if let Err(e) = session.write_input(&format!("{}\n", command)) {
                            tab.content.push_str(&format!("Error writing to terminal: {}\n", e));
                        }
                    } else {
                        // Fallback demo mode
                        tab.content.push_str(&format!("{}\n", command));
                        tab.history.push(command.clone());
                        
                        match command.trim() {
                            "clear" => tab.content.clear(),
                            "pwd" => tab.content.push_str("/home/user\n"),
                            "ls" => tab.content.push_str("Documents  Downloads  Pictures  Videos\n"),
                            cmd if cmd.starts_with("echo ") => {
                                let text = &cmd[5..];
                                tab.content.push_str(&format!("{}\n", text));
                            }
                            _ => {
                                tab.content.push_str(&format!("Command '{}' not found. This is a demo terminal.\n", command));
                            }
                        }
                        tab.content.push_str("$ ");
                    }
                }
                AppMode::AiAsist => {
                    // Add user message
                    tab.ai_conversation.push(AiMessage {
                        role: "user".to_string(),
                        content: command.clone(),
                    });
                    
                    // Get AI response
                    if let Some(ref client) = self.gemini_client {
                        let client = client.clone();
                        let context = tab.ai_conversation.clone();
                        let question = command.clone();
                        
                        // Use the runtime to execute the async call
                        let response = self.rt.block_on(async {
                            client.ask_question(&question, &context).await
                        });
                        
                        let ai_response = match response {
                            Ok(response) => response,
                            Err(e) => format!("AI Error: {}", e),
                        };
                        
                        tab.ai_conversation.push(AiMessage {
                            role: "assistant".to_string(),
                            content: ai_response,
                        });
                    } else {
                        let ai_response = "AI Assistant: Please set GEMINI_API_KEY environment variable to enable AI features.".to_string();
                        tab.ai_conversation.push(AiMessage {
                            role: "assistant".to_string(),
                            content: ai_response,
                        });
                    }
                }
            }
        }
    }
    
    fn update_terminal_output(&mut self) {
        for tab in &mut self.tabs {
            if let Some(ref session) = tab.terminal_session {
                // Check for new output every frame
                while let Some(output) = session.read_output() {
                    tab.content.push_str(&output);
                }
            }
        }
    }
}

impl eframe::App for TerminalApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Update terminal output for all tabs
        self.update_terminal_output();
        
        // Set dark theme similar to Zed
        ctx.set_style({
            let mut style = (*ctx.style()).clone();
            style.visuals.dark_mode = true;
            style.visuals.window_fill = egui::Color32::from_rgb(24, 24, 27); // Zed's dark background
            style.visuals.panel_fill = egui::Color32::from_rgb(32, 32, 35);
            style.visuals.extreme_bg_color = egui::Color32::from_rgb(16, 16, 18);
            style.visuals.faint_bg_color = egui::Color32::from_rgb(40, 40, 43);
            style.visuals.selection.bg_fill = egui::Color32::from_rgb(58, 73, 102);
            style
        });
        
        // Handle keyboard shortcuts
        ctx.input(|i| {
            if i.modifiers.command && i.key_pressed(egui::Key::I) {
                self.switch_mode();
            }
            if i.modifiers.command && i.key_pressed(egui::Key::T) {
                self.add_new_tab();
            }
        });

        // Top menu bar
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            ui.style_mut().visuals.button_frame = true;
            ui.style_mut().spacing.item_spacing = egui::vec2(8.0, 4.0);
            
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("🆕 New Tab (Cmd+T)").clicked() {
                        self.add_new_tab();
                        ui.close_menu();
                    }
                    if ui.button("❌ Close Tab").clicked() && self.tabs.len() > 1 {
                        self.close_tab(self.active_tab);
                        ui.close_menu();
                    }
                });
                
                ui.menu_button("View", |ui| {
                    if ui.button("🔄 Switch Mode (Cmd+I)").clicked() {
                        self.switch_mode();
                        ui.close_menu();
                    }
                });
                
                ui.separator();
                
                // Current mode indicator with better styling
                if let Some(tab) = self.tabs.get(self.active_tab) {
                    let (mode_text, mode_color) = match tab.mode {
                        AppMode::Terminal => ("🖥️ Terminal Mode", egui::Color32::from_rgb(100, 200, 100)),
                        AppMode::AiAsist => ("🤖 AI Assistant Mode", egui::Color32::from_rgb(100, 150, 255)),
                    };
                    ui.colored_label(mode_color, mode_text);
                }
            });
        });

        // Tab bar
        egui::TopBottomPanel::top("tab_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                for (i, tab) in self.tabs.iter().enumerate() {
                    let selected = i == self.active_tab;
                    if ui.selectable_label(selected, &tab.title).clicked() {
                        self.active_tab = i;
                    }
                    
                    // Close button for tab
                    if self.tabs.len() > 1 {
                        if ui.small_button("✕").clicked() {
                            self.close_tab(i);
                            return;
                        }
                    }
                }
                
                if ui.button("+").clicked() {
                    self.add_new_tab();
                }
            });
        });

        // Main terminal area
        egui::CentralPanel::default().show(ctx, |ui| {
            // Get current tab data to avoid borrowing issues
            let current_tab_index = self.active_tab;
            let (tab_mode, tab_content, tab_input, tab_ai_conversation) = {
                if let Some(tab) = self.tabs.get(current_tab_index) {
                    (tab.mode.clone(), tab.content.clone(), tab.input.clone(), tab.ai_conversation.clone())
                } else {
                    return;
                }
            };

            ui.vertical(|ui| {
                // Terminal/AI content area
                egui::ScrollArea::vertical()
                    .max_height(ui.available_height() - 60.0)
                    .stick_to_bottom(true)
                    .show(ui, |ui| {
                        match tab_mode {
                            AppMode::Terminal => {
                                let mut content = tab_content;
                                ui.add(
                                    egui::TextEdit::multiline(&mut content)
                                        .font(egui::TextStyle::Monospace)
                                        .code_editor()
                                        .desired_width(f32::INFINITY)
                                        .interactive(false)
                                        .text_color(egui::Color32::from_rgb(200, 255, 200)) // Terminal green
                                );
                            }
                            AppMode::AiAsist => {
                                ui.heading("🤖 AI Assistant");
                                ui.separator();
                                
                                for message in &tab_ai_conversation {
                                    let (color, prefix) = if message.role == "user" {
                                        (egui::Color32::from_rgb(100, 150, 255), "👤 You:")
                                    } else {
                                        (egui::Color32::from_rgb(100, 255, 150), "🤖 Assistant:")
                                    };
                                    
                                    ui.horizontal(|ui| {
                                        ui.colored_label(color, prefix);
                                    });
                                    
                                    ui.add_space(4.0);
                                    
                                    // Message content with word wrapping
                                    ui.add(
                                        egui::Label::new(&message.content)
                                            .wrap()
                                    );
                                    
                                    ui.add_space(8.0);
                                    ui.separator();
                                }
                            }
                        }
                    });

                // Input area
                ui.separator();
                ui.horizontal(|ui| {
                    let input_label = match tab_mode {
                        AppMode::Terminal => "$ ",
                        AppMode::AiAsist => "Ask AI: ",
                    };
                    ui.label(input_label);
                    
                    // Get mutable reference to current tab's input
                    let mut input_text = tab_input;
                    let response = ui.add(
                        egui::TextEdit::singleline(&mut input_text)
                            .font(egui::TextStyle::Monospace)
                            .desired_width(f32::INFINITY)
                    );
                    
                    // Update the tab's input field
                    if let Some(tab) = self.tabs.get_mut(current_tab_index) {
                        tab.input = input_text.clone();
                    }
                    
                    let mut should_execute = false;
                    if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                        should_execute = true;
                        response.request_focus();
                    }
                    
                    if ui.button("Send").clicked() {
                        should_execute = true;
                    }
                    
                    if should_execute && !input_text.trim().is_empty() {
                        if let Some(tab) = self.tabs.get_mut(current_tab_index) {
                            tab.input.clear();
                        }
                        self.execute_command(input_text);
                    }
                });
                
                // Help text with better styling
                ui.separator();
                ui.horizontal(|ui| {
                    ui.small(match tab_mode {
                        AppMode::Terminal => "💡 Terminal Mode - Type commands and press Enter. Try: pwd, ls, echo hello, clear",
                        AppMode::AiAsist => "💡 AI Assistant Mode - Ask questions and get AI-powered responses",
                    });
                });
                ui.horizontal(|ui| {
                    ui.small("🔧 Press Cmd+I to switch modes, Cmd+T for new tab");
                });
            });
        });
    }
}
