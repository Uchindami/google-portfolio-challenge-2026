//! Interactive Terminal Portfolio
//! A shell-based TUI portfolio built with Ratatui

use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use ratatui::{
    style::Style,
    widgets::{Block, Borders},
    DefaultTerminal, Frame,
};

mod apps;
mod shell;
mod theme;
mod ui;

use apps::{AboutApp, App as SubApp, ContactApp, DashboardApp, ResumeApp};
use shell::{Shell, ShellResult};
use theme::Theme;

/// Current view state
enum View {
    Shell,
    Dashboard(DashboardApp),
    Resume(ResumeApp),
    Contact(ContactApp),
    About(AboutApp),
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = App::new().run(terminal);
    ratatui::restore();
    result
}

/// Main application state
pub struct App {
    shell: Shell,
    view: View,
    should_quit: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            shell: Shell::new(),
            view: View::Shell,
            should_quit: false,
        }
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        // Show welcome sequence
        self.shell.show_welcome();

        while !self.should_quit {
            terminal.draw(|frame| self.draw(frame))?;
            
            // Poll for events with timeout
            if event::poll(std::time::Duration::from_millis(100))? {
                self.handle_events()?;
            }
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        match &self.view {
            View::Shell => self.draw_shell(frame),
            View::Dashboard(app) => app.render(frame),
            View::Resume(app) => app.render(frame),
            View::Contact(app) => app.render(frame),
            View::About(app) => app.render(frame),
        }
    }

    fn draw_shell(&self, frame: &mut Frame) {
        let area = frame.area();

        let block = crate::ui::themed_block("portfolio");
        let inner = block.inner(area);
        frame.render_widget(block, area);

        // Render shell output + prompt
        self.shell.render(frame, inner);
    }

    fn handle_events(&mut self) -> Result<()> {
        if let Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press {
                return Ok(());
            }

            // Route events based on current view
            match &mut self.view {
                View::Shell => self.handle_shell_events(key),
                View::Dashboard(app) => {
                    if app.handle_key(key) {
                        // Check if dashboard wants to launch another app
                        if let Some(app_name) = app.launch_app {
                            self.launch_app(app_name);
                        } else {
                            self.view = View::Shell;
                        }
                    }
                }
                View::Resume(app) => {
                    if app.handle_key(key) {
                        self.view = View::Shell;
                    }
                }
                View::Contact(app) => {
                    if app.handle_key(key) {
                        self.view = View::Shell;
                    }
                }
                View::About(app) => {
                    if app.handle_key(key) {
                        self.view = View::Shell;
                    }
                }
            }
        }
        Ok(())
    }

    fn handle_shell_events(&mut self, key: crossterm::event::KeyEvent) {
        // Ctrl+C or Ctrl+D to quit
        if key.modifiers.contains(KeyModifiers::CONTROL) {
            match key.code {
                KeyCode::Char('c') | KeyCode::Char('d') => {
                    self.should_quit = true;
                    return;
                }
                _ => {}
            }
        }

        match key.code {
            KeyCode::Enter => match self.shell.submit() {
                ShellResult::None => {}
                ShellResult::Exit => {
                    self.should_quit = true;
                }
                ShellResult::LaunchApp(app_name) => {
                    self.launch_app(app_name);
                }
            },
            KeyCode::Char(c) => {
                self.shell.input(c);
            }
            KeyCode::Backspace => {
                self.shell.backspace();
            }
            KeyCode::Left => {
                self.shell.cursor_left();
            }
            KeyCode::Right => {
                self.shell.cursor_right();
            }
            KeyCode::Up => {
                self.shell.history_prev();
            }
            KeyCode::Down => {
                self.shell.history_next();
            }
            KeyCode::Tab => {
                self.shell.tab_complete();
            }
            _ => {}
        }
    }

    fn launch_app(&mut self, app_name: &str) {
        match app_name {
            "dashboard" => self.view = View::Dashboard(DashboardApp::new()),
            "resume" => self.view = View::Resume(ResumeApp::new()),
            "contact" => self.view = View::Contact(ContactApp::new()),
            "about" => self.view = View::About(AboutApp::new()),
            _ => {}
        }
    }
}
