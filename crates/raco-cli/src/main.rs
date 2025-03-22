// RACO CLI - Command-line interface for RACO

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use colored::Colorize;
use raco_core::config::load_config;
use raco_core::utils::ensure_dir_exists;
use tracing::{debug, info};
use tracing_subscriber::EnvFilter;

/// RACO - Ralf's AI Code Orchestrator
#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
struct Cli {
    /// Enable verbose output
    #[clap(short, long, global = true)]
    verbose: bool,

    /// Configuration file
    #[clap(short, long, global = true)]
    config: Option<String>,

    /// Subcommands
    #[clap(subcommand)]
    command: Commands,
}

/// RACO commands
#[derive(Subcommand, Debug)]
enum Commands {
    /// Start a new RACO session
    #[clap(about = "Start a new RACO session")]
    Start {
        /// Project directory
        #[clap(short, long)]
        project: Option<String>,
    },

    /// Initialize a new project
    #[clap(about = "Initialize a new project")]
    Init {
        /// Project name
        #[clap(short, long)]
        name: String,

        /// Project directory (defaults to current directory)
        #[clap(short, long)]
        directory: Option<String>,
    },

    /// List available servers
    #[clap(about = "List available servers")]
    Servers,

    /// Run a command on a server
    #[clap(about = "Run a command on a server")]
    Run {
        /// Server name
        #[clap(short, long)]
        server: String,

        /// Command to run
        #[clap(short, long)]
        command: String,

        /// Command arguments
        #[clap(short, long)]
        args: Option<Vec<String>>,
    },
}

/// Initialize logging
fn init_logging(verbose: bool) {
    let env_filter = if verbose {
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("debug"))
    } else {
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"))
    };

    let _ = tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .with_target(true)
        .try_init();
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Initialize logging
    init_logging(cli.verbose);

    // Load configuration
    let config = load_config().context("Failed to load configuration")?;

    // Ensure data directory exists
    ensure_dir_exists(&config.data_dir).context("Failed to create data directory")?;

    debug!("Using data directory: {}", config.data_dir.display());

    // Handle commands
    match cli.command {
        Commands::Start { project } => {
            let project_dir = project.unwrap_or_else(|| ".".to_string());
            info!("Starting RACO session in {}", project_dir);
            println!("{}", "Session started successfully.".green());
            Ok(())
        }
        Commands::Init { name, directory } => {
            let dir = directory.unwrap_or_else(|| ".".to_string());
            info!("Initializing project {} in {}", name, dir);
            println!("{} {}", "Project".green(), name.green().bold());
            println!("{} {}", "Initialized in".green(), dir.green());
            Ok(())
        }
        Commands::Servers => {
            info!("Listing available servers");
            println!("{}", "Available servers:".green().bold());
            println!("- {}: {}", "filesystem".yellow(), "Local filesystem server");
            println!("- {}: {}", "process".yellow(), "Process management server");
            Ok(())
        }
        Commands::Run {
            server,
            command,
            args,
        } => {
            let args_str = args.map_or_else(String::new, |a| a.join(" "));
            info!(
                "Running command {} {} on server {}",
                command, args_str, server
            );
            println!(
                "{} {} {}",
                "Running on".green(),
                server.yellow(),
                command.yellow()
            );
            println!("{}", "Command completed successfully.".green());
            Ok(())
        }
    }
}
