🎯 Shell-Based Portfolio TUI Design
Connecting to uchindami.dev...
Authenticated.

┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
┃                                                                            ┃
┃   ██╗   ██╗ ██████╗██╗  ██╗██╗███╗   ██╗██████╗  █████╗ ███╗   ███╗██╗   ┃
┃   ██║   ██║██╔════╝██║  ██║██║████╗  ██║██╔══██╗██╔══██╗████╗ ████║██║   ┃
┃   ██║   ██║██║     ███████║██║██╔██╗ ██║██║  ██║███████║██╔████╔██║██║   ┃
┃   ██║   ██║██║     ██╔══██║██║██║╚██╗██║██║  ██║██╔══██║██║╚██╔╝██║██║   ┃
┃   ╚██████╔╝╚██████╗██║  ██║██║██║ ╚████║██████╔╝██║  ██║██║ ╚═╝ ██║██║   ┃
┃    ╚═════╝  ╚═════╝╚═╝  ╚═╝╚═╝╚═╝  ╚═══╝╚═════╝ ╚═╝  ╚═╝╚═╝     ╚═╝╚═╝   ┃
┃                                                                            ┃
┃                    Welcome to my interactive portfolio                    ┃
┃                                                                            ┃
┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛

System Information:
  OS: Portfolio OS v2.0.26
  Kernel: Rust 1.83.0
  Uptime: 420 days, 69 hours
  Shell: rsh (Rust Shell)

Type 'ls' to see available commands, or 'help' for assistance.

guest@uchindami:~$ _

System Information:
  OS: Portfolio OS v2.0.26
  Kernel: Rust 1.83.0
  Uptime: 420 days, 69 hours
  Shell: rsh (Rust Shell)

Type 'ls' to see available commands, or 'help' for assistance.

guest@uchindami:~$ _

After typing ls
bashguest@uchindami:~$ ls
total 7
drwxr-xr-x  2 uchindami  uchindami   512 Jan 29 15:30 .
drwxr-xr-x  5 root       root        512 Jan 01 00:00 ..
-rw-r--r--  1 uchindami  uchindami   220 Jan 15 12:00 .profile
-rw-r--r--  1 uchindami  uchindami  3.2K Jan 29 09:15 README.md
-rwxr-xr-x  1 uchindami  uchindami   42K Jan 28 14:20 about
-rwxr-xr-x  1 uchindami  uchindami   38K Jan 29 15:30 dashboard
-rwxr-xr-x  1 uchindami  uchindami   51K Jan 27 11:45 projects
-rwxr-xr-x  1 uchindami  uchindami   29K Jan 26 16:00 skills
-rwxr-xr-x  1 uchindami  uchindami   33K Jan 25 10:30 experience
-rwxr-xr-x  1 uchindami  uchindami   45K Jan 29 08:00 resume
-rwxr-xr-x  1 uchindami  uchindami   21K Jan 24 14:15 contact
drwxr-xr-x  4 uchindami  uchindami   512 Jan 20 09:00 blog/
drwxr-xr-x  3 uchindami  uchindami   512 Jan 18 13:25 downloads/
-rwxr-xr-x  1 uchindami  uchindami   15K Jan 15 11:00 eastereggs

guest@uchindami:~$ _

Command Structure & Behavior
Core Shell Commands
bash# Navigation
ls              # List files/executables
ls -la          # Detailed listing with hidden files
cd <dir>        # Change directory
pwd             # Print working directory
cat <file>      # View file contents
clear           # Clear screen
help            # Show help
exit / quit     # Exit portfolio

Smart aliases

home → cd /

back → cd ..

# Portfolio "Executables"
./dashboard     # Launch interactive dashboard TUI
./resume        # Launch resume viewer (CV)
./about         # Launch about me app
./projects      # Launch projects browser
./skills        # Launch skills matrix
./experience    # Timeline viewer
./contact       # Contact card with QR codes
./eastereggs    # Fun stuff

Example: cat README.md
bashguest@uchindami:~$ cat README.md

# Welcome to Uchindami's Portfolio! 👋

Hey there! You've just ssh'd into my interactive terminal portfolio.
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

guest@uchindami:~$ _

Example: Running ./dashboard
bashguest@uchindami:~$ ./dashboard

Launching Dashboard v1.0...
Initializing UI components... ✓
Loading data... ✓

╔══════════════════════════════════════════════════════════════════════════════╗
║                          UCHINDAMI - DASHBOARD                               ║
╠══════════════════════════════════════════════════════════════════════════════╣
║                                                                              ║
║  ┌─ QUICK STATS ────────────────────────────────────────────────────────┐   ║
║  │  👨‍💻 Software Engineer  |  📍 Location  |  🎯 5+ Years Experience      │   ║
║  │  ⭐ 127 GitHub Stars    |  🚀 23 Projects  |  📝 42 Blog Posts         │   ║
║  └──────────────────────────────────────────────────────────────────────┘   ║
║                                                                              ║
║  ┌────────────────────┐  ┌────────────────────┐  ┌────────────────────┐   ║
║  │ [1] 📋 RESUME      │  │ [2] 🚀 PROJECTS    │  │ [3] 💡 SKILLS      │   ║
║  │                    │  │                    │  │                    │   ║
║  │ Download PDF       │  │ 23 Projects        │  │ Rust, Go, Python   │   ║
║  │ View Online        │  │ 12 Open Source     │  │ Cloud Native       │   ║
║  │ LaTeX Source       │  │ Featured Works     │  │ System Design      │   ║
║  └────────────────────┘  └────────────────────┘  └────────────────────┘   ║
║                                                                              ║
║  ┌────────────────────┐  ┌────────────────────┐  ┌────────────────────┐   ║
║  │ [4] 💼 EXPERIENCE  │  │ [5] 📞 CONTACT     │  │ [6] 📚 BLOG        │   ║
║  │                    │  │                    │  │                    │   ║
║  │ Senior Engineer    │  │ Email, GitHub      │  │ Latest Posts       │   ║
║  │ Tech Lead          │  │ LinkedIn, Twitter  │  │ Tech Tutorials     │   ║
║  │ Timeline View      │  │ QR Codes           │  │ Case Studies       │   ║
║  └────────────────────┘  └────────────────────┘  └────────────────────┘   ║
║                                                                              ║
╠══════════════════════════════════════════════════════════════════════════════╣
║ [1-6] Select  [Q]uit  [H]elp  [/]Search  [Tab]Navigate                      ║
╚══════════════════════════════════════════════════════════════════════════════╝

