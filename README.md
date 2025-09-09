# GPUI-Multi-Page-Ai-Terminal

A Zed-like terminal with AI features built in Rust using ratatui (TUI library).

## Features

✅ **AI Auto Completion** - Press `Tab` for command suggestions  
✅ **AI Ask Question** - Use `ai <your question>` to get help from AI assistant  
✅ **Terminal Emulator** - Basic shell simulation with common commands  
✅ **Multi Tab Support** - Create and manage multiple terminal sessions  

## Commands

- `help` - Show available commands
- `clear` - Clear terminal
- `history` - Show command history
- `echo <text>` - Echo text
- `ai <question>` - Ask AI assistant a question
- `ls` - List files
- `pwd` - Print working directory
- `whoami` - Show current user
- `date` - Show current date and time

## Keyboard Shortcuts

- `Ctrl+Q` - Quit application
- `Ctrl+T` - Create new tab
- `Ctrl+W` - Close current tab
- `Ctrl+Tab` - Switch to next tab
- `Tab` - Show command completions
- `Enter` - Execute command
- `Backspace` - Delete character

## Installation & Usage

### Prerequisites

- Rust (1.70+ recommended)
- Cargo

### Build & Run

```bash
# Clone the repository
git clone https://github.com/alessandrobrunoh/GPUI-Multi-Page-Ai-Terminal.git
cd GPUI-Multi-Page-Ai-Terminal

# Build the project
cargo build --release

# Run the application
cargo run
```

## Architecture

The application is built with a modular architecture:

- **Terminal Module** - Handles terminal emulation and tab management
- **AI Module** - Provides AI completion and question-answering features
- **UI Module** - Renders the terminal interface using ratatui
- **App Module** - Main application state and event handling

## AI Integration

The AI features are currently implemented with mock responses for demonstration. In a production version, you can integrate with real AI APIs by:

1. Setting the `AI_API_KEY` environment variable
2. Implementing actual API calls in `src/ai/mod.rs`

## License

MIT License