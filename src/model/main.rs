// Dimensionless Developments Rust Ai
// This application demonstrates:
// - Building AI agents in Rust
// - Using Ollama for local LLM inference
// - Web search integration with DuckDuckGo
// - CLI design with clap
// - Structured logging with tracing
// - Error handling best practices
// 
// ## Quick Start
// ```bash
// cargo run -- "What are the latest developments in Rust?"
// ```

// =============================================================================
// MODULE DECLARATIONS
// =============================================================================
// Rust requires explicit module declarations. Each `mod` statement tells
// the compiler to look for a file with that name (e.g., config.rs).

/// Configuration management
#[cfg(not(target_arch = "wasm32"))]
mod config;

/// Research agent implementation
#[cfg(not(target_arch = "wasm32"))]
mod agent;

/// Web search and other tools
#[cfg(not(target_arch = "wasm32"))]
mod tools;

/// WebSocket server for web interface
#[cfg(not(target_arch = "wasm32"))]
mod server;

// =============================================================================
// IMPORTS
// =============================================================================
#[cfg(not(target_arch = "wasm32"))]
use anyhow::Result;
#[cfg(not(target_arch = "wasm32"))]
use clap::Parser;
#[cfg(not(target_arch = "wasm32"))]
use tracing::{error, info, Level};
#[cfg(not(target_arch = "wasm32"))]
use tracing_subscriber::FmtSubscriber;

#[cfg(not(target_arch = "wasm32"))]
use crate::agent::ResearchAgent;
#[cfg(not(target_arch = "wasm32"))]
use crate::config::Config;

// =============================================================================
// MAIN - Only compile for non-WASM targets
// =============================================================================
#[cfg(not(target_arch = "wasm32"))]
fn main() -> Result<()> {
    tokio::runtime::Runtime::new()?.block_on(async_main())
}

#[cfg(target_arch = "wasm32")]
fn main() {
    // WASM doesn't use this binary entry point
}

#[cfg(not(target_arch = "wasm32"))]

// =============================================================================
// CLI ARGUMENTS
// =============================================================================
/// # Rust Concept: Derive Macros with Clap
/// 
/// Clap's derive feature lets us define CLI arguments as a struct.
/// The macros automatically generate argument parsing code.
/// 
/// - #[command(...)]: Configures the overall program
/// - #[arg(...)]: Configures individual arguments
#[derive(Parser, Debug)]
#[command(
    name = "ai-research-agent",
    author = "Your Name",
    version = "0.1.0",
    about = "An AI-powered research assistant that searches the web and summarizes findings",
    long_about = r#"
AI Research Agent - Your intelligent research companion!

This tool uses local LLMs (via Ollama) and web search to help you research any topic.
It will:
  1. Search the web for relevant information
  2. Analyze and synthesize the results
  3. Provide a comprehensive summary with sources

PREREQUISITES:
  1. Install Ollama: https://ollama.ai
  2. Pull a model: ollama pull llama3.2
  3. Start Ollama: ollama serve

EXAMPLES:
  # Basic research query
  ai-research-agent "What are the latest developments in Rust async?"
  
  # Quick search without synthesis
  ai-research-agent --quick "Rust web frameworks 2024"
  
  # Use a specific model
  ai-research-agent --model deepseek-v3.2 "Machine learning in Rust"
"#
)]
struct Args {
    /// The research topic or question to investigate
    #[arg(
        help = "The topic to research",
        value_name = "QUERY"
    )]
    query: Option<String>,
    
    /// Run in web server mode
    #[arg(
        short = 'w',
        long = "web",
        help = "Start web server mode",
        default_value = "false"
    )]
    web: bool,
    
    /// Port for web server
    #[arg(
        short = 'p',
        long = "port",
        help = "Port for web server",
        default_value = "8080"
    )]
    port: u16,
    
    /// The Ollama model to use (overrides OLLAMA_MODEL env var)
    #[arg(
        short = 'm',
        long = "model",
        help = "Ollama model to use",
        env = "OLLAMA_MODEL"
    )]
    model: Option<String>,
    
    /// Quick search mode - just search, don't synthesize
    #[arg(
        short = 'q',
        long = "quick",
        help = "Quick search mode (no AI synthesis)",
        default_value = "false"
    )]
    quick: bool,
    
    /// Verbose output (debug logging)
    #[arg(
        short = 'v',
        long = "verbose",
        help = "Enable verbose/debug logging",
        default_value = "false"
    )]
    verbose: bool,
}

