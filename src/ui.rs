//! Reusable UI components with themed styling
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

use crate::theme::Theme;

/// Creates a themed block with title
pub fn themed_block<'a>(title: &'a str) -> Block<'a> {
    Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Theme::BORDER))
        .title(format!(" {} ", title))
        .title_style(Style::default().fg(Theme::SECONDARY).add_modifier(Modifier::BOLD))
}

/// Creates a themed block without title
pub fn themed_block_plain<'a>() -> Block<'a> {
    Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Theme::BORDER))
}

/// Creates a footer line with key bindings
pub fn footer_keybinds<'a>(bindings: &[(&'a str, &'a str)]) -> Line<'a> {
    let mut spans = vec![Span::raw(" ")];
    
    for (i, (key, desc)) in bindings.iter().enumerate() {
        if i > 0 {
            spans.push(Span::raw("  "));
        }
        spans.push(Span::styled(
            format!("[{}] ", key),
            Style::default().fg(Theme::WARNING),
        ));
        spans.push(Span::styled(*desc, Style::default().fg(Theme::FOREGROUND)));
    }
    
    Line::from(spans)
}

/// Creates a heading line
pub fn heading(text: &str) -> Line<'static> {
    Line::from(Span::styled(
        text.to_string(),
        Style::default()
            .fg(Theme::SECONDARY)
            .add_modifier(Modifier::BOLD),
    ))
}

/// Creates a subheading line
pub fn subheading(text: &str) -> Line<'static> {
    Line::from(Span::styled(
        text.to_string(),
        Style::default()
            .fg(Theme::ACCENT)
            .add_modifier(Modifier::BOLD),
    ))
}

/// Creates a muted/gray text line
pub fn muted_text(text: &str) -> Line<'static> {
    Line::from(Span::styled(
        text.to_string(),
        Style::default().fg(Theme::MUTED),
    ))
}

/// Creates an error message line
pub fn error_text(text: &str) -> Line<'static> {
    Line::from(Span::styled(
        text.to_string(),
        Style::default().fg(Theme::ERROR),
    ))
}

/// Creates a success message line
pub fn success_text(text: &str) -> Line<'static> {
    Line::from(Span::styled(
        text.to_string(),
        Style::default().fg(Theme::SUCCESS),
    ))
}

/// Creates a warning message line
pub fn warning_text(text: &str) -> Line<'static> {
    Line::from(Span::styled(
        text.to_string(),
        Style::default().fg(Theme::WARNING),
    ))
}

/// Creates a tag/badge span
pub fn tag(text: &str) -> Span<'static> {
    Span::styled(
        format!(" {} ", text),
        Style::default()
            .fg(Theme::BACKGROUND)
            .bg(Theme::ACCENT),
    )
}

/// Creates a themed list item
pub fn list_item(text: &str, selected: bool) -> ListItem<'static> {
    let style = if selected {
        Style::default()
            .fg(Theme::SELECTED_FG)
            .bg(Theme::SELECTED_BG)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Theme::FOREGROUND)
    };
    
    ListItem::new(Line::from(Span::styled(text.to_string(), style)))
}

/// Creates a themed list item with multiple lines
pub fn list_item_multi(lines: Vec<Line<'static>>, selected: bool) -> ListItem<'static> {
    if selected {
        let styled_lines: Vec<Line> = lines
            .into_iter()
            .map(|mut line| {
                for span in &mut line.spans {
                    span.style = Style::default()
                        .fg(Theme::SELECTED_FG)
                        .bg(Theme::SELECTED_BG);
                }
                line
            })
            .collect();
        ListItem::new(styled_lines)
    } else {
        ListItem::new(lines)
    }
}

/// Style for selected items
pub fn selected_style() -> Style {
    Style::default()
        .fg(Theme::SELECTED_FG)
        .bg(Theme::SELECTED_BG)
        .add_modifier(Modifier::BOLD)
}

/// Style for normal text
pub fn normal_style() -> Style {
    Style::default().fg(Theme::FOREGROUND)
}

// =============================================================================
// UI UTILITIES
// =============================================================================

/// Center text within a given width
pub fn center_text(text: &str, width: usize) -> String {
    let text_len = text.chars().count();
    if text_len >= width {
        text.to_string()
    } else {
        let padding = (width - text_len) / 2;
        format!("{}{}", " ".repeat(padding), text)
    }
}

