use anyhow::Result;
use chrono::{DateTime, Utc};
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
        println!("{:<30} {:<30} {:<8} {:<20} {:<20}", "Name", "Email", "Commits", "First Commit", "Last Commit");
        println!("{}", "=".repeat(120));
        
        for contributor in contributors.iter().take(10) {
            println!("{:<30} {:<30} {:<8} {:<20} {:<20}", 
                contributor.name,
                contributor.email,
                contributor.commits,
                contributor.first_commit.format("%Y-%m-%d %H:%M"),
                contributor.last_commit.format("%Y-%m-%d %H:%M")
            );
        }
    }
    
    Ok(())
}

fn collect_contributor_data(repo: &Repository) -> Result<Vec<ContributorStats>> {
    let mut contributors: HashMap<String, ContributorStats> = HashMap::new();
    
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
            .and_modify(|stats| {
                stats.commits += 1;
                if commit_time < stats.first_commit {
                    stats.first_commit = commit_time;
                }
                if commit_time > stats.last_commit {
                    stats.last_commit = commit_time;
                }
            })
            .or_insert(ContributorStats {
                name: author_name,
                email: author_email,
                commits: 1,
                first_commit: commit_time,
                last_commit: commit_time,
            });
    }
    
    let mut sorted_contributors: Vec<_> = contributors.into_values().collect();
    sorted_contributors.sort_by(|a, b| b.commits.cmp(&a.commits));
    
    Ok(sorted_contributors)
}

fn analyze_activity(repo_path: &PathBuf, json_output: bool) -> Result<()> {
    let repo = Repository::open(repo_path)?;
    println!("📈 Analyzing activity in repository: {:?}", repo_path);
    Ok(())
}

fn analyze_files(repo_path: &PathBuf, json_output: bool) -> Result<()> {
    let repo = Repository::open(repo_path)?;
    println!("📁 Analyzing files in repository: {:?}", repo_path);
    Ok(())
}

fn analyze_all(repo_path: &PathBuf, json_output: bool) -> Result<()> {
    println!("🔍 Running all analyses...");
    analyze_contributors(repo_path, json_output)?;
    analyze_activity(repo_path, json_output)?;
    analyze_files(repo_path, json_output)?;
    Ok(())
}
