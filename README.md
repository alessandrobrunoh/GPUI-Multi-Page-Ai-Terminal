# GPUI Multi-Page AI Terminal

A modern, Zed-like desktop terminal application built with Rust and eframe/egui, featuring AI assistance powered by Google's Gemini API.

## Features

✅ **Multi-Tab Terminal Interface**
- Create and manage multiple terminal tabs
- Tab switching and closing functionality
- Keyboard shortcuts (Cmd+T for new tab)

✅ **Dual Mode Operation**
- **Terminal Mode**: Real terminal emulation with PTY support
- **AI Assistant Mode**: Integrated AI chat interface
- Switch modes with Cmd+I keyboard shortcut

✅ **AI Integration**
- Google Gemini API integration for AI responses
- Conversation history per tab
- Ask questions and get intelligent responses
- Code completion capabilities (ready for future implementation)

✅ **Modern Interface**
- Clean, Zed-inspired design
- Responsive layout with proper scrolling
- Monospace font for terminal output
- Color-coded AI conversation display

✅ **Real Terminal Support**
- Portable PTY integration for real shell sessions
- Cross-platform compatibility (Windows, macOS, Linux)
- Fallback demo mode when PTY unavailable

## Getting Started

### Prerequisites

- Rust 1.70 or later
- System dependencies for GUI development:
  - On Ubuntu/Debian: `sudo apt install libgtk-3-dev libglib2.0-dev`
  - On Fedora: `sudo dnf install gtk3-devel glib2-devel`
  - On macOS: No additional dependencies needed
  - On Windows: No additional dependencies needed

### Installation

1. Clone the repository:
```bash
git clone https://github.com/alessandrobrunoh/GPUI-Multi-Page-Ai-Terminal.git
cd GPUI-Multi-Page-Ai-Terminal
```

2. Build the application:
```bash
cargo build --release
```

3. (Optional) Set up Gemini API for AI features:
```bash
export GEMINI_API_KEY=your_api_key_here
```

### Running

```bash
cargo run
```

## Usage

### Keyboard Shortcuts

- **Cmd+T** (Ctrl+T on Linux/Windows): Create new terminal tab
- **Cmd+I** (Ctrl+I on Linux/Windows): Switch between Terminal and AI Assistant modes
- **Enter**: Execute command or send AI message
- **Mouse**: Click tabs to switch, click "+" to add new tab, click "✕" to close tab

### Terminal Mode

In terminal mode, you can:
- Run shell commands (if PTY is available)
- Use built-in demo commands: `pwd`, `ls`, `echo`, `clear`
- Navigate command history
- Real-time terminal output display

### AI Assistant Mode

In AI mode, you can:
- Ask questions and get intelligent responses
- Maintain conversation context
- Get help with coding and technical questions
- View conversation history

## Configuration

### Environment Variables

- `GEMINI_API_KEY`: Your Google Gemini API key for AI features
- `SHELL`: Preferred shell for terminal sessions (defaults to `/bin/bash` on Unix)

### API Key Setup

To use AI features, you need a Google Gemini API key:

1. Visit [Google AI Studio](https://makersuite.google.com/)
2. Create a new API key
3. Set the environment variable:
   ```bash
   export GEMINI_API_KEY=your_key_here
   ```

## Architecture

The application is structured with clean separation of concerns:

- **Main Application** (`src/main.rs`): Core UI and application logic
- **Terminal Module** (`src/terminal/`): PTY integration and terminal session management
- **AI Module** (`src/ai/`): Gemini API integration and AI conversation handling

### Technology Stack

- **UI Framework**: eframe/egui (immediate mode GUI)
- **Terminal Emulation**: portable-pty
- **AI Integration**: Google Gemini API via reqwest
- **Async Runtime**: Tokio
- **Cross-platform**: Native compilation for all major platforms

## Development

### Building from Source

```bash
# Debug build
cargo build

# Release build
cargo build --release

# Run with logging
RUST_LOG=debug cargo run
```

### Code Structure

```
src/
├── main.rs           # Main application and UI logic
├── terminal/         # Terminal emulation
│   ├── mod.rs       # PTY session management
│   └── pty.rs       # PTY utilities
└── ai/              # AI integration
    └── mod.rs       # Gemini API client
```

## Contributing

1. Fork the repository
2. Create a feature branch: `git checkout -b feature-name`
3. Make your changes and add tests
4. Commit: `git commit -am 'Add feature'`
5. Push: `git push origin feature-name`
6. Create a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- Inspired by [Zed](https://zed.dev/) editor's interface design
- Built with [eframe/egui](https://github.com/emilk/egui) immediate mode GUI
- Terminal emulation powered by [portable-pty](https://github.com/wez/wezterm/tree/main/pty)
- AI features powered by [Google Gemini](https://ai.google.dev/)

## Roadmap

- [ ] Enhanced terminal emulation with proper ANSI escape sequence support
- [ ] Auto-completion suggestions in terminal mode
- [ ] Theme customization and Zed-like styling
- [ ] Split panes and layout management
- [ ] Plugin system for extensions
- [ ] Session persistence and restoration
- [ ] Advanced AI features (code generation, debugging assistance)