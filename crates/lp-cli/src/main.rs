use clap::{Parser, Subcommand};
use lp_core::extractor::extract_from_text;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "lifeplanner", about = "LifePlanner CLI — offline personal planner")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Extract events and tasks from text")]
    Extract { text: String },
    #[command(about = "Show today's summary")]
    Today,
    #[command(about = "Import an ICS file")]
    Import { path: PathBuf },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Extract { text } => {
            let result = extract_from_text(&text);
            println!("Events: {}", result.events.len());
            for ev in &result.events {
                println!("  📅 {} — {}", ev.start.format("%Y-%m-%d %H:%M"), ev.title);
            }
            println!("Tasks: {}", result.tasks.len());
            for task in &result.tasks {
                println!("  ✓ {}", task.title);
            }
        }
        Commands::Today => {
            println!("Open LifePlanner desktop app for the full daily summary.");
        }
        Commands::Import { path } => {
            let events = lp_core::calendar::parse_ics_file(&path)
                .expect("Failed to parse ICS file");
            println!("Parsed {} events from {}", events.len(), path.display());
            for ev in &events {
                println!("  📅 {} — {}", ev.start.format("%Y-%m-%d"), ev.title);
            }
        }
    }
}
