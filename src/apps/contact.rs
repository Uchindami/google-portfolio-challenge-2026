//! Contact App - Contact information with QR codes

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use super::App;
use crate::theme::Theme;

const CONTACTS: [(&str, &str, &str); 6] = [
    ("📧", "Email", "manfredchirambojz@gmail.com"),
    ("📞", "Phone", "(+265) 885 624 718"),
    ("🐙", "GitHub", "github.com/Uchindami"),
    ("💼", "LinkedIn", "linkedin.com/in/manfred-chirambo"),
    ("🌐", "Website", "uchindami.vercel.app"),
    ("📍", "Location", "Blantyre, Malawi"),
];

pub struct ContactApp {
    selected: usize,
}

impl ContactApp {
    pub fn new() -> Self {
        Self { selected: 0 }
    }
}

impl App for ContactApp {
    fn handle_key(&mut self, key: KeyEvent) -> bool {
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => true,
            KeyCode::Up | KeyCode::Char('k') => {
                if self.selected > 0 {
                    self.selected -= 1;
                }
                false
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if self.selected < CONTACTS.len() - 1 {
                    self.selected += 1;
                }
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
            .title(" CONTACT ")
            .title_style(Style::default().fg(Theme::SUCCESS).add_modifier(Modifier::BOLD));

        let inner = block.inner(area);
        frame.render_widget(block, area);

        // Layout
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(inner);

        // Contact list
        let mut lines = vec![
            Line::from(""),
            Line::from(Span::styled(
                "  Let's connect!",
                Style::default().fg(Theme::SUCCESS).add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
        ];

        for (i, (icon, label, value)) in CONTACTS.iter().enumerate() {
            let style = if i == self.selected {
                Style::default().fg(Theme::BACKGROUND).bg(Theme::SUCCESS)
            } else {
                Style::default()
            };

            lines.push(Line::from(vec![
                Span::raw("  "),
                Span::styled(*icon, style),
                Span::raw(" "),
                Span::styled(format!("{}: ", label), Style::default().add_modifier(Modifier::BOLD)),
                Span::styled(*value, Style::default().fg(Theme::SECONDARY)),
            ]));
            lines.push(Line::from(""));
        }

        lines.push(Line::from(""));
        lines.push(Line::from(vec![
            Span::styled(" [↑↓] ", Style::default().fg(Theme::WARNING)),
            Span::raw("Select  "),
            Span::styled("[C] ", Style::default().fg(Theme::WARNING)),
            Span::raw("Copy  "),
            Span::styled("[Q] ", Style::default().fg(Theme::WARNING)),
            Span::raw("Quit"),
        ]));

        let contact_list = Paragraph::new(lines)
            .block(Block::default().borders(Borders::ALL).title(" INFO "));
        frame.render_widget(contact_list, layout[0]);

        // ASCII QR Code placeholder
        let qr = Paragraph::new(vec![
            Line::from(""),
            Line::from(Span::styled("  Scan to connect:", Style::default().add_modifier(Modifier::BOLD))),
            Line::from(""),
            Line::from("  ██████████████  ████  ██████████████"),
            Line::from("  ██          ██  ████  ██          ██"),
            Line::from("  ██  ██████  ██    ██  ██  ██████  ██"),
            Line::from("  ██  ██████  ██  ████  ██  ██████  ██"),
            Line::from("  ██  ██████  ██  ██    ██  ██████  ██"),
            Line::from("  ██          ██        ██          ██"),
            Line::from("  ██████████████  ██  ██████████████"),
            Line::from("                  ██████            "),
            Line::from("  ████████████████      ██████████  "),
            Line::from("                  ██  ██            "),
            Line::from("  ██████████████  ██      ████████  "),
            Line::from("  ██          ██    ██  ██  ██  ██  "),
            Line::from("  ██  ██████  ██  ██████████    ██  "),
            Line::from("  ██  ██████  ██  ██    ██████████  "),
            Line::from("  ██  ██████  ██    ██  ██      ██  "),
            Line::from("  ██          ██  ██  ████████  ██  "),
            Line::from("  ██████████████  ██  ██    ██████  "),
            Line::from(""),
            Line::from(Span::styled("  github.com/Uchindami", Style::default().fg(Theme::MUTED))),
        ])
        .block(Block::default().borders(Borders::ALL).title(" QR CODE "));

        frame.render_widget(qr, layout[1]);
    }

    fn name(&self) -> &'static str {
        "Contact"
    }
}
