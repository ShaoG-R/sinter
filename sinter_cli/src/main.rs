mod compiler;
mod themes;

use anyhow::Result;
use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;
use tracing::{Level, info};
use tracing_subscriber::FmtSubscriber;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Build the site
    Build(BuildArgs),
}

#[derive(Args, Debug)]
struct BuildArgs {
    /// Path to posts directory
    #[arg(short, long = "posts", default_value = "./posts")]
    posts_dir: PathBuf,

    /// Path to archives directory
    #[arg(short, long = "archives", default_value = "./archives")]
    archives_dir: PathBuf,

    /// Data output directory
    #[arg(short, long, default_value = "./sinter_web/sinter_data")]
    data_output: PathBuf,

    /// Enable verbose logging
    #[arg(short, long)]
    verbose: bool,

    /// Path to config file
    #[arg(short, long, default_value = "./sinter.toml")]
    config: PathBuf,

    /// Path to themes configuration
    #[arg(long, default_value = "./sinter_themes/themes.toml")]
    themes_config: PathBuf,

    /// Themes output directory
    #[arg(long, default_value = "./sinter_web/themes")]
    themes_output: PathBuf,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Build(args) => {
            // Initialize logging
            let log_level = if args.verbose {
                Level::DEBUG
            } else {
                Level::INFO
            };

            let subscriber = FmtSubscriber::builder().with_max_level(log_level).finish();
            tracing::subscriber::set_global_default(subscriber)
                .expect("setting default subscriber failed");

            info!("Starting Sinter compilation...");

            // Process themes
            if args.themes_config.exists() {
                themes::process_themes(&args.themes_config, &args.themes_output)?;
            } else {
                info!(
                    "Themes configuration not found at {:?}, skipping theme build.",
                    args.themes_config
                );
            }

            info!("Posts directory: {:?}", args.posts_dir);
            info!("Archives directory: {:?}", args.archives_dir);
            info!("Data output directory: {:?}", args.data_output);

            // Implement core compilation logic here
            compiler::compile(
                &args.posts_dir,
                &args.archives_dir,
                &args.data_output,
                &args.config,
            )?;
        }
    }

    Ok(())
}