#[cfg(not(target_arch = "wasm32"))]
async fn async_main() -> Result<()> {
    // Parse command-line arguments
    let args = Args::parse();
    
    // Initialize logging
    init_logging(args.verbose)?;
    
    info!("AI Research Agent starting up...");
    
    // Load configuration from environment/.env file
    let mut config = Config::from_env()?;
    
    // Override model if specified on command line
    if let Some(model) = args.model {
        info!(model = %model, "Using model from command line");
        config.model = model;
    }
    
    // Validate configuration
    config.validate()?;
    
    info!(
        model = %config.model,
        host = %config.ollama_host,
        "Configuration loaded"
    );
    
    // Check if web server mode
    if args.web {
        info!(port = args.port, "Starting web server mode");
        return start_web_server(args.port).await;
    }
    
    // CLI mode - require query
    let query = args.query.ok_or_else(|| {
        anyhow::anyhow!("Query required in CLI mode. Use --web for web server mode.")
    })?;
    
    // Create the research agent
    let agent = ResearchAgent::new(config);
    
    // Execute the query
    let result = if args.quick {
        // Quick mode: just search, no synthesis
        info!("Running in quick search mode");
        agent.quick_search(&query).await
    } else {
        // Full mode: search + AI synthesis
        info!("Running full research mode");
        agent.research(&query).await
    };
    
    // Handle the result
    match result {
        Ok(response) => {
            // Print the result to stdout
            println!("\n{}", "=".repeat(60));
            println!("RESEARCH RESULTS");
            println!("{}\n", "=".repeat(60));
            println!("{}", response);
            println!("\n{}", "=".repeat(60));
        }
        Err(e) => {
            // Print a user-friendly error message
            error!(error = %e, "Research failed");
            
            // Give helpful suggestions based on common errors
            eprintln!("\n❌ Research failed: {}", e);
            
            if e.to_string().contains("connection refused") {
                eprintln!("\n💡 Tip: Make sure Ollama is running:");
                eprintln!("   ollama serve");
            } else if e.to_string().contains("model") {
                eprintln!("\n💡 Tip: Make sure the model is installed:");
                eprintln!("   ollama pull llama3.2");
            }
            
            // Return the error to set non-zero exit code
            return Err(e);
        }
    }
    
    info!("Research completed successfully");
    Ok(())
}

/// Start the web server
#[cfg(not(target_arch = "wasm32"))]
async fn start_web_server(port: u16) -> Result<()> {
    let app = server::create_router();
    let addr = format!("0.0.0.0:{}", port);
    
    info!("🚀 Server starting at http://localhost:{}", port);
    info!("📱 Open http://localhost:{} in your browser", port);
    
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to bind to {}: {}", addr, e))?;
    
    axum::serve(listener, app)
        .await
        .map_err(|e| anyhow::anyhow!("Server error: {}", e))?;
    
    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
fn init_logging(verbose: bool) -> Result<()> {
    // Set log level based on verbose flag
    let level = if verbose { Level::DEBUG } else { Level::INFO };
    
    // Build the subscriber
    // 
    // # Rust Concept: Builder Pattern
    // Many Rust libraries use builders for configuration.
    // Each method modifies the builder and returns it for chaining.
    let subscriber = FmtSubscriber::builder()
        .with_max_level(level)
        .with_target(true)  // Show the module that logged
        .with_thread_names(false)
        .with_file(false)
        .with_line_number(false)
        .finish();
    
    // Set as the global default
    tracing::subscriber::set_global_default(subscriber)
        .map_err(|e| anyhow::anyhow!("Failed to set logging subscriber: {}", e))?;
    
    Ok(())
}

// =============================================================================
// INTEGRATION TESTS
// =============================================================================
/// # Rust Concept: Integration Tests
/// 
/// These tests check that all components work together.
/// They're placed in the same module but could also be in tests/ directory.
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_args_parsing() {
        // Test that CLI args parse correctly
        let args = Args::parse_from(["test", "What is Rust?"]);
        assert_eq!(args.query, Some("What is Rust?".to_string()));
        assert!(!args.quick);
        assert!(!args.verbose);
    }
    
    #[test]
    fn test_args_with_flags() {
        let args = Args::parse_from([
            "test",
            "--quick",
            "--verbose",
            "--model", "llama3.2",
            "Test query"
        ]);
        
        assert_eq!(args.query, Some("Test query".to_string()));
        assert!(args.quick);
        assert!(args.verbose);
        assert_eq!(args.model, Some("llama3.2".to_string()));
    }
}