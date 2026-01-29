//! Virtual filesystem for the shell

use std::collections::HashMap;

/// Entry in the virtual filesystem
#[derive(Clone)]
pub enum FSEntry {
    File {
        content: String,
        size: usize,
        permissions: &'static str,
    },
    Directory {
        children: Vec<String>,
    },
    Executable {
        description: &'static str,
        size: usize,
    },
}

/// Virtual filesystem
pub struct VirtualFS {
    entries: HashMap<String, FSEntry>,
}

impl VirtualFS {
    pub fn new() -> Self {
        let mut entries = HashMap::new();

        // Root directory
        entries.insert(
            "~".to_string(),
            FSEntry::Directory {
                children: vec![
                    ".profile".to_string(),
                    ".secret".to_string(),
                    "README.md".to_string(),
                    "about".to_string(),
                    "dashboard".to_string(),
                    "resume".to_string(),
                    "contact".to_string(),
                    "blog".to_string(),
                    "downloads".to_string(),
                ],
            },
        );

        // Files
        entries.insert(
            "~/.profile".to_string(),
            FSEntry::File {
                content: "# Uchindami's Shell Profile\nexport EDITOR=nvim\nexport LANG=en_US.UTF-8\n".to_string(),
                size: 220,
                permissions: "-rw-r--r--",
            },
        );

        entries.insert(
            "~/.secret".to_string(),
            FSEntry::File {
                content: "You found a secret! Here's a cookie: 🍪\n\nP.S. Try running 'hack' or 'sudo rm -rf /' 😉".to_string(),
                size: 180,
                permissions: "-rw-------",
            },
        );

        entries.insert(
            "~/README.md".to_string(),
            FSEntry::File {
                content: r#"# Welcome to Uchindami's Portfolio! 👋

Hey there! You've just connected to my interactive terminal portfolio.
This isn't your typical portfolio website - it's a fully functional
shell environment where you can explore my work like a real filesystem.

## Quick Start

- `ls` - See all available commands
- `./dashboard` - Launch the interactive dashboard
- `./resume` - View my CV/resume
- `./projects` - Browse my projects
- `help` - Get detailed help

## Easter Eggs 🥚

I've hidden some fun surprises throughout. Try exploring different
commands and see what you find!

## Tech Stack

This portfolio is built with:
- Rust + Ratatui (TUI framework)
- Hosted on Google Cloud Run
- Accessible via web terminal (ttyd)

Enjoy exploring!
"#.to_string(),
                size: 3200,
                permissions: "-rw-r--r--",
            },
        );

        // Executables
        entries.insert(
            "~/about".to_string(),
            FSEntry::Executable {
                description: "About me TUI",
                size: 42000,
            },
        );
        entries.insert(
            "~/dashboard".to_string(),
            FSEntry::Executable {
                description: "Interactive dashboard",
                size: 38000,
            },
        );
        entries.insert(
            "~/resume".to_string(),
            FSEntry::Executable {
                description: "CV/Resume viewer",
                size: 45000,
            },
        );
        entries.insert(
            "~/contact".to_string(),
            FSEntry::Executable {
                description: "Contact card",
                size: 21000,
            },
        );

        // Subdirectories
        entries.insert(
            "~/blog".to_string(),
            FSEntry::Directory {
                children: vec!["2026".to_string(), "2025".to_string()],
            },
        );
        entries.insert(
            "~/blog/2026".to_string(),
            FSEntry::Directory {
                children: vec![
                    "building-portfolio-in-rust.md".to_string(),
                    "mastering-async-rust.md".to_string(),
                ],
            },
        );
        entries.insert(
            "~/blog/2026/building-portfolio-in-rust.md".to_string(),
            FSEntry::File {
                content: "# Building a Portfolio in Rust\n\nComing soon...".to_string(),
                size: 1200,
                permissions: "-rw-r--r--",
            },
        );

        entries.insert(
            "~/downloads".to_string(),
            FSEntry::Directory {
                children: vec!["resume.pdf".to_string(), "resume.tex".to_string()],
            },
        );

        Self { entries }
    }

    /// Get entry at path
    pub fn get(&self, path: &str) -> Option<&FSEntry> {
        self.entries.get(path)
    }

    /// Resolve relative path to absolute
    pub fn resolve_path(&self, path: &str, cwd: &str) -> String {
        if path.starts_with("~/") || path == "~" {
            path.to_string()
        } else if path.starts_with("./") {
            format!("{}/{}", cwd, &path[2..])
        } else if path == ".." {
            if cwd == "~" {
                "~".to_string()
            } else {
                let parts: Vec<&str> = cwd.rsplitn(2, '/').collect();
                if parts.len() > 1 {
                    parts[1].to_string()
                } else {
                    "~".to_string()
                }
            }
        } else if path == "." {
            cwd.to_string()
        } else {
            format!("{}/{}", cwd, path)
        }
    }

    /// List directory contents
    pub fn list_dir(&self, path: &str) -> Option<Vec<(String, &FSEntry)>> {
        if let Some(FSEntry::Directory { children }) = self.get(path) {
            let mut result = Vec::new();
            for child in children {
                let child_path = format!("{}/{}", path, child);
                if let Some(entry) = self.get(&child_path) {
                    result.push((child.clone(), entry));
                }
            }
            Some(result)
        } else {
            None
        }
    }

    /// Tab completion
    pub fn complete(&self, input: &str, cwd: &str) -> Vec<String> {
        let mut completions = Vec::new();

        // Get the part after the last space (the argument being completed)
        let parts: Vec<&str> = input.rsplitn(2, ' ').collect();
        let (prefix, partial) = if parts.len() == 2 {
            (parts[1], parts[0])
        } else {
            ("", parts[0])
        };

        // List current directory and find matches
        if let Some(entries) = self.list_dir(cwd) {
            for (name, entry) in entries {
                let display_name = match entry {
                    FSEntry::Directory { .. } => format!("{}/", name),
                    FSEntry::Executable { .. } => format!("./{}", name),
                    FSEntry::File { .. } => name.clone(),
                };

                if display_name.starts_with(partial) || name.starts_with(partial) {
                    let completion = if prefix.is_empty() {
                        display_name
                    } else {
                        format!("{} {}", prefix, display_name)
                    };
                    completions.push(completion);
                }
            }
        }

        completions.sort();
        completions
    }
}
