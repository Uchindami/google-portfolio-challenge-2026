//! Shell module - handles command input, history, and output

use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

use crate::theme::Theme;

mod commands;
mod filesystem;

use commands::{execute_command, CommandResult};
pub use filesystem::{FSEntry, VirtualFS};

const PROMPT: &str = "guest@uchindami:~$ ";
const MAX_HISTORY: usize = 100;
const MAX_OUTPUT_LINES: usize = 500;

/// Result of submitting a command
pub enum ShellResult {
    /// Nothing special happened
    None,
    /// User wants to exit
    Exit,
    /// Launch a sub-app
    LaunchApp(&'static str),
}

/// Shell state
pub struct Shell {
    /// Current input buffer
    input_buffer: String,
    /// Cursor position in input
    cursor_pos: usize,
    /// Command history
    history: Vec<String>,
    /// Current position in history (for up/down navigation)
    history_pos: Option<usize>,
    /// Output lines (scrollback buffer)
    output: Vec<Line<'static>>,
    /// Virtual filesystem
    fs: VirtualFS,
    /// Current working directory
    cwd: String,
}

impl Shell {
    pub fn new() -> Self {
        Self {
            input_buffer: String::new(),
            cursor_pos: 0,
            history: Vec::new(),
            history_pos: None,
            output: Vec::new(),
            fs: VirtualFS::new(),
            cwd: "~".to_string(),
        }
    }

    /// Display welcome message
    pub fn show_welcome(&mut self) {
        let welcome = vec![
            "",
            "Connecting to uchindami.dev...",
            "Authenticated.",
            "",
            "╔══════════════════════════════════════════════════════════════════════════════╗",
            "║                                                                              ║",
            "║   ██╗   ██╗ ██████╗██╗  ██╗██╗███╗   ██╗██████╗  █████╗ ███╗   ███╗██╗       ║",
            "║   ██║   ██║██╔════╝██║  ██║██║████╗  ██║██╔══██╗██╔══██╗████╗ ████║██║       ║",
            "║   ██║   ██║██║     ███████║██║██╔██╗ ██║██║  ██║███████║██╔████╔██║██║       ║",
            "║   ██║   ██║██║     ██╔══██║██║██║╚██╗██║██║  ██║██╔══██║██║╚██╔╝██║██║       ║",
            "║   ╚██████╔╝╚██████╗██║  ██║██║██║ ╚████║██████╔╝██║  ██║██║ ╚═╝ ██║██║       ║",
            "║    ╚═════╝  ╚═════╝╚═╝  ╚═╝╚═╝╚═╝  ╚═══╝╚═════╝ ╚═╝  ╚═╝╚═╝     ╚═╝╚═╝       ║",
            "║                                                                              ║",
            "║                       Welcome to my portfolio                                ║",
            "║                                                                              ║",
            "╚══════════════════════════════════════════════════════════════════════════════╝",
            "",
            "System Information:",
            "  OS: uchindami OS v2.0.26",
            "  Kernel: Rust 1.83.0",
            "  Uptime: 420 days, 69 hours",
            "  Shell: rsh (Rust Shell)",
            "  Complete the sentence: I use ____ btw",
            "",
            "Type 'ls' to see available commands, or 'help' for assistance.",
            "",
        ];

        for line in welcome {
            self.output.push(Line::from(Span::styled(
                line.to_string(),
                Style::default().fg(Theme::FOREGROUND),
            )));
        }
    }
    


    /// Handle character input
    pub fn input(&mut self, c: char) {
        self.input_buffer.insert(self.cursor_pos, c);
        self.cursor_pos += 1;
        self.history_pos = None;
    }

    /// Handle backspace
    pub fn backspace(&mut self) {
        if self.cursor_pos > 0 {
            self.cursor_pos -= 1;
            self.input_buffer.remove(self.cursor_pos);
        }
    }

    /// Move cursor left
    pub fn cursor_left(&mut self) {
        if self.cursor_pos > 0 {
            self.cursor_pos -= 1;
        }
    }

    /// Move cursor right
    pub fn cursor_right(&mut self) {
        if self.cursor_pos < self.input_buffer.len() {
            self.cursor_pos += 1;
        }
    }

    /// Navigate to previous history entry
    pub fn history_prev(&mut self) {
        if self.history.is_empty() {
            return;
        }
        let new_pos = match self.history_pos {
            None => self.history.len() - 1,
            Some(0) => 0,
            Some(p) => p - 1,
        };
        self.history_pos = Some(new_pos);
        self.input_buffer = self.history[new_pos].clone();
        self.cursor_pos = self.input_buffer.len();
    }

