use anyhow::Result;
use chrono::{DateTime, Utc, Timelike, Datelike, Weekday};
use clap::{Args, Parser, Subcommand};
use git2::Repository;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "git-analyzer")]
#[command(about = "Analyze Git repositories for insights and statistics")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Contributors(ContributorsArgs),
    Activity(ActivityArgs),
    Files(FilesArgs),
    All(AllArgs),
}

#[derive(Args)]
struct ContributorsArgs {
    #[arg(short, long, default_value = ".")]
    path: PathBuf,
    #[arg(short, long)]
    json: bool,
}

#[derive(Args)]
struct ActivityArgs {
    #[arg(short, long, default_value = ".")]
    path: PathBuf,
    #[arg(short, long)]
    json: bool,
}

#[derive(Args)]
struct FilesArgs {
    #[arg(short, long, default_value = ".")]
    path: PathBuf,
    #[arg(short, long)]
    json: bool,
}

#[derive(Args)]
struct AllArgs {
    #[arg(short, long, default_value = ".")]
    path: PathBuf,
    #[arg(short, long)]
    json: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct ContributorStats {
    name: String,
    email: String,
    commits: u32,
    first_commit: DateTime<Utc>,
    last_commit: DateTime<Utc>,
    work_pattern: WorkPattern,
    hourly_commits: HashMap<u8, u32>,
}

#[derive(Serialize, Deserialize, Debug)]
struct WorkPattern {
    pattern_type: String,  // "day_worker", "moonlighter", "mixed", "unknown"
    day_commits: u32,      // 9 AM - 6 PM
    night_commits: u32,    // 6 PM - 9 AM
    weekend_commits: u32,
    confidence: f32,       // 0.0 - 1.0
}

#[derive(Serialize, Deserialize, Debug)]
struct FileStats {
    path: String,
    commits: u32,
    last_modified: DateTime<Utc>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Contributors(args) => {
            analyze_contributors(&args.path, args.json)?
        },
        Commands::Activity(args) => {
            analyze_activity(&args.path, args.json)?
        },
        Commands::Files(args) => {
            analyze_files(&args.path, args.json)?
        },
        Commands::All(args) => {
            analyze_all(&args.path, args.json)?
        },
    }

    Ok(())
}

fn analyze_contributors(repo_path: &PathBuf, json_output: bool) -> Result<()> {
    let repo = Repository::open(repo_path)?;
    let contributors = collect_contributor_data(&repo)?;
    
    if json_output {
        println!("{}", serde_json::to_string_pretty(&contributors)?);
    } else {
        println!("📊 Top Contributors:");
        println!("{:<25} {:<25} {:<8} {:<15} {:<15} {:<12}", "Name", "Email", "Commits", "Pattern", "Confidence", "Day/Night");
        println!("{}", "=".repeat(120));
        
        for contributor in contributors.iter().take(10) {
            let pattern_emoji = match contributor.work_pattern.pattern_type.as_str() {
                "day_worker" => "☀️",
                "moonlighter" => "🌙",
                "mixed" => "⚖️",
                _ => "❓",
            };
            
            println!("{:<25} {:<25} {:<8} {:<15} {:<15.1}% {:<12}", 
                contributor.name,
                contributor.email,
                contributor.commits,
                format!("{} {}", pattern_emoji, contributor.work_pattern.pattern_type),
                contributor.work_pattern.confidence * 100.0,
                format!("{}/{}", contributor.work_pattern.day_commits, contributor.work_pattern.night_commits)
            );
        }
        
        println!("\nLegend:");
        println!("☀️  Day Worker: Primarily commits during business hours (9 AM - 6 PM)");
        println!("🌙  Moonlighter: Commits mostly evenings/nights and weekends");
        println!("⚖️  Mixed: Balanced between day and night work");
        println!("❓  Unknown: Insufficient data (< 5 commits)");
    }
    
    Ok(())
}

