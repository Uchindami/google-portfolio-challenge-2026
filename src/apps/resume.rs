//! Resume/CV App - Professional resume viewer

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
    Frame,
};

use super::App;
use crate::theme::Theme;

const SECTIONS: [(&str, &str); 7] = [
    ("Summary", "PROFESSIONAL SUMMARY"),
    ("Experience", "WORK EXPERIENCE"),
    ("Education", "EDUCATION"),
    ("Skills", "TECHNICAL SKILLS"),
    ("Projects", "KEY PROJECTS"),
    ("Certifications", "CERTIFICATIONS"),
    ("Languages", "LANGUAGES"),
];

/// Project with name, description, and URL
const PROJECTS: [(&str, &str, &str); 4] = [
    (
        "Apply Malawi",
        "AI-powered job application platform with resume parsing",
        "https://github.com/uchindami/apply-malawi",
    ),
    (
        "Chichewa ASR",
        "Speech recognition for Chichewa language - Google ASR Challenge 2nd Place",
        "https://github.com/uchindami/chichewa-asr",
    ),
    (
        "Portfolio TUI",
        "This terminal portfolio built with Rust & Ratatui",
        "https://github.com/uchindami/portfolio-tui",
    ),
    (
        "GeoSight Dashboards",
        "Geospatial data visualization for UNICEF EMOPS",
        "https://geosight.unicef.org",
    ),
];

/// Create an OSC 8 hyperlink (clickable in modern terminals)
fn osc8_link(url: &str, text: &str) -> String {
    format!("\x1b]8;;{}\x07{}\x1b]8;;\x07", url, text)
}

pub struct ResumeApp {
    selected_section: usize,
    list_state: ListState,
    /// Selected project index (when in Projects section)
    selected_project: usize,
    /// Status message (shows URL when [O] is pressed)
    status_message: Option<String>,
}

