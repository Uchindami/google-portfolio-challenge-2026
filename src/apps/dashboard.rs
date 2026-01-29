//! Dashboard App - Interactive overview of the portfolio

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

use super::App;
use crate::theme::Theme;

const CARDS: [(&str, &str, &str, Option<&str>); 4] = [
    ("1", "📋 RESUME", "Skills, Projects\nExperience, Education\nCertifications", Some("resume")),
    ("2", "📞 CONTACT", "Email, LinkedIn\nGitHub, Website\nLilongwe, Malawi", Some("contact")),
    ("3", "📚 ABOUT", "Background\nInterests\nGoals", Some("about")),
    ("4", "🎯 BACK", "Return to\nShell Prompt\n", None),
];

pub struct DashboardApp {
    selected: usize,
    /// App to launch when exiting (if any)
    pub launch_app: Option<&'static str>,
}

impl DashboardApp {
    pub fn new() -> Self {
        Self { 
            selected: 0,
            launch_app: None,
        }
    }
}

impl App for DashboardApp {
    fn handle_key(&mut self, key: KeyEvent) -> bool {
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => true,
            KeyCode::Char('1') => { self.selected = 0; false }
            KeyCode::Char('2') => { self.selected = 1; false }
            KeyCode::Char('3') => { self.selected = 2; false }
            KeyCode::Char('4') => { self.selected = 3; false }
            KeyCode::Left | KeyCode::Char('h') => {
                if self.selected > 0 { self.selected -= 1; }
                false
            }
            KeyCode::Right | KeyCode::Char('l') => {
                if self.selected < 3 { self.selected += 1; }
                false
            }
            KeyCode::Up | KeyCode::Char('k') => {
                if self.selected >= 2 { self.selected -= 2; }
                false
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if self.selected < 2 { self.selected += 2; }
                false
            }
            KeyCode::Enter => {
                let (_, _, _, app) = CARDS[self.selected];
                if let Some(app_name) = app {
                    self.launch_app = Some(app_name);
                }
                // Return true to exit dashboard (either to shell or to another app)
                true
            }
            _ => false,
        }
    }

    fn render(&self, frame: &mut Frame) {
        let area = frame.area();

        // Main border
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Theme::SUCCESS))
            .title(" MANFRED CHIRAMBO - DASHBOARD ")
            .title_style(Style::default().fg(Theme::SUCCESS).add_modifier(Modifier::BOLD));

        let inner = block.inner(area);
        frame.render_widget(block, area);

        // Layout: Header, Cards (2x2), Footer
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(5),  // Quick stats
                Constraint::Min(12),    // Cards
                Constraint::Length(2),  // Footer
            ])
            .split(inner);

        // Quick stats header
        let stats = Paragraph::new(vec![
            Line::from(""),
            Line::from(vec![
                Span::raw("  👨‍💻 Software Engineer & ML Specialist  |  📍 Blantyre, Malawi  |  🎯 3+ Years Experience"),
            ]),
            Line::from(vec![
                Span::raw("  ⭐ Hackathon Winner (x3)   |  🚀 Full Stack & AI  |  📝 BSc ICT"),
            ]),
        ])
        .block(Block::default().borders(Borders::ALL).title(" QUICK STATS "));
        frame.render_widget(stats, layout[0]);

        // Cards grid (2x2)
        let card_rows = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(layout[1]);

        for (row_idx, row) in card_rows.iter().enumerate() {
            let cols = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(*row);

            for (col_idx, col) in cols.iter().enumerate() {
                let card_idx = row_idx * 2 + col_idx;
                let (num, title, content, _) = CARDS[card_idx];
                let is_selected = card_idx == self.selected;

                let style = if is_selected {
                    Style::default().fg(Theme::BACKGROUND).bg(Theme::SUCCESS)
                } else {
                    Style::default().fg(Theme::FOREGROUND)
                };

                let border_style = if is_selected {
                    Style::default().fg(Theme::SUCCESS).add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Theme::MUTED)
                };

                let card = Paragraph::new(vec![
                    Line::from(""),
                    Line::from(Span::styled(title, style.add_modifier(Modifier::BOLD))),
                    Line::from(""),
                    Line::from(content.lines().next().unwrap_or("")),
                    Line::from(content.lines().nth(1).unwrap_or("")),
                    Line::from(content.lines().nth(2).unwrap_or("")),
                ])
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(border_style)
                        .title(format!(" [{}] ", num)),
                )
                .wrap(Wrap { trim: true });

                frame.render_widget(card, *col);
            }
        }

        // Footer
        let footer = Paragraph::new(Line::from(vec![
            Span::styled(" [1-4] ", Style::default().fg(Theme::WARNING)),
            Span::raw("Select  "),
            Span::styled("[Enter] ", Style::default().fg(Theme::WARNING)),
            Span::raw("Open  "),
            Span::styled("[Q] ", Style::default().fg(Theme::WARNING)),
            Span::raw("Quit  "),
            Span::styled("[←→↑↓] ", Style::default().fg(Theme::WARNING)),
            Span::raw("Navigate"),
        ]));
        frame.render_widget(footer, layout[2]);
    }

    fn name(&self) -> &'static str {
        "Dashboard"
    }
}

