use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::Colorize;

#[derive(Parser)]
#[command(name = "dbtective")]
#[command(about = "Detective on the case of dbt metadata issues")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Check for dbt metadata issues
    Detect {
        #[arg(short, long, default_value = ".")]
        project_directory: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Detect { project_directory }) => {
            println!(
                "  {}",
                "   ( •_•)>⌐■-■    dbt-tective is on the case!".bright_cyan()
            );

            println!(
                "{}",
                "     (⌐■_■)        Case solved with no clues left!".bright_cyan()
            );
            println!("\n")
        }
        None => {
            println!(
                "{}",
                "No command provided. Use --help for options.".bright_red()
            );
            return Ok(());
        }
    }

    Ok(())
}
