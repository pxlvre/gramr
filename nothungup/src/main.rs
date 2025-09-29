use anyhow::{anyhow, Context, Result};
use clap::{Parser, Subcommand};
use colored::*;
use std::io::{self, Write};
use std::process::{Command, Stdio};
use which::which;

const RUSTUP_INSTALL_URL: &str = "https://sh.rustup.rs";
const FOUNDRY_INSTALL_URL: &str = "https://raw.githubusercontent.com/foundry-rs/foundry/master/foundryup/foundryup";

#[derive(Parser)]
#[command(
    name = "nothungup",
    version,
    about = "⚔️ Installer for Nothung - The legendary sword that reforges smart contracts",
    long_about = "Nothungup installs Nothung and its dependencies (Rust, Cargo, Foundry) on Unix-based systems"
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Install Nothung and its dependencies
    Install {
        /// Skip dependency checks
        #[arg(long, short = 's')]
        skip_checks: bool,
        
        /// Install from local repository instead of GitHub
        #[arg(long, short = 'l')]
        local: bool,
        
        /// Force reinstall even if already installed
        #[arg(long, short = 'f')]
        force: bool,
    },
    /// Update Nothung to the latest version
    Update,
    /// Uninstall Nothung
    Uninstall,
    /// Check system dependencies
    Check,
    /// Self-management commands
    #[command(name = "self", subcommand)]
    SelfCmd(SelfCommands),
}

#[derive(Subcommand)]
enum SelfCommands {
    /// Uninstall nothungup itself
    Uninstall,
}

#[derive(Debug)]
struct DependencyStatus {
    rust: bool,
    cargo: bool,
    forge: bool,
    anvil: bool,
    cast: bool,
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
        Some(Commands::Install { skip_checks, local, force }) => {
            install(skip_checks, local, force)
        }
        Some(Commands::Update) => update(),
        Some(Commands::Uninstall) => uninstall(),
        Some(Commands::Check) => {
            check_dependencies()?;
            Ok(())
        }
        Some(Commands::SelfCmd(self_cmd)) => {
            match self_cmd {
                SelfCommands::Uninstall => self_uninstall(),
            }
        }
        None => install(false, false, false), // Default to install
    }
}

fn install(skip_checks: bool, local: bool, force: bool) -> Result<()> {
    println!("{}", "⚔️  Nothungup - Forging the legendary sword...".cyan().bold());
    println!();
    
    // Check if nothung is already installed
    if !force && which("nothung").is_ok() {
        println!("{} Nothung is already installed!", "✓".green().bold());
        println!("Use 'nothungup update' to update or 'nothungup install --force' to reinstall");
        return Ok(());
    }
    
    // Check and install dependencies
    if !skip_checks {
        let status = check_dependencies()?;
        
        if !status.rust || !status.cargo {
            if prompt_install("Rust and Cargo")? {
                install_rust()?;
            } else {
                return Err(anyhow!("Rust and Cargo are required to install Nothung"));
            }
        }
        
        if !status.forge || !status.anvil || !status.cast {
            if prompt_install("Foundry (forge, anvil, cast)")? {
                install_foundry()?;
            } else {
                println!("{}", "⚠️  Warning: Foundry is recommended for full functionality".yellow());
            }
        }
    }
    
    // Install Nothung
    if local {
        install_from_local()?;
    } else {
        install_from_github()?;
    }
    
    println!();
    println!("{} Nothung installed successfully!", "✓".green().bold());
    println!();
    println!("Get started with:");
    println!("  {} wotan                                    # Interactive wizard", "→".cyan());
    println!("  {} nothung wizard                           # Also launches wizard", "→".cyan());
    println!("  {} nothung new contract MyToken --solidity --oz-erc20  # Direct CLI", "→".cyan());
    println!();
    
    Ok(())
}

fn check_dependencies() -> Result<DependencyStatus> {
    println!("{}", "Checking system dependencies...".cyan());
    println!();
    
    let status = DependencyStatus {
        rust: check_command("rustc", "--version"),
        cargo: check_command("cargo", "--version"),
        forge: check_command("forge", "--version"),
        anvil: check_command("anvil", "--version"),
        cast: check_command("cast", "--version"),
    };
    
    // Print status
    print_check("Rust", status.rust);
    print_check("Cargo", status.cargo);
    print_check("Forge", status.forge);
    print_check("Anvil", status.anvil);
    print_check("Cast", status.cast);
    println!();
    
    Ok(status)
}

