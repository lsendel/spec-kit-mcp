//! Spec-Kit MCP Server Binary
//!
//! Main entry point for the MCP server.

use anyhow::Result;
use clap::Parser;
use spec_kit_mcp::{create_registry, McpServer, SpecKitCli};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

/// Spec-Kit MCP Server
#[derive(Parser, Debug)]
#[command(name = "spec-kit-mcp")]
#[command(about = "MCP server for GitHub Spec-Kit", long_about = None)]
#[command(version)]
struct Args {
    /// Log level (trace, debug, info, warn, error)
    #[arg(short, long, default_value = "info")]
    log_level: String,

    /// Path to spec-kit CLI (defaults to 'specify' in PATH)
    #[arg(long)]
    cli_path: Option<String>,

    /// Timeout for spec-kit commands in seconds
    #[arg(long, default_value = "300")]
    timeout: u64,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Parse arguments
    let args = Args::parse();

    // Initialize logging
    init_logging(&args.log_level)?;

    tracing::info!(
        version = env!("CARGO_PKG_VERSION"),
        "Starting spec-kit-mcp server"
    );

    // Create spec-kit CLI interface
    let mut cli = SpecKitCli::new().with_timeout(args.timeout);

    if let Some(cli_path) = args.cli_path {
        cli = cli.with_cli_path(cli_path);
    }

    // Check if spec-kit is installed
    if !cli.is_installed().await {
        eprintln!("Error: spec-kit CLI not found!");
        eprintln!("Please install it with: uv tool install specify-cli");
        eprintln!("Or: pip install specify-cli");
        std::process::exit(1);
    }

    tracing::info!("Spec-kit CLI found and validated");

    // Create tool registry
    let registry = create_registry(cli);
    tracing::info!(tool_count = registry.len(), "Tool registry initialized");

    // Create and run server
    let mut server = McpServer::new(registry);

    tracing::info!("MCP server ready, listening on stdio");

    // Run server
    server.run().await?;

    Ok(())
}

/// Initialize logging
fn init_logging(level: &str) -> Result<()> {
    let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new(level));

    tracing_subscriber::registry()
        .with(env_filter)
        .with(tracing_subscriber::fmt::layer().with_writer(std::io::stderr))
        .init();

    Ok(())
}