dashboard> _
Dashboard Commands:

Press 1-6 or use arrow keys + Enter to navigate
q to exit back to shell
Has its own internal command mode


Example: Running ./resume
bashguest@uchindami:~$ ./resume

Loading resume...

╔══════════════════════════════════════════════════════════════════════════════╗
║                              CURRICULUM VITAE                                ║
║                                UCHINDAMI                                     ║
╠═══════════════════════╦══════════════════════════════════════════════════════╣
║                       ║                                                      ║
║ ┌─ SECTIONS ────────┐ ║  ┌─ PROFESSIONAL SUMMARY ─────────────────────────┐ ║
║ │                   │ ║  │                                                 │ ║
║ │ ▶ Summary         │ ║  │  Passionate software engineer with 5+ years     │ ║
║ │   Experience      │ ║  │  of experience building scalable systems and    │ ║
║ │   Education       │ ║  │  leading engineering teams.                     │ ║
║ │   Skills          │ ║  │                                                 │ ║
║ │   Projects        │ ║  │  Specializations:                               │ ║
║ │   Certifications  │ ║  │  • Backend systems & microservices              │ ║
║ │   Languages       │ ║  │  • Cloud infrastructure (GCP, AWS)              │ ║
║ │   Download        │ ║  │  • Developer tooling & CLI applications         │ ║
║ └───────────────────┘ ║  │  • Rust, Go, Python, TypeScript                 │ ║
║                       ║  │                                                 │ ║
║ ┌─ ACTIONS ─────────┐ ║  │  Core Values:                                   │ ║
║ │                   │ ║  │  → Writing clean, maintainable code             │ ║
║ │ [D] Download PDF  │ ║  │  → Building developer-first tools               │ ║
║ │ [L] LaTeX Source  │ ║  │  → Open source contribution                     │ ║
║ │ [E] Export JSON   │ ║  │  → Continuous learning                          │ ║
║ │ [P] Print View    │ ║  │                                                 │ ║
║ │ [Q] Quit          │ ║  └─────────────────────────────────────────────────┘ ║
║ └───────────────────┘ ║                                                      ║
║                       ║                                                      ║
╠═══════════════════════╩══════════════════════════════════════════════════════╣
║ [↑↓]Navigate  [Enter]Select  [Q]uit  [D]ownload PDF  [/]Search              ║
╚══════════════════════════════════════════════════════════════════════════════╝

resume> _

Directory Structure
bash~/
├── .profile          # Shell config
├── .bashrc           # More config (easter egg if user cats it)
├── README.md         # Welcome guide
├── about*            # Executable: About me TUI
├── dashboard*        # Executable: Dashboard TUI
├── resume*           # Executable: CV/Resume viewer
├── projects*         # Executable: Projects browser
├── skills*           # Executable: Skills matrix
├── experience*       # Executable: Timeline viewer
├── contact*          # Executable: Contact card
├── eastereggs*       # Executable: Fun stuff
├── blog/
│   ├── 2026/
│   │   ├── building-portfolio-in-rust.md
│   │   └── mastering-async-rust.md
│   └── 2025/
│       └── ...
└── downloads/
    ├── resume.pdf
    ├── resume.tex
    └── projects/
        └── sample-code.rs

Special Shell Features
Tab Completion
bashguest@uchindami:~$ ./pro[TAB]
guest@uchindami:~$ ./projects
Command History (↑/↓ arrows)
bashguest@uchindami:~$ history
  1  ls
  2  cat README.md
  3  ./dashboard
  4  ls -la
  5  ./resume
File Operations
bash# View blog posts
guest@uchindami:~$ ls blog/2026/
building-portfolio-in-rust.md
mastering-async-rust.md

guest@uchindami:~$ cat blog/2026/building-portfolio-in-rust.md
# Building a Portfolio in Rust
...

# Download files
guest@uchindami:~$ cp downloads/resume.pdf ~
# (Triggers browser download)

Easter Eggs Examples
bash# Hidden commands
guest@uchindami:~$ ls -la
-rw-r--r--  1 uchindami  uchindami   180 Jan 15 12:00 .secret

guest@uchindami:~$ cat .secret
You found a secret! Here's a cookie: 🍪

# Fake commands
guest@uchindami:~$ sudo rm -rf /
Nice try! This is a sandboxed environment 😉

guest@uchindami:~$ hack
        ⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⣀⣤⣤⣤⣤⣤⣤⣀⣀⠀⠀⠀⠀⠀⠀⠀⠀⠀
        ⠀⠀⠀⠀⠀⢀⣤⣶⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣶⣤⡀⠀⠀⠀⠀⠀
   ACCESSING MAINFRAME...
   [████████████████████] 100%
   
   Just kidding! But I like your style 😎

guest@uchindami:~$ vim
        VIM - Vi IMproved
        
        version 8.2.0
        
   BTW, I use Neovim

# System info
guest@uchindami:~$ uname -a
PortfolioOS 2.0.26-uchindami #1 SMP Rust x86_64 GNU/Rust