//! Command execution

use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
};

use super::{FSEntry, VirtualFS};
use crate::theme::Theme;
use crate::ui;

/// Result of executing a command
pub enum CommandResult {
    /// Regular output lines
    Output(Vec<Line<'static>>),
    /// Clear the screen
    Clear,
    /// Launch a sub-app (handled by main)
    AppLaunch(&'static str),
}

/// Execute a shell command and return the result
pub fn execute_command(cmd: &str, fs: &mut VirtualFS, cwd: &mut String) -> CommandResult {
    let parts: Vec<&str> = cmd.split_whitespace().collect();
    if parts.is_empty() {
        return CommandResult::Output(vec![]);
    }

    let command = parts[0];
    let args = &parts[1..];

    match command {
        "ls" => CommandResult::Output(cmd_ls(args, fs, cwd)),
        "cat" => CommandResult::Output(cmd_cat(args, fs, cwd)),
        "cd" => CommandResult::Output(cmd_cd(args, fs, cwd)),
        "pwd" => CommandResult::Output(vec![Line::from(cwd.clone())]),
        "clear" => CommandResult::Clear,
        "help" => CommandResult::Output(cmd_help()),
        "history" => CommandResult::Output(vec![Line::from("(history is stored in memory)")]),
        "whoami" => CommandResult::Output(vec![Line::from("guest")]),
        "uname" => CommandResult::Output(cmd_uname(args)),
        "echo" => CommandResult::Output(vec![Line::from(args.join(" "))]),
        "date" => CommandResult::Output(vec![Line::from("Wed Jan 29 10:30:00 CAT 2026")]),

        // Executables - launch as sub-apps
        "./dashboard" | "dashboard" => CommandResult::AppLaunch("dashboard"),
        "./resume" | "resume" => CommandResult::AppLaunch("resume"),
        "./contact" | "contact" => CommandResult::AppLaunch("contact"),
        "./about" | "about" => CommandResult::AppLaunch("about"),
        
        // Hidden command - still works but not shown in ls/help
        "eastereggs" => CommandResult::Output(cmd_eastereggs()),

        // Easter eggs
        "sudo" => CommandResult::Output(cmd_sudo(args)),
        "hack" => CommandResult::Output(cmd_hack()),
        "vim" | "nvim" => CommandResult::Output(cmd_vim()),
        "emacs" => CommandResult::Output(cmd_emacs()),
        "neofetch" => CommandResult::Output(cmd_neofetch()),
        "cowsay" => CommandResult::Output(cmd_cowsay(args)),
        "fortune" => CommandResult::Output(cmd_fortune()),
        "matrix" => CommandResult::Output(cmd_matrix()),
        "sl" => CommandResult::Output(cmd_sl()),
        "rm" => CommandResult::Output(cmd_rm(args)),

        // Unknown command
        _ => CommandResult::Output(vec![ui::error_text(&format!(
            "rsh: command not found: {}",
            command
        ))]),
    }
}

fn cmd_ls(args: &[&str], fs: &VirtualFS, cwd: &str) -> Vec<Line<'static>> {
    let show_all = args.contains(&"-la") || args.contains(&"-a") || args.contains(&"-l");
    let path = args.iter().find(|a| !a.starts_with('-')).unwrap_or(&".");
    let resolved = fs.resolve_path(path, cwd);

    match fs.list_dir(&resolved) {
        Some(entries) => {
            let mut lines = Vec::new();

            if show_all {
                lines.push(Line::from(format!("total {}", entries.len())));
            }

            for (name, entry) in entries {
                if !show_all && name.starts_with('.') {
                    continue;
                }

                let line = match entry {
                    FSEntry::Directory { .. } => Line::from(Span::styled(
                        format!("{}/", name),
                        Style::default().fg(Theme::PRIMARY),
                    )),
                    FSEntry::Executable { size, .. } => {
                        if show_all {
                            Line::from(vec![
                                Span::raw("-rwxr-xr-x  1 uchindami  uchindami  "),
                                Span::raw(format!("{:>5}K ", size / 1000)),
                                Span::styled(name, Style::default().fg(Theme::SUCCESS)),
                            ])
                        } else {
                            Line::from(Span::styled(name, Style::default().fg(Theme::SUCCESS)))
                        }
                    }
                    FSEntry::File { size, permissions, .. } => {
                        if show_all {
                            Line::from(vec![
                                Span::raw(format!("{}  1 uchindami  uchindami  ", permissions)),
                                Span::raw(format!("{:>5} ", size)),
                                Span::raw(name),
                            ])
                        } else {
                            Line::from(name)
                        }
                    }
                };
                lines.push(line);
            }

            lines
        }
        None => vec![Line::from(Span::styled(
            format!("ls: cannot access '{}': No such file or directory", path),
            Style::default().fg(Theme::ERROR),
        ))],
    }
}

fn cmd_cat(args: &[&str], fs: &VirtualFS, cwd: &str) -> Vec<Line<'static>> {
    if args.is_empty() {
        return vec![Line::from("cat: missing operand")];
    }