impl ResumeApp {
    pub fn new() -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));
        Self {
            selected_section: 0,
            list_state,
            selected_project: 0,
            status_message: None,
        }
    }
    
    /// Get the currently selected project's URL (if in Projects section)
    fn get_current_project_url(&self) -> Option<&'static str> {
        if self.selected_section == 4 {
            Some(PROJECTS[self.selected_project].2)
        } else {
            None
        }
    }

    fn get_section_content(&self) -> Vec<Line<'static>> {
        match self.selected_section {
            0 => vec![
                Line::from(""),
                Line::from(Span::styled(
                    "Software Engineer | ML/AI Engineer | BSc ICT",
                    Style::default().add_modifier(Modifier::BOLD),
                )),
                Line::from("Specializing in full-stack development, machine learning, and system integration."),
                Line::from(""),
                Line::from("Focus Areas:"),
                Line::from("  • Web & Mobile Applications (React, Next.js, Flutter)"),
                Line::from("  • Backend Systems & REST APIs (Node.js, PostgreSQL)"),
                Line::from("  • Machine Learning & AI Integration"),
                Line::from("  • DevOps & Automation (Docker, Nginx)"),
                Line::from(""),
                Line::from("Core Values:"),
                Line::from("  → Building scalable, secure applications"),
                Line::from("  → Continuous learning and innovation"),
                Line::from("  → Open source contribution"),
            ],
            1 => vec![
                Line::from(""),
                Line::from(Span::styled("Software Engineer & ML Specialist", Style::default().add_modifier(Modifier::BOLD))),
                Line::from("Qubix Robotics • Aug 2024 - Present"),
                Line::from("  • Designed web/mobile apps (React, Next.js, Flutter, Node.js)"),
                Line::from("  • Optimized PostgreSQL/Firebase databases"),
                Line::from("  • Automated pipelines with Docker & Nginx"),
                Line::from(""),
                Line::from(Span::styled("Data Science Volunteer", Style::default().add_modifier(Modifier::BOLD))),
                Line::from("UNICEF EMOPS • Jan 2025 - Feb 2025"),
                Line::from("  • Aggregated data into geospatial dashboards (GeoSight)"),
                Line::from("  • Developed Power BI dashboards for decision-making"),
                Line::from(""),
                Line::from(Span::styled("Information Technology Intern", Style::default().add_modifier(Modifier::BOLD))),
                Line::from("Zuwa Energy • Jan 2022 - Apr 2023"),
                Line::from("  • Managed networks, firewalls, and business systems"),
                Line::from("  • Maintained Windows/Linux environments"),
                Line::from(""),
                Line::from(Span::styled("Intern Software Engineer", Style::default().add_modifier(Modifier::BOLD))),
                Line::from("Archi's Academy • Nov 2021 - Feb 2022"),
                Line::from("  • Contributed to software dev and codebase optimization"),
            ],
            2 => vec![
                Line::from(""),
                Line::from(Span::styled("BSc Information Communication Technology (ICT)", Style::default().add_modifier(Modifier::BOLD))),
                Line::from("Daeyang University • 2024"),
                Line::from("  • Specialization: Software Engineering, System Design, DB Mgmt"),
                Line::from("  • Member: CodeBrains & ICT Association of Malawi (ICTAM)"),
                Line::from(""),
                Line::from(Span::styled("Malawi School Certificate of Education (MSCE)", Style::default().add_modifier(Modifier::BOLD))),
                Line::from("St. John's Catholic Secondary School • 2018"),
                Line::from("  • 16 Points, strong performance in Math & Science"),
            ],
            3 => vec![
                Line::from(""),
                Line::from(Span::styled("Languages:", Style::default().fg(Theme::WARNING))),
                Line::from("  Rust, Python, JavaScript, PHP, Java, C#.NET, SQL"),
                Line::from(""),
                Line::from(Span::styled("Frameworks & Tools:", Style::default().fg(Theme::WARNING))),
                Line::from("  React, Next.js, Flutter, Node.js, Power BI"),
                Line::from(""),
                Line::from(Span::styled("Infrastructure & OS:", Style::default().fg(Theme::WARNING))),
                Line::from("  Docker, Nginx, Linux, Windows Server"),
                Line::from(""),
                Line::from(Span::styled("Databases:", Style::default().fg(Theme::WARNING))),
                Line::from("  PostgreSQL, MySQL, Firebase, Oracle"),
            ],
            4 => {
                // Projects section with selectable items
                let mut lines = vec![Line::from("")];
                
                for (i, (name, desc, url)) in PROJECTS.iter().enumerate() {
                    let is_selected = i == self.selected_project;
                    let prefix = if is_selected { "▶ " } else { "  " };
                    
                    // Project name with selection indicator
                    let name_style = if is_selected {
                        Style::default().fg(Theme::SUCCESS).add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().add_modifier(Modifier::BOLD)
                    };
                    
                    lines.push(Line::from(Span::styled(format!("{}{}", prefix, name), name_style)));
                    lines.push(Line::from(format!("    {}", desc)));
                    
                    // OSC 8 clickable link (works in modern terminals)
                    let link_text = osc8_link(url, "🔗 Open Link");
                    lines.push(Line::from(format!("    {}", link_text)));
                    lines.push(Line::from(""));
                }
                
                // Add status message if present
                if let Some(ref msg) = self.status_message {
                    lines.push(Line::from(""));
                    lines.push(Line::from(Span::styled(
                        msg.clone(),
                        Style::default().fg(Theme::WARNING),
                    )));
                }
                
                lines.push(Line::from(""));
                lines.push(Line::from(Span::styled(
                    "  [↑↓] Select project  [O] Show URL  [Enter] Copy URL",
                    Style::default().fg(Theme::MUTED),
                )));
                
                lines
            },
            5 => vec![
                Line::from(""),
                Line::from("  • Meta Front-End Engineer (2022)"),
                Line::from("  • Google Africa ASR Challenge (2nd Place)"),
                Line::from("  • Digitalization Malawi Hackathon (Winner)"),
            ],
            6 => vec![
                Line::from(""),
                Line::from("  • English"),
                Line::from("  • Chichewa"), // Inferred from location/nationality if safe, otherwise just standard
            ],
            _ => vec![],
        }
    }
}

