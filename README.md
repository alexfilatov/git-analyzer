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
- Rust 1.70+ (2021 edition)
- Git (for repository operations)

### Build from Source
```bash
git clone <your-repo-url>
cd git-analyzer
cargo build --release
```

### Using the Binary

```bash
# Analyze contributors
./target/release/git-analyzer contributors

# Analyze activity patterns
./target/release/git-analyzer activity

# Analyze file changes
./target/release/git-analyzer files

# Run all analyses
./target/release/git-analyzer all

# JSON output
./target/release/git-analyzer contributors --json

# Analyze remote repository
./target/release/git-analyzer contributors --url https://github.com/user/repo.git
```

### Command Options

All commands support:
- `--json` or `-j`: Output results in JSON format
- `--path` or `-p`: Specify local repository path (default: current directory)
- `--url` or `-u`: Clone and analyze remote repository from URL

## 📊 Example Output

### Contributors Analysis
```
📊 Top Contributors:
Name                     Email                    Commits  Pattern         Confidence      Day/Night
================================================================================================================
John Doe                 john@example.com         142      ☀️ day_worker    85.2%           120/22
Jane Smith               jane@example.com         89       🌙 moonlighter   72.1%           25/64
Bob Wilson               bob@example.com          45       ⚖️ mixed         68.9%           22/23

Legend:
☀️  Day Worker: Primarily commits during business hours (9 AM - 6 PM)
🌙  Moonlighter: Commits mostly evenings/nights and weekends
⚖️  Mixed: Balanced between day and night work
❓  Unknown: Insufficient data (< 5 commits)
```

### Activity Analysis
```
📈 Commit Activity by Month:
2024-01: 45 commits
2024-02: 67 commits
2024-03: 89 commits

📊 Commit Activity by Hour:
00:00 - 00:59: 2 commits
01:00 - 01:59: 1 commits
...
09:00 - 09:59: 15 commits
10:00 - 10:59: 23 commits
```

### Files Analysis
```
📁 Most Modified Files:
File Path                                          Commits  Last Modified
================================================================================
src/main.rs                                        45       2024-03-15 14:30
Cargo.toml                                         12       2024-03-10 09:15
README.md                                          8        2024-03-14 16:45
```

## 🏗️ Architecture

The tool is structured around several key components:

- **CLI Interface**: Built with `clap` for robust argument parsing
- **Git Integration**: Uses `git2` for repository operations
- **Analysis Engine**: Core logic for contributor, activity, and file analysis
- **Work Pattern Detection**: Advanced algorithms to classify contributor behavior
- **Output Formatting**: Both human-readable tables and structured JSON

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Built with [Rust](https://www.rust-lang.org/)
- Uses [git2](https://github.com/rust-lang/git2-rs) for Git operations
- CLI powered by [clap](https://github.com/clap-rs/clap)
- Time handling with [chrono](https://github.com/chronotope/chrono)
