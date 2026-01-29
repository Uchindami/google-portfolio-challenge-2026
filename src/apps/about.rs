//! About App - About me / bio section

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

pub struct AboutApp {
    scroll: u16,
}

impl AboutApp {
    pub fn new() -> Self {
        Self { scroll: 0 }
    }
}

impl App for AboutApp {
    fn handle_key(&mut self, key: KeyEvent) -> bool {
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => true,
            KeyCode::Up | KeyCode::Char('k') => {
                self.scroll = self.scroll.saturating_sub(1);
                false
            }
            KeyCode::Down | KeyCode::Char('j') => {
                self.scroll = self.scroll.saturating_add(1);
                false
            }
            _ => false,
        }
    }

    fn render(&self, frame: &mut Frame) {
        let area = frame.area();

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Theme::SUCCESS))
            .title(" ABOUT ME ")
            .title_style(Style::default().fg(Theme::SUCCESS).add_modifier(Modifier::BOLD));

        let inner = block.inner(area);
        frame.render_widget(block, area);

        // Two column layout
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Length(30), Constraint::Min(40)])
            .split(inner);

        // ASCII art avatar
        let avatar = Paragraph::new(vec![
            Line::from(""),
            Line::from(Span::styled("     .-\"\"\"\"\"-.", Style::default().fg(Theme::SUCCESS))),
            Line::from(Span::styled("   .'          '.", Style::default().fg(Theme::SUCCESS))),
            Line::from(Span::styled("  /   O      O   \\", Style::default().fg(Theme::SUCCESS))),
            Line::from(Span::styled(" :                :", Style::default().fg(Theme::SUCCESS))),
            Line::from(Span::styled(" |                |", Style::default().fg(Theme::SUCCESS))),
            Line::from(Span::styled(" :    .------.    :", Style::default().fg(Theme::SUCCESS))),
            Line::from(Span::styled("  \\  '        '  /", Style::default().fg(Theme::SUCCESS))),
            Line::from(Span::styled("   '.          .'", Style::default().fg(Theme::SUCCESS))),
            Line::from(Span::styled("     '-......-'", Style::default().fg(Theme::SUCCESS))),
            Line::from(""),
            Line::from(Span::styled("   MANFRED CHIRAMBO", Style::default().add_modifier(Modifier::BOLD))),
            Line::from(Span::styled("  Software Engineer & ML", Style::default().fg(Theme::MUTED))),
            Line::from(""),
            Line::from("  ━━━━━━━━━━━━━━━━━━━━"),
            Line::from(""),
            Line::from(Span::styled("  📍 Blantyre, MW", Style::default())),
            Line::from(Span::styled("  💼 3+ years exp", Style::default())),
            Line::from(Span::styled("  🎯 Full Stack/AI", Style::default())),
            Line::from(Span::styled("  ❤️  Rust & Python", Style::default())),
        ])
        .block(Block::default().borders(Borders::ALL).title(" PROFILE "));

        frame.render_widget(avatar, layout[0]);

        // Bio text
        let bio_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(10), Constraint::Length(2)])
            .split(layout[1]);

        let bio = Paragraph::new(vec![
            Line::from(""),
            Line::from(Span::styled("Hello, World! 👋", Style::default().fg(Theme::SUCCESS).add_modifier(Modifier::BOLD))),
            Line::from(""),
            Line::from("I'm a Software Engineer & ML Specialist passionate about building"),
            Line::from("intelligent systems and scalable web applications. My expertise"),
            Line::from("spans across full-stack development, cloud infrastructure, and"),
            Line::from("machine learning integration."),
            Line::from(""),
            Line::from(Span::styled("What drives me:", Style::default().add_modifier(Modifier::BOLD))),
            Line::from(""),
            Line::from("  → Creating impactful solutions (Health, Energy, Robotics)"),
            Line::from("  → Architecture & System Design"),
            Line::from("  → Winning Hackathons & Solving Complex Problems"),
            Line::from("  → Open Source Contribution"),
            Line::from(""),
            Line::from(Span::styled("When I'm not coding:", Style::default().add_modifier(Modifier::BOLD))),
            Line::from(""),
            Line::from("  🤖 Training ML models"),
            Line::from("  📚 Exploring new tech stacks"),
            Line::from("  🤝 Mentoring & Community work"),
            Line::from("  🏃 Active lifestyle"),
            Line::from(""),
            Line::from(Span::styled("Fun fact:", Style::default().fg(Theme::WARNING))),
            Line::from("I won the Digitalization Malawi Hackathon and the Africa ASR Challenge!"),
            Line::from("Built with Rust + Ratatui."),
        ])
        .block(Block::default().borders(Borders::ALL).title(" BIO "))
        .wrap(Wrap { trim: true })
        .scroll((self.scroll, 0));

        frame.render_widget(bio, bio_layout[0]);

        // Footer
        let footer = Paragraph::new(Line::from(vec![
            Span::styled(" [↑↓] ", Style::default().fg(Theme::WARNING)),
            Span::raw("Scroll  "),
            Span::styled("[Q] ", Style::default().fg(Theme::WARNING)),
            Span::raw("Quit"),
        ]));
        frame.render_widget(footer, bio_layout[1]);
    }

    fn name(&self) -> &'static str {
        "About"
    }
}
