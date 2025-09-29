mod commands;

use clap::{Parser, Subcommand, Args};
use colored::*;
use nothung::Result;

#[derive(Parser)]
#[command(
    name = "nothung",
    version,
    about = "⚔️ A blazing-fast CLI for scaffolding smart contracts",
    long_about = "Nothung is a Rust-based CLI tool that rapidly generates boilerplate for Solidity contracts, tests, and deployment scripts in Foundry projects."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new contract, test, or script
    New(NewArgs),
    /// Show version information
    Version,
}

#[derive(Args)]
struct NewArgs {
    /// Type of resource to create (contract, library, script, or test)
    resource_type: String,
    
    /// Name of the resource
    name: String,
    
    /// Generate a Solidity contract
    #[arg(long)]
    solidity: bool,
    
    /// Generate a Rust/Stylus contract for Arbitrum Stylus
    #[arg(long = "rust-stylus")]
    rust_stylus: bool,
    
    /// Inherit from OpenZeppelin ERC20
    #[arg(long = "oz-erc20")]
    oz_erc20: bool,
    
    /// Inherit from OpenZeppelin ERC721
    #[arg(long = "oz-erc721")]
    oz_erc721: bool,
    
    /// Inherit from OpenZeppelin ERC1155
    #[arg(long = "oz-erc1155")]
    oz_erc1155: bool,
    
    /// Use upgradeable version of the contract
    #[arg(long = "upgradeable")]
    upgradeable: bool,
    
    /// Add token extensions (comma-separated: burnable,pausable,votes)
    #[arg(long = "extensions", value_delimiter = ',')]
    extensions: Vec<String>,
    
    /// Generate corresponding test file
    #[arg(long = "with-test")]
    with_test: bool,
    
    /// Generate deployment script
    #[arg(long = "with-script")]
    with_script: bool,
    
    /// Solidity pragma version
    #[arg(long = "pragma", default_value = "0.8.30")]
    pragma: String,
    
    /// SPDX License Identifier
    #[arg(long = "license", default_value = "UNLICENSED")]
    license: String,
}

fn main() {
    if let Err(e) = run() {
        eprintln!("{} {}", "Error:".red().bold(), e);
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::New(args) => {
            commands::execute_new(
                &args.resource_type,
                args.name,
                args.solidity,
                args.rust_stylus,
                args.oz_erc20,
                args.oz_erc721,
                args.oz_erc1155,
                args.upgradeable,
                args.extensions,
                args.with_test,
                args.with_script,
                args.pragma,
                args.license,
            )
        }
        Commands::Version => {
            println!("⚔️  Nothung v{}", env!("CARGO_PKG_VERSION"));
            println!("    The legendary sword that reforges smart contracts");
            Ok(())
        }
    }
}