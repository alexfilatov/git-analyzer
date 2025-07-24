# 🔍 Git Repository Analyzer

A powerful command-line tool written in Rust that provides comprehensive analysis of Git repositories, including contributor patterns, activity insights, and file change statistics.

## ✨ Features

### 👥 Contributor Analysis
- **Top Contributors**: Ranked by commit count with detailed statistics
- **Work Pattern Classification**: Identify moonlighters vs day workers vs mixed patterns
- **Confidence Scoring**: Reliability measure for work pattern classifications (0-100%)
- **Time-based Insights**: First/last commit dates and hourly distribution
- **Smart Categorization**:
  - ☀️ **Day Worker**: Primarily commits during business hours (9 AM - 6 PM)
  - 🌙 **Moonlighter**: Commits mostly evenings/nights and weekends
  - ⚖️ **Mixed**: Balanced between day and night work
  - ❓ **Unknown**: Insufficient data (< 5 commits)

### 📊 Activity Pattern Analysis
- **Monthly Trends**: Commit activity over time
- **Hourly Distribution**: When commits happen throughout the day
- **Weekend vs Weekday**: Work-life balance insights
- **Team Rhythm**: Understanding peak activity periods

### 📁 File Change Analysis
- **Most Modified Files**: Identify hotspots in your codebase
- **Change Frequency**: How often files are being modified
- **Last Modified**: Recent activity tracking
- **Maintenance Insights**: Files that need attention

### 🌐 Remote Repository Support
- **Clone & Analyze**: Works with any Git URL (GitHub, GitLab, Bitbucket, etc.)
- **Progress Display**: Real-time cloning progress with percentage completion
- **Delta Resolution**: Shows progress during Git object processing
- **Temporary Storage**: Automatic cleanup after analysis

### 📄 Multiple Output Formats
- **Human-readable**: Beautiful ASCII tables with proper alignment
- **JSON Output**: Structured data for further processing or integration
- **Comprehensive Reports**: All analyses combined in one command

## 🚀 Installation

### Prerequisites
- Rust 1.70+ (2024 edition)
- Git (for repository operations)

### Build from Source
```bash
git clone <your-repo-url>
cd git-analyzer
cargo build --release
```

## 📖 Usage

### Basic Commands

#### Analyze Contributors
```bash
# Local repository
cargo run -- contributors --path /path/to/repo

# Remote repository
cargo run -- contributors --url https://github.com/user/repo.git

# JSON output
cargo run -- contributors --url https://github.com/user/repo.git --json
```

#### Activity Patterns
```bash
# Local repository
cargo run -- activity --path /path/to/repo

# Remote repository with JSON
cargo run -- activity --url https://github.com/rust-lang/rust.git --json
```

#### File Analysis
```bash
# Most modified files
cargo run -- files --url https://github.com/torvalds/linux.git

# JSON format
cargo run -- files --path . --json
```

#### Complete Analysis
```bash
# Everything at once
cargo run -- all --url https://github.com/microsoft/vscode.git

# Local repo with JSON output
cargo run -- all --path . --json
```

### Command-Line Options

| Flag | Description | Example |
|------|-------------|---------|
| `--path` / `-p` | Local repository path | `--path /home/user/myproject` |
| `--url` / `-u` | Remote repository URL | `--url https://github.com/user/repo.git` |
| `--json` / `-j` | Output in JSON format | `--json` |
| `--help` / `-h` | Show help information | `--help` |

## 📊 Sample Output

### Contributors Analysis
```
Top Contributors:
Name                      Email                          Commits  Work Pattern    Day/Night/Weekend Confidence
========================================================================================================================
Alex Filatov              alex@alexfilatov.com           37       🌙               21/16/21          71.6     %
Max Strother              max@confly.pl                  9        ☀️               09/00/00          100.0    %
John Developer            john.dev@company.com          156      ⚖️               78/45/33          65.4     %
Weekend Warrior           warrior@example.com           23       🌙               05/08/15          82.1     %

📊  Work Pattern Legend:

☀️  Day Worker: Primarily commits during business hours (9 AM - 6 PM)
🌙  Moonlighter: Commits mostly evenings/nights and weekends
⚖️  Mixed: Balanced between day and night work
❓  Unknown: Insufficient data (< 5 commits)
```

### Activity Patterns
```
Commit Activity by Month:
2024-01: 45 commits
2024-02: 67 commits
2024-03: 89 commits

Commit Activity by Hour:
09:00 - 09:59: 12 commits
10:00 - 10:59: 18 commits
14:00 - 14:59: 15 commits
22:00 - 22:59: 8 commits
```

### File Analysis
```
Most Modified Files:
File Path                                          Commits  Last Modified       
================================================================================
src/main.rs                                        45       2024-03-15 14:23
lib/utils.rs                                       32       2024-03-14 09:41
README.md                                          28       2024-03-13 16:55
```

## 🔧 Technical Details

### Work Pattern Algorithm
The tool analyzes commit timestamps to classify contributors:

1. **Time Classification**:
   - **Day**: 9 AM - 6 PM (business hours)
   - **Night**: 6 PM - 9 AM (after hours)
   - **Weekend**: Saturday & Sunday

2. **Pattern Detection**:
   - **Day Worker**: ≥70% day commits + <30% weekend commits
   - **Moonlighter**: ≥60% night commits OR ≥40% weekend commits
   - **Mixed**: Balanced patterns
   - **Unknown**: <5 total commits

3. **Confidence Scoring**:
   - Based on pattern strength and data volume
   - Higher confidence = more reliable classification

### Dependencies
- `git2`: Git repository operations
- `clap`: Command-line argument parsing
- `chrono`: Date and time handling
- `serde`: JSON serialization
- `anyhow`: Error handling
- `tempfile`: Temporary directory management

## 🌟 Use Cases

### Team Management
- **Identify work patterns** across distributed teams
- **Understand time zones** and working preferences
- **Plan meetings** based on team activity patterns
- **Balance workload** between day and night workers

### Project Analysis
- **Code hotspots**: Find frequently modified files
- **Contributor insights**: Who's most active and when
- **Maintenance planning**: Identify areas needing attention
- **Team dynamics**: Understand collaboration patterns

### Open Source Projects
- **Contributor diversity**: Mix of professional vs hobby contributors
- **Global participation**: Time zone distribution
- **Project health**: Activity trends and file stability
- **Community insights**: Understanding contributor behavior

### DevOps & Monitoring
- **CI/CD planning**: Schedule builds during low-activity periods
- **Code review scheduling**: Align with contributor patterns
- **Release planning**: Consider team availability
- **Resource allocation**: Based on activity patterns

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Built with [Rust](https://www.rust-lang.org/) for performance and safety
- Uses [git2-rs](https://github.com/rust-lang/git2-rs) for Git operations
- Inspired by the need for better repository insights

## 🐛 Issues & Support

If you encounter any issues or have questions:
1. Check existing [Issues](../../issues)
2. Create a new issue with detailed information
3. Include sample repository URLs when possible

---

**Made with ❤️ and ☕ by developers, for developers**