fn check_command(cmd: &str, arg: &str) -> bool {
    Command::new(cmd)
        .arg(arg)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

fn print_check(name: &str, installed: bool) {
    if installed {
        println!("  {} {}", "✓".green().bold(), name);
    } else {
        println!("  {} {} (not installed)", "✗".red().bold(), name);
    }
}

fn prompt_install(tool: &str) -> Result<bool> {
    print!("{} {} is not installed. Install it now? (yes/no): ", 
           "?".yellow().bold(), 
           tool);
    io::stdout().flush()?;
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    
    Ok(input.trim().to_lowercase().starts_with('y'))
}

fn install_rust() -> Result<()> {
    println!();
    println!("{}", "Installing Rust and Cargo...".cyan());
    
    // Download and run rustup
    let output = Command::new("sh")
        .arg("-c")
        .arg(format!("curl --proto '=https' --tlsv1.2 -sSf {} | sh -s -- -y", RUSTUP_INSTALL_URL))
        .status()
        .context("Failed to install Rust")?;
    
    if !output.success() {
        return Err(anyhow!("Rust installation failed"));
    }
    
    // Source cargo env
    let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
    let cargo_env = format!("{}/.cargo/env", home);
    
    println!("{} Rust and Cargo installed!", "✓".green().bold());
    println!("  Run 'source {}' to update your current shell", cargo_env);
    
    Ok(())
}

fn install_foundry() -> Result<()> {
    println!();
    println!("{}", "Installing Foundry...".cyan());
    
    // Download and run foundryup
    let output = Command::new("sh")
        .arg("-c")
        .arg(format!("curl -L {} | bash && $HOME/.foundry/bin/foundryup", FOUNDRY_INSTALL_URL))
        .status()
        .context("Failed to install Foundry")?;
    
    if !output.success() {
        return Err(anyhow!("Foundry installation failed"));
    }
    
    println!("{} Foundry installed!", "✓".green().bold());
    println!("  Add '$HOME/.foundry/bin' to your PATH");
    
    Ok(())
}

fn install_from_local() -> Result<()> {
    println!();
    println!("{}", "Installing Nothung from local repository...".cyan());
    
    // Build the project
    let output = Command::new("cargo")
        .args(&["build", "--release", "--package", "nothung-cli"])
        .status()
        .context("Failed to build Nothung")?;
    
    if !output.success() {
        return Err(anyhow!("Build failed"));
    }
    
    // Install the CLI binary
    let output = Command::new("cargo")
        .args(&["install", "--path", "cli", "--force"])
        .status()
        .context("Failed to install Nothung CLI")?;
    
    if !output.success() {
        return Err(anyhow!("Nothung CLI installation failed"));
    }
    
    // Install Wotan wizard
    let output = Command::new("cargo")
        .args(&["install", "--path", "wotan", "--force"])
        .status()
        .context("Failed to install Wotan wizard")?;
    
    if !output.success() {
        println!("{} Wotan wizard installation failed, but continuing...", "⚠️".yellow());
    } else {
        println!("{} Wotan wizard installed!", "✓".green());
    }
    
    Ok(())
}

fn install_from_github() -> Result<()> {
    println!();
    println!("{}", "Installing Nothung from GitHub...".cyan());
    
    // Clone the repository
    let temp_dir = std::env::temp_dir().join("nothung-install");
    
    // Remove temp dir if it exists
    let _ = std::fs::remove_dir_all(&temp_dir);
    
    let output = Command::new("git")
        .args(&["clone", "https://github.com/pxlvre/nothung.git"])
        .arg(&temp_dir)
        .status()
        .context("Failed to clone Nothung repository")?;
    
    if !output.success() {
        return Err(anyhow!("Failed to clone repository"));
    }
    
    // Build and install Nothung CLI
    let output = Command::new("cargo")
        .args(&["install", "--path"])
        .arg(temp_dir.join("cli"))
        .args(&["--force"])
        .status()
        .context("Failed to install Nothung CLI")?;
    
    if !output.success() {
        return Err(anyhow!("Nothung CLI installation failed"));
    }
    
    // Build and install Wotan wizard
    let output = Command::new("cargo")
        .args(&["install", "--path"])
        .arg(temp_dir.join("wotan"))
        .args(&["--force"])
        .status()
        .context("Failed to install Wotan wizard")?;
    
    if !output.success() {
        println!("{} Wotan wizard installation failed, but continuing...", "⚠️".yellow());
    } else {
        println!("{} Wotan wizard installed!", "✓".green());
    }
    
    // Clean up
    let _ = std::fs::remove_dir_all(&temp_dir);
    
    Ok(())
}

fn update() -> Result<()> {
    println!("{}", "⚔️  Updating Nothung...".cyan().bold());
    
    // Check if installed
    if which("nothung").is_err() {
        println!("{} Nothung is not installed!", "✗".red().bold());
        println!("Run 'nothungup install' to install it");
        return Ok(());
    }
    
    // Reinstall from GitHub
    install_from_github()?;
    
    println!();
    println!("{} Nothung updated successfully!", "✓".green().bold());
    
    Ok(())
}

fn uninstall() -> Result<()> {
    println!("{}", "⚔️  Uninstalling Nothung...".cyan().bold());
    
    // Check if installed
    if which("nothung").is_err() {
        println!("{} Nothung is not installed!", "✗".red().bold());
        return Ok(());
    }
    
    // Uninstall using cargo
    let output = Command::new("cargo")
        .args(&["uninstall", "nothung-cli"])
        .status()
        .context("Failed to uninstall Nothung")?;
    
    if !output.success() {
        return Err(anyhow!("Uninstallation failed"));
    }
    
    println!();
    println!("{} Nothung uninstalled successfully!", "✓".green().bold());
    
    Ok(())
}

fn self_uninstall() -> Result<()> {
    println!("{}", "⚔️  Uninstalling nothungup...".cyan().bold());
    
    // Get the current executable path
    let current_exe = std::env::current_exe()
        .context("Failed to get current executable path")?;
    
    println!("Current executable: {}", current_exe.display());
    
    // Confirm with user
    print!("{} Are you sure you want to uninstall nothungup? This will remove the nothungup binary. (yes/no): ", 
           "?".yellow().bold());
    io::stdout().flush()?;
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    
    if !input.trim().to_lowercase().starts_with('y') {
        println!("Uninstall cancelled.");
        return Ok(());
    }
    
    // On Unix systems, we can't delete the currently running executable
    // So we create a temporary script to delete it after we exit
    #[cfg(unix)]
    {
        let temp_script = std::env::temp_dir().join("nothungup_uninstall.sh");
        let script_content = format!(
            "#!/bin/bash\nsleep 1\nrm -f '{}'\nrm -f '{}'\n",
            current_exe.display(),
            temp_script.display()
        );
        
        std::fs::write(&temp_script, script_content)
            .context("Failed to create uninstall script")?;
        
        // Make script executable
        Command::new("chmod")
            .args(&["+x"])
            .arg(&temp_script)
            .status()
            .context("Failed to make uninstall script executable")?;
        
        println!("{} nothungup will be uninstalled after this process exits.", "✓".green().bold());
        
        // Execute the script in the background and exit
        Command::new("nohup")
            .arg(&temp_script)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .context("Failed to start uninstall script")?;
    }
    
    #[cfg(windows)]
    {
        // On Windows, we can use a batch script
        let temp_script = std::env::temp_dir().join("nothungup_uninstall.bat");
        let script_content = format!(
            "@echo off\ntimeout /t 1 /nobreak >nul\ndel /f \"{}\"\ndel /f \"{}\"\n",
            current_exe.display(),
            temp_script.display()
        );
        
        std::fs::write(&temp_script, script_content)
            .context("Failed to create uninstall script")?;
        
        println!("{} nothungup will be uninstalled after this process exits.", "✓".green().bold());
        
        // Execute the script and exit
        Command::new("cmd")
            .args(&["/C", "start", "/B"])
            .arg(&temp_script)
            .spawn()
            .context("Failed to start uninstall script")?;
    }
    
    Ok(())
}