fn collect_contributor_data(repo: &Repository) -> Result<Vec<ContributorStats>> {
    let mut contributors: HashMap<String, (ContributorStats, Vec<DateTime<Utc>>)> = HashMap::new();
    
    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;
    
    for oid in revwalk {
        let oid = oid?;
        let commit = repo.find_commit(oid)?;
        let author = commit.author();
        let author_name = author.name().unwrap_or("Unknown").to_string();
        let author_email = author.email().unwrap_or("unknown@example.com").to_string();
        let commit_time = DateTime::from_timestamp(author.when().seconds(), 0)
            .unwrap_or_default()
            .with_timezone(&Utc);
        
        let key = format!("{} <{}>", author_name, author_email);
        
        contributors.entry(key)
            .and_modify(|(stats, times)| {
                stats.commits += 1;
                if commit_time < stats.first_commit {
                    stats.first_commit = commit_time;
                }
                if commit_time > stats.last_commit {
                    stats.last_commit = commit_time;
                }
                times.push(commit_time);
            })
            .or_insert((
                ContributorStats {
                    name: author_name,
                    email: author_email,
                    commits: 1,
                    first_commit: commit_time,
                    last_commit: commit_time,
                    work_pattern: WorkPattern {
                        pattern_type: "unknown".to_string(),
                        day_commits: 0,
                        night_commits: 0,
                        weekend_commits: 0,
                        confidence: 0.0,
                    },
                    hourly_commits: HashMap::new(),
                },
                vec![commit_time],
            ));
    }
    
    let mut sorted_contributors: Vec<ContributorStats> = contributors
        .into_iter()
        .map(|(_, (mut stats, times))| {
            stats.work_pattern = calculate_work_pattern(&times);
            stats.hourly_commits = calculate_hourly_distribution(&times);
            stats
        })
        .collect();
    
    sorted_contributors.sort_by(|a, b| b.commits.cmp(&a.commits));
    
    Ok(sorted_contributors)
}

fn calculate_work_pattern(commit_times: &[DateTime<Utc>]) -> WorkPattern {
    let mut day_commits = 0;
    let mut night_commits = 0;
    let mut weekend_commits = 0;
    
    for time in commit_times {
        let hour = time.hour();
        let weekday = time.weekday();
        
        if weekday == Weekday::Sat || weekday == Weekday::Sun {
            weekend_commits += 1;
        }
        
        if hour >= 9 && hour < 18 {
            day_commits += 1;
        } else {
            night_commits += 1;
        }
    }
    
    let total_commits = commit_times.len() as u32;
    let day_ratio = day_commits as f32 / total_commits as f32;
    let night_ratio = night_commits as f32 / total_commits as f32;
    let weekend_ratio = weekend_commits as f32 / total_commits as f32;
    
    // Determine pattern type with confidence
    let (pattern_type, confidence) = if total_commits < 5 {
        ("unknown".to_string(), 0.0)
    } else if day_ratio >= 0.7 && weekend_ratio < 0.3 {
        ("day_worker".to_string(), day_ratio)
    } else if night_ratio >= 0.6 || weekend_ratio >= 0.4 {
        ("moonlighter".to_string(), (night_ratio + weekend_ratio * 0.5).min(1.0))
    } else {
        ("mixed".to_string(), 1.0 - (day_ratio - night_ratio).abs())
    };
    
    WorkPattern {
        pattern_type,
        day_commits,
        night_commits,
        weekend_commits,
        confidence,
    }
}

fn calculate_hourly_distribution(commit_times: &[DateTime<Utc>]) -> HashMap<u8, u32> {
    let mut hourly_commits = HashMap::new();
    
    for time in commit_times {
        let hour = time.hour() as u8;
        *hourly_commits.entry(hour).or_insert(0) += 1;
    }
    
    hourly_commits
}

