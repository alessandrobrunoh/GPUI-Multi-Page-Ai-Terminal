use eframe::egui;
use anyhow::Result;
use uuid::Uuid;

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
    gemini_api_key: String,
}

impl TerminalApp {
    fn new() -> Self {
        let mut app = Self {
            tabs: Vec::new(),
            active_tab: 0,
            show_ai_input: false,
            gemini_api_key: std::env::var("GEMINI_API_KEY").unwrap_or_default(),
        };
        
        // Create initial tab
        app.add_new_tab();
        app
    }

    fn add_new_tab(&mut self) {
        let tab = TerminalTab {
            id: Uuid::new_v4(),
            title: format!("Terminal {}", self.tabs.len() + 1),
            content: "Welcome to GPUI Multi-Page AI Terminal\n$ ".to_string(),
            input: String::new(),
            mode: AppMode::Terminal,
            history: Vec::new(),
            ai_conversation: Vec::new(),
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
                    tab.content.push_str(&format!("{}\n", command));
                    tab.history.push(command.clone());
                    
                    // Simple command simulation
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
                AppMode::AiAsist => {
                    // Add user message
                    tab.ai_conversation.push(AiMessage {
                        role: "user".to_string(),
                        content: command.clone(),
                    });
                    
                    // Simulate AI response (in real app, this would call Gemini API)
                    let ai_response = if self.gemini_api_key.is_empty() {
                        "AI Assistant: Please set GEMINI_API_KEY environment variable to enable AI features.".to_string()
                    } else {
                        format!("AI Assistant: I understand you asked about '{}'. This is a simulated response. In the full implementation, this would connect to Gemini API.", command)
                    };
                    
                    tab.ai_conversation.push(AiMessage {
                        role: "assistant".to_string(),
                        content: ai_response,
                    });
                }
            }
        }
    }
}

impl eframe::App for TerminalApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
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
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("New Tab (Cmd+T)").clicked() {
                        self.add_new_tab();
                        ui.close_menu();
                    }
                    if ui.button("Close Tab").clicked() && self.tabs.len() > 1 {
                        self.close_tab(self.active_tab);
                        ui.close_menu();
                    }
                });
                
                ui.menu_button("View", |ui| {
                    if ui.button("Switch Mode (Cmd+I)").clicked() {
                        self.switch_mode();
                        ui.close_menu();
                    }
                });
                
                ui.separator();
                
                // Current mode indicator
                if let Some(tab) = self.tabs.get(self.active_tab) {
                    let mode_text = match tab.mode {
                        AppMode::Terminal => "🖥️ Terminal Mode",
                        AppMode::AiAsist => "🤖 AI Assistant Mode",
                    };
                    ui.colored_label(egui::Color32::LIGHT_BLUE, mode_text);
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
                                );
                            }
                            AppMode::AiAsist => {
                                ui.heading("🤖 AI Assistant");
                                ui.separator();
                                
                                for message in &tab_ai_conversation {
                                    let color = if message.role == "user" {
                                        egui::Color32::LIGHT_BLUE
                                    } else {
                                        egui::Color32::LIGHT_GREEN
                                    };
                                    
                                    ui.colored_label(color, format!("{}: {}", message.role, message.content));
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
                
                // Help text
                ui.separator();
                ui.small(match tab_mode {
                    AppMode::Terminal => "Terminal Mode - Type commands and press Enter. Try: pwd, ls, echo hello, clear",
                    AppMode::AiAsist => "AI Assistant Mode - Ask questions and get AI-powered responses",
                });
                ui.small("Press Cmd+I to switch modes, Cmd+T for new tab");
            });
        });
    }
}