    let path = fs.resolve_path(args[0], cwd);
    match fs.get(&path) {
        Some(FSEntry::File { content, .. }) => {
            content.lines().map(|l: &str| Line::from(l.to_string())).collect()
        }
        Some(FSEntry::Directory { .. }) => vec![Line::from(Span::styled(
            format!("cat: {}: Is a directory", args[0]),
            Style::default().fg(Theme::ERROR),
        ))],
        Some(FSEntry::Executable { .. }) => vec![Line::from(Span::styled(
            format!("cat: {}: Is an executable (try running it with ./{})", args[0], args[0]),
            Style::default().fg(Theme::WARNING),
        ))],
        None => vec![Line::from(Span::styled(
            format!("cat: {}: No such file or directory", args[0]),
            Style::default().fg(Theme::ERROR),
        ))],
    }
}

fn cmd_cd(args: &[&str], fs: &VirtualFS, cwd: &mut String) -> Vec<Line<'static>> {
    let target = args.first().unwrap_or(&"~");

    // Handle special cases
    let new_path = match *target {
        "~" | "" => "~".to_string(),
        ".." => {
            if *cwd == "~" {
                "~".to_string()
            } else {
                let parts: Vec<&str> = cwd.rsplitn(2, '/').collect();
                if parts.len() > 1 { parts[1].to_string() } else { "~".to_string() }
            }
        }
        "." => cwd.clone(),
        path => fs.resolve_path(path, cwd),
    };

    // Check if directory exists
    match fs.get(&new_path) {
        Some(FSEntry::Directory { .. }) => {
            *cwd = new_path;
            vec![]
        }
        Some(_) => vec![Line::from(Span::styled(
            format!("cd: not a directory: {}", target),
            Style::default().fg(Theme::ERROR),
        ))],
        None => vec![Line::from(Span::styled(
            format!("cd: no such file or directory: {}", target),
            Style::default().fg(Theme::ERROR),
        ))],
    }
}

fn cmd_help() -> Vec<Line<'static>> {
    vec![
        Line::from(""),
        ui::heading("Available Commands:"),
        Line::from(""),
        Line::from("  ls [-la]       List directory contents"),
        Line::from("  cd <dir>       Change directory"),
        Line::from("  cat <file>     Display file contents"),
        Line::from("  pwd            Print working directory"),
        Line::from("  clear          Clear screen"),
        Line::from("  help           Show this help"),
        Line::from("  exit           Exit portfolio"),
        Line::from(""),
        ui::heading("Executables:"),
        Line::from(""),
        Line::from("  ./dashboard    Interactive dashboard"),
        Line::from("  ./resume       View CV/resume (Skills, Projects, Experience)"),
        Line::from("  ./contact      Contact information"),
        Line::from("  ./about        About me"),
        Line::from(""),
    ]
}

fn cmd_run_app(name: &str) -> Vec<Line<'static>> {
    vec![
        Line::from(""),
        Line::from(format!("Launching {}...", name)),
        Line::from(Span::styled(
            "[Coming soon - sub-TUI apps in Phase 2]",
            Style::default().fg(Theme::WARNING),
        )),
        Line::from(""),
    ]
}

fn cmd_uname(args: &[&str]) -> Vec<Line<'static>> {
    if args.contains(&"-a") {
        vec![Line::from("PortfolioOS 2.0.26-uchindami #1 SMP Rust x86_64 GNU/Rust")]
    } else {
        vec![Line::from("PortfolioOS")]
    }
}

