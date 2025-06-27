use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::Colorize;
use dbtective_core::DbtProjectAnalyzer;

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
            println!("\n{}\n", "( •_•)>⌐■-■   dbt-tective".bright_cyan());
            // Use the core analyzer
            let analyzer = DbtProjectAnalyzer::new(project_directory.clone());
            match analyzer.analyze() {
                Ok(report) => {
                    println!(
                        "📊 Analysis complete for: {}",
                        report.project_path.bright_yellow()
                    );
                    println!(
                        "🎯 Overall Score: {:.1}%",
                        report.overall_score.to_string().bright_green()
                    );

                    if !report.findings.is_empty() {
                        println!("\n🔍 Findings:");
                        for finding in &report.findings {
                            println!("  • {}", finding);
                        }
                    }
                }
                Err(e) => {
                    println!("{} {}", "❌ Analysis failed:".bright_red(), e);
                }
            }

            println!("\n{}\n", "(⌐■_■)        Case solved!".bright_cyan());
        }
        None => {
            println!("\n{}", "Use --help for available commands.".bright_yellow());
        }
    }

    Ok(())
}
