//! Sub-TUI Applications
//! Each app is a full-screen TUI that replaces the shell while running

mod about;
mod contact;
mod dashboard;
mod resume;

pub use about::AboutApp;
pub use contact::ContactApp;
pub use dashboard::DashboardApp;
pub use resume::ResumeApp;

use crossterm::event::KeyEvent;
use ratatui::Frame;

/// Trait for sub-TUI applications
pub trait App {
    /// Handle a key event, return true if the app should exit
    fn handle_key(&mut self, key: KeyEvent) -> bool;

    /// Render the app
    fn render(&self, frame: &mut Frame);

    /// Get the app name for display
    fn name(&self) -> &'static str;
}