fn cmd_sudo(args: &[&str]) -> Vec<Line<'static>> {
    if args.join(" ").contains("rm -rf") {
        vec![
            Line::from(""),
            Line::from(Span::styled(
                "Nice try! This is a sandboxed environment 😉",
                Style::default().fg(Theme::WARNING),
            )),
            Line::from(""),
        ]
    } else {
        vec![Line::from(Span::styled(
            "guest is not in the sudoers file. This incident will be reported.",
            Style::default().fg(Theme::ERROR),
        ))]
    }
}

fn cmd_hack() -> Vec<Line<'static>> {
    vec![
        Line::from(""),
        Line::from("   ACCESSING MAINFRAME..."),
        Line::from("   [████████████████████] 100%"),
        Line::from(""),
        Line::from(Span::styled(
            "   Just kidding! But I like your style 😎",
            Style::default().fg(Theme::SUCCESS),
        )),
        Line::from(""),
    ]
}

fn cmd_vim() -> Vec<Line<'static>> {
    vec![
        Line::from(""),
        Line::from("   VIM - Vi IMproved"),
        Line::from("   version 8.2.0"),
        Line::from(""),
        Line::from(Span::styled(
            "   BTW, I use Neovim 💚",
            Style::default().fg(Theme::SUCCESS),
        )),
        Line::from(""),
    ]
}

fn cmd_neofetch() -> Vec<Line<'static>> {
    vec![
        Line::from(""),
        Line::from(Span::styled("       _,met$$$$$gg.          ", Style::default().fg(Theme::SECONDARY))),
        Line::from(Span::styled("    ,g$$$$$$$$$$$$$$$P.       ", Style::default().fg(Theme::SECONDARY))),
        Line::from(Span::styled("  ,g$$P\"     \"\"\"Y$$.\".        ", Style::default().fg(Theme::SECONDARY))),
        Line::from(Span::styled(" ,$$P'              `$$$.     ", Style::default().fg(Theme::SECONDARY))),
        Line::from(Span::styled("',$$P       ,ggs.     `$$b:   guest@uchindami", Style::default().fg(Theme::SECONDARY))),
        Line::from(Span::styled("`d$$'     ,$P\"'   .    $$$    ----------------", Style::default().fg(Theme::SECONDARY))),
        Line::from(Span::styled(" $$P      d$'     ,    $$P    OS: Portfolio OS v2.0.26", Style::default().fg(Theme::SECONDARY))),
        Line::from(Span::styled(" $$:      $$.   -    ,d$$'    Kernel: Rust 1.83.0", Style::default().fg(Theme::SECONDARY))),
        Line::from(Span::styled(" $$;      Y$b._   _,d$P'      Shell: rsh", Style::default().fg(Theme::SECONDARY))),
        Line::from(Span::styled(" Y$$.    `.`\"Y$$$$P\"'         Terminal: ttyd", Style::default().fg(Theme::SECONDARY))),
        Line::from(Span::styled(" `$$b      \"-.__              Framework: Ratatui", Style::default().fg(Theme::SECONDARY))),
        Line::from(""),
    ]
}

fn cmd_cowsay(args: &[&str]) -> Vec<Line<'static>> {
    let msg = if args.is_empty() { "Moo!" } else { &args.join(" ") };
    let border = "-".repeat(msg.len() + 2);
    vec![
        Line::from(format!(" {}", border)),
        Line::from(format!("< {} >", msg)),
        Line::from(format!(" {}", border)),
        Line::from("        \\   ^__^"),
        Line::from("         \\  (oo)\\_______"),
        Line::from("            (__)\\       )\\/\\"),
        Line::from("                ||----w |"),
        Line::from("                ||     ||"),
    ]
}

fn cmd_eastereggs() -> Vec<Line<'static>> {
    vec![
        Line::from(""),
        Line::from(Span::styled("🥚 Easter Eggs 🥚", Style::default().fg(Theme::WARNING))),
        Line::from(""),
        Line::from("Try some of these hidden commands:"),
        Line::from(""),
        Line::from("  • cat .secret         - Find a hidden secret"),
        Line::from("  • sudo rm -rf /       - Nice try..."),
        Line::from("  • hack                - Hack the mainframe"),
        Line::from("  • vim / emacs         - Editor wars"),
        Line::from("  • neofetch            - System info"),
        Line::from("  • cowsay <message>    - Moo!"),
        Line::from("  • fortune             - Get your fortune"),
        Line::from("  • matrix              - Enter the matrix"),
        Line::from("  • sl                  - Missing train?"),
        Line::from(""),
        Line::from("More surprises await... keep exploring! 🔍"),
        Line::from(""),
    ]
}