fn analyze_activity(repo_path: &PathBuf, json_output: bool) -> Result<()> {
    let repo = Repository::open(repo_path)?;
    let (monthly_commits, hourly_commits) = collect_activity_data(&repo)?;

    if json_output {
        let activity = serde_json::json!({
            "monthly_commits": monthly_commits,
            "hourly_commits": hourly_commits
        });
        println!("{}", serde_json::to_string_pretty(&activity)?);
    } else {
        println!("📈 Commit Activity by Month:");
        let mut sorted_months: Vec<_> = monthly_commits.iter().collect();
        sorted_months.sort_by(|a, b| a.0.cmp(b.0));
        
        for (month, count) in sorted_months {
            println!("{}: {} commits", month, count);
        }

        println!("\n📊 Commit Activity by Hour:");
        for hour in 0..24 {
            let count = hourly_commits.get(&hour).unwrap_or(&0);
            println!("{:02}:00 - {:02}:59: {} commits", hour, hour, count);
        }
    }

    Ok(())
}

fn collect_activity_data(repo: &Repository) -> Result<(HashMap<String, u32>, HashMap<u8, u32>)> {
    let mut monthly_commits: HashMap<String, u32> = HashMap::new();
    let mut hourly_commits: HashMap<u8, u32> = HashMap::new();
    
    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;
    
    for oid in revwalk {
        let oid = oid?;
        let commit = repo.find_commit(oid)?;
        let author = commit.author();
        let commit_time = DateTime::from_timestamp(author.when().seconds(), 0)
            .unwrap_or_default()
            .with_timezone(&Utc);
        
        let month_key = commit_time.format("%Y-%m").to_string();
        *monthly_commits.entry(month_key).or_insert(0) += 1;
        
        let hour = commit_time.hour() as u8;
        *hourly_commits.entry(hour).or_insert(0) += 1;
    }
    
    Ok((monthly_commits, hourly_commits))
}

fn analyze_files(repo_path: &PathBuf, json_output: bool) -> Result<()> {
    let repo = Repository::open(repo_path)?;
    let sorted_files = collect_file_data(&repo)?;

    if json_output {
        println!("{}", serde_json::to_string_pretty(&sorted_files)?);
    } else {
        println!("📁 Most Modified Files:");
        println!("{:<50} {:<8} {:<20}", "File Path", "Commits", "Last Modified");
        println!("{}", "=".repeat(80));
        
        for file in sorted_files.iter().take(20) {
            println!("{:<50} {:<8} {:<20}", 
                file.path,
                file.commits,
                file.last_modified.format("%Y-%m-%d %H:%M")
            );
        }
    }

    Ok(())
}

fn collect_file_data(repo: &Repository) -> Result<Vec<FileStats>> {
    let mut file_stats: HashMap<String, FileStats> = HashMap::new();
    
    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;
    
    for oid in revwalk {
        let oid = oid?;
        let commit = repo.find_commit(oid)?;
        let tree = commit.tree()?;
        let author = commit.author();
        let commit_time = DateTime::from_timestamp(author.when().seconds(), 0)
            .unwrap_or_default()
            .with_timezone(&Utc);
        
        if let Some(parent) = commit.parents().next() {
            let parent_tree = parent.tree()?;
            let diff = repo.diff_tree_to_tree(Some(&parent_tree), Some(&tree), None)?;
            
            diff.foreach(
                &mut |delta, _progress| {
                    if let Some(new_file) = delta.new_file().path() {
                        let path_str = new_file.to_string_lossy().to_string();
                        
                        file_stats.entry(path_str.clone())
                            .and_modify(|stats| {
                                stats.commits += 1;
                                if commit_time > stats.last_modified {
                                    stats.last_modified = commit_time;
                                }
                            })
                            .or_insert(FileStats {
                                path: path_str,
                                commits: 1,
                                last_modified: commit_time,
                            });
                    }
                    true
                },
                None,
                None,
                None,
            )?;
        }
    }
    
    let mut sorted_files: Vec<_> = file_stats.into_values().collect();
    sorted_files.sort_by(|a, b| b.commits.cmp(&a.commits));
    
    Ok(sorted_files)
}

fn analyze_all(repo_path: &PathBuf, json_output: bool) -> Result<()> {
    println!("🔍 Running all analyses...");
    analyze_contributors(repo_path, json_output)?;
    analyze_activity(repo_path, json_output)?;
    analyze_files(repo_path, json_output)?;
    Ok(())
}