    /// Navigate to next history entry
    pub fn history_next(&mut self) {
        match self.history_pos {
            None => {}
            Some(p) if p >= self.history.len() - 1 => {
                self.history_pos = None;
                self.input_buffer.clear();
                self.cursor_pos = 0;
            }
            Some(p) => {
                self.history_pos = Some(p + 1);
                self.input_buffer = self.history[p + 1].clone();
                self.cursor_pos = self.input_buffer.len();
            }
        }
    }

    /// Tab completion
    pub fn tab_complete(&mut self) {
        let completions = self.fs.complete(&self.input_buffer, &self.cwd);
        if completions.len() == 1 {
            self.input_buffer = completions[0].clone();
            self.cursor_pos = self.input_buffer.len();
        } else if completions.len() > 1 {
            // Show available completions
            self.output.push(Line::from(format!("{}{}", PROMPT, self.input_buffer)));
            self.output.push(Line::from(completions.join("  ")));
        }
    }

    /// Submit current input and execute command
    pub fn submit(&mut self) -> ShellResult {
        let cmd = self.input_buffer.trim().to_string();

        // Add prompt + input to output
        self.output.push(Line::from(vec![
            Span::styled(self.get_prompt(), Style::default().fg(Theme::SECONDARY)),
            Span::raw(cmd.clone()),
        ]));

        // Clear input
        self.input_buffer.clear();
        self.cursor_pos = 0;

        if cmd.is_empty() {
            return ShellResult::None;
        }

        // Check for exit commands first
        if cmd == "exit" || cmd == "quit" {
            return ShellResult::Exit;
        }

        // Add to history
        if self.history.last() != Some(&cmd) {
            self.history.push(cmd.clone());
            if self.history.len() > MAX_HISTORY {
                self.history.remove(0);
            }
        }
        self.history_pos = None;

        // Execute command
        let result = execute_command(&cmd, &mut self.fs, &mut self.cwd);
        match result {
            CommandResult::Output(lines) => {
                for line in lines {
                    self.output.push(line);
                }
                ShellResult::None
            }
            CommandResult::Clear => {
                self.output.clear();
                ShellResult::None
            }
            CommandResult::AppLaunch(app_name) => {
                ShellResult::LaunchApp(app_name)
            }
        }
    }

    /// Get current prompt string
    fn get_prompt(&self) -> String {
        format!("guest@uchindami:{}$ ", self.cwd)
    }

    /// Render shell to frame
    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let height = area.height as usize;

        // Calculate visible lines (reserve 1 for input line)
        let visible_height = height.saturating_sub(1);

        // Build output lines
        let mut lines: Vec<Line> = Vec::new();

        // Add output history (show last N lines)
        let start = self.output.len().saturating_sub(visible_height);
        for line in &self.output[start..] {
            lines.push(line.clone());
        }

        // Add current input line with cursor - multi-color prompt
        let before_cursor: String = self.input_buffer.chars().take(self.cursor_pos).collect();
        let cursor_char = self.input_buffer.chars().nth(self.cursor_pos).unwrap_or(' ');
        let after_cursor: String = self.input_buffer.chars().skip(self.cursor_pos + 1).collect();

        lines.push(Line::from(vec![
            // User: green
            Span::styled("guest", Style::default().fg(Theme::GUEST)),
            // @ symbol: muted
            Span::styled("@", Style::default().fg(Theme::SECONDARY)),
            // Host: primary color
            Span::styled("uchindami", Style::default().fg(Theme::SECONDARY)),
            // Colon: muted
            Span::styled(":", Style::default().fg(Theme::MUTED)),
            // Path: secondary/coral
            Span::styled(&self.cwd, Style::default().fg(Theme::SECONDARY)),
            // Dollar sign: warning/gold
            Span::styled("$ ", Style::default().fg(Theme::SECONDARY)),
            // User input
            Span::raw(before_cursor),
            // Cursor
            Span::styled(
                cursor_char.to_string(),
                Style::default().bg(Theme::SUCCESS).fg(Theme::BACKGROUND),
            ),
            Span::raw(after_cursor),
        ]));

        let paragraph = Paragraph::new(lines);
        frame.render_widget(paragraph, area);
    }
}