fn cmd_emacs() -> Vec<Line<'static>> {
    vec![
        Line::from(""),
        Line::from("   GNU Emacs 29.1"),
        Line::from("   Copyright (C) 2024 Free Software Foundation"),
        Line::from(""),
        Line::from(Span::styled(
            "   Real programmers use ed. Or maybe vim. 🔥",
            Style::default().fg(Theme::WARNING),
        )),
        Line::from(""),
    ]
}

fn cmd_fortune() -> Vec<Line<'static>> {
    let fortunes = [
        "A bug in the code is worth two in documentation.",
        "Your code will compile on the first try today. (Just kidding)",
        "The best time to document your code was yesterday. The second best is never.",
        "Today you will discover a missing semicolon that took 3 hours to find.",
        "A commit a day keeps the merge conflicts away.",
        "You will refactor that function 'later'. You won't.",
        "The cloud is just someone else's computer.",
        "There are only 10 types of people: those who understand binary and those who don't.",
    ];
    // Use a simple "random" based on current line count
    let idx = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as usize % fortunes.len())
        .unwrap_or(0);
    vec![
        Line::from(""),
        Line::from(Span::styled("🔮 Your fortune:", Style::default().fg(Theme::SECONDARY))),
        Line::from(""),
        Line::from(format!("   {}", fortunes[idx])),
        Line::from(""),
    ]
}

fn cmd_matrix() -> Vec<Line<'static>> {
    vec![
        Line::from(""),
        Line::from(Span::styled("   ░▒▓█ THE MATRIX █▓▒░", Style::default().fg(Theme::SUCCESS))),
        Line::from(""),
        Line::from(Span::styled("   Wake up, Neo...", Style::default().fg(Theme::SUCCESS))),
        Line::from(Span::styled("   The Matrix has you...", Style::default().fg(Theme::SUCCESS))),
        Line::from(Span::styled("   Follow the white rabbit.", Style::default().fg(Theme::SUCCESS))),
        Line::from(""),
        Line::from("   01001000 01100101 01101100 01101100 01101111"),
        Line::from(""),
    ]
}

fn cmd_sl() -> Vec<Line<'static>> {
    vec![
        Line::from(""),
        Line::from("                          (  ) (@@) ( )  (@)  ()    @@    O     @     O"),
        Line::from("                     (@@@)"),
        Line::from("                 (    )"),
        Line::from("              (@@@@)"),
        Line::from("            (   )"),
        Line::from("        ====        ________                ___________"),
        Line::from("    _D _|  |_______/        \\__I_I_____===__|_________|"),
        Line::from("     |(_)---  |   H\\________/ |   |        =|___ ___|"),
        Line::from("     /     |  |   H  |  |     |   |         ||_| |_||"),
        Line::from("    |      |  |   H  |__--------------------| [___] |"),
        Line::from("    | ________|___H__/__|_____/[][]~\\_______|       |"),
        Line::from("    |/ |   |-----------I_____I [][] []  D   |=======|"),
        Line::from(""),
        Line::from(Span::styled("   🚂 You've been hit by a smooth criminal train!", Style::default().fg(Theme::WARNING))),
        Line::from(""),
    ]
}

fn cmd_rm(args: &[&str]) -> Vec<Line<'static>> {
    if args.join(" ").contains("-rf") || args.contains(&"-r") {
        vec![
            Line::from(""),
            Line::from(Span::styled(
                "⚠️  rm: refusing to remove '/' recursively",
                Style::default().fg(Theme::WARNING),
            )),
            Line::from(Span::styled(
                "   This is a sandboxed portfolio, nice try! 😉",
                Style::default().fg(Theme::MUTED),
            )),
            Line::from(""),
        ]
    } else {
        vec![Line::from(Span::styled(
            "rm: cannot remove: Read-only file system",
            Style::default().fg(Theme::ERROR),
        ))]
    }
}