impl App for ResumeApp {
    fn handle_key(&mut self, key: KeyEvent) -> bool {
        // Clear status message on any key
        self.status_message = None;
        
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => true,
            KeyCode::Up | KeyCode::Char('k') => {
                if self.selected_section == 4 {
                    // In Projects section - navigate projects
                    if self.selected_project > 0 {
                        self.selected_project -= 1;
                    }
                } else {
                    // Navigate sections
                    if self.selected_section > 0 {
                        self.selected_section -= 1;
                        self.list_state.select(Some(self.selected_section));
                    }
                }
                false
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if self.selected_section == 4 {
                    // In Projects section - navigate projects
                    if self.selected_project < PROJECTS.len() - 1 {
                        self.selected_project += 1;
                    }
                } else {
                    // Navigate sections
                    if self.selected_section < SECTIONS.len() - 1 {
                        self.selected_section += 1;
                        self.list_state.select(Some(self.selected_section));
                    }
                }
                false
            }
            KeyCode::Left | KeyCode::Char('h') => {
                // Navigate to previous section
                if self.selected_section > 0 {
                    self.selected_section -= 1;
                    self.list_state.select(Some(self.selected_section));
                    self.selected_project = 0;
                }
                false
            }
            KeyCode::Right | KeyCode::Char('l') => {
                // Navigate to next section
                if self.selected_section < SECTIONS.len() - 1 {
                    self.selected_section += 1;
                    self.list_state.select(Some(self.selected_section));
                    self.selected_project = 0;
                }
                false
            }
            KeyCode::Char('o') | KeyCode::Char('O') => {
                // Show URL for current project
                if let Some(url) = self.get_current_project_url() {
                    self.status_message = Some(format!("📋 URL: {}", url));
                }
                false
            }
            KeyCode::Enter => {
                // Show URL with copy hint
                if let Some(url) = self.get_current_project_url() {
                    self.status_message = Some(format!("🔗 {}", url));
                }
                false
            }
            KeyCode::Char('d') => {
                // TODO: Trigger download
                false
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
            .title(" CURRICULUM VITAE - UCHINDAMI ")
            .title_style(Style::default().fg(Theme::SUCCESS).add_modifier(Modifier::BOLD));

        let inner = block.inner(area);
        frame.render_widget(block, area);

        // Two-column layout
        let columns = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
            .split(inner);

        // Left column: Sections list + Actions
        let left_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(10), Constraint::Length(8)])
            .split(columns[0]);

        // Sections list
        let items: Vec<ListItem> = SECTIONS
            .iter()
            .enumerate()
            .map(|(i, (name, _))| {
                let style = if i == self.selected_section {
                    Style::default().fg(Theme::BACKGROUND).bg(Theme::SUCCESS)
                } else {
                    Style::default()
                };
                let prefix = if i == self.selected_section { "▶ " } else { "  " };
                ListItem::new(format!("{}{}", prefix, name)).style(style)
            })
            .collect();

        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title(" SECTIONS "));
        frame.render_stateful_widget(list, left_layout[0], &mut self.list_state.clone());

        // Actions
        let actions = Paragraph::new(vec![
            Line::from(""),
            Line::from(vec![
                Span::styled(" [D] ", Style::default().fg(Theme::WARNING)),
                Span::raw("Download PDF"),
            ]),
            Line::from(vec![
                Span::styled(" [L] ", Style::default().fg(Theme::WARNING)),
                Span::raw("LaTeX Source"),
            ]),
            Line::from(vec![
                Span::styled(" [Q] ", Style::default().fg(Theme::WARNING)),
                Span::raw("Quit"),
            ]),
        ])
        .block(Block::default().borders(Borders::ALL).title(" ACTIONS "));
        frame.render_widget(actions, left_layout[1]);

        // Right column: Content
        let (_, title) = SECTIONS[self.selected_section];
        let content = Paragraph::new(self.get_section_content())
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(format!(" {} ", title)),
            )
            .wrap(Wrap { trim: true });
        frame.render_widget(content, columns[1]);
    }

    fn name(&self) -> &'static str {
        "Resume"
    }
}
