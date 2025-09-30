mod wizard;

use anyhow::Result;
use clap::{Arg, Command};
use colored::*;
use gramr::{
    ContractType, GenericContractGenerator, LibraryGenerator, ProjectType, ScriptGenerator,
    TestGenerator,
};
use wizard::{ContractWizard, WizardState};

fn main() {
    if let Err(e) = run() {
        eprintln!("{} {}", "Error:".red().bold(), e);
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let app = Command::new("wotan")
        .version(env!("CARGO_PKG_VERSION"))
        .about("🧙‍♂️ Interactive wizard for generating smart contracts with Gramr")
        .long_about("Wotan is the interactive wizard companion to Gramr that guides you through creating smart contracts, libraries, scripts, and tests step by step.")
        .arg(
            Arg::new("non-interactive")
                .long("non-interactive")
                .short('n')
                .help("Skip interactive mode and show help")
                .action(clap::ArgAction::SetTrue)
        );

    let matches = app.try_get_matches();

    match matches {
        Ok(matches) => {
            if matches.get_flag("non-interactive") {
                print_non_interactive_help();
                return Ok(());
            }
        }
        Err(_) => {
            // If argument parsing fails, still run the wizard
            // This allows running `wotan` without any arguments
        }
    }

    // Run the interactive wizard
    let wizard = ContractWizard::new();
    let state = wizard.run()?;

    // Generate the files based on wizard choices
    generate_from_state(state)?;

    Ok(())
}

fn print_non_interactive_help() {
    println!(
        "{}",
        "🧙‍♂️ Wotan - Interactive Smart Contract Wizard".bold().cyan()
    );
    println!();
    println!("{}", "USAGE:".bold());
    println!("    wotan                    # Start interactive wizard");
    println!("    wotan --non-interactive  # Show this help");
    println!();
    println!("{}", "DESCRIPTION:".bold());
    println!("    Wotan guides you through creating smart contracts, libraries,");
    println!("    scripts, and tests with an easy-to-use interactive interface.");
    println!();
    println!("{}", "SUPPORTED LANGUAGES:".bold());
    println!(
        "    • {} - Full support (contracts, libraries, scripts, tests)",
        "Solidity".green()
    );
    println!(
        "    • {} - Experimental (contracts, libraries only)",
        "Rust/Stylus".yellow()
    );
    println!();
    println!("{}", "FEATURES:".bold());
    println!("    • Interactive prompts with validation");
    println!("    • Token standard selection (ERC20, ERC721, ERC1155)");
    println!("    • Extension selection (burnable, pausable, etc.)");
    println!("    • Upgradeable contract support (Solidity)");
    println!("    • Automatic dependency management");
    println!();
    println!(
        "{}",
        "For non-interactive usage, use the gramr CLI directly:"
    );
    println!("    {}", "gramr new contract MyToken --solidity --oz-erc20");
}

fn generate_from_state(state: WizardState) -> Result<()> {
    println!("\n{} Generating files...", "🔨".bold());

    // Detect project
    let project = ProjectType::detect(&state.language)?;

    match state.resource_type.as_str() {
        "contract" => {
            let contract_type = state.contract_type.unwrap_or(ContractType::Basic);

            // Convert our extensions to strings for the generator
            let _extension_strings: Vec<String> = state
                .extensions
                .into_iter()
                .map(|ext| extension_to_string(ext))
                .collect();

            let generator = GenericContractGenerator::new(
                project,
                state.language,
                state.name,
                contract_type,
                state.with_test,
                state.with_script,
                Some(state.pragma),
                Some(state.license),
            );
            generator.generate()?;
        }

        "library" => {
            let generator = LibraryGenerator::new(
                project,
                state.language,
                state.name,
                Some(state.pragma),
                Some(state.license),
            );
            generator.generate()?;
        }

        "script" => match project {
            ProjectType::Foundry(foundry_project) => {
                let generator =
                    ScriptGenerator::new(foundry_project, state.name, state.pragma, state.license);
                generator.generate()?;
            }
            _ => {
                return Err(anyhow::anyhow!(
                    "Script generation is only supported for Foundry projects"
                ))
            }
        },

        "test" => match project {
            ProjectType::Foundry(foundry_project) => {
                let generator =
                    TestGenerator::new(foundry_project, state.name, state.pragma, state.license);
                generator.generate()?;
            }
            _ => {
                return Err(anyhow::anyhow!(
                    "Test generation is only supported for Foundry projects"
                ))
            }
        },

        _ => {
            return Err(anyhow::anyhow!(
                "Unsupported resource type: {}",
                state.resource_type
            ))
        }
    }

    println!("\n{} Generation complete! Happy coding! ⚔️", "🎉".bold());

    Ok(())
}

fn extension_to_string(extension: gramr::TokenExtension) -> String {
    use gramr::TokenExtension::*;

    match extension {
        // ERC20
        ERC20Permit => "permit",
        ERC20Burnable => "burnable",
        ERC20Capped => "capped",
        ERC20Pausable => "pausable",
        ERC20Votes => "votes",
        ERC20Wrapper => "wrapper",
        ERC20FlashMint => "flashmint",
        ERC20TemporaryApproval => "temporaryapproval",
        ERC20Bridgeable => "bridgeable",
        ERC1363 => "erc1363",
        ERC4626 => "erc4626",

        // ERC721
        ERC721Pausable => "pausable",
        ERC721Burnable => "burnable",
        ERC721Consecutive => "consecutive",
        ERC721URIStorage => "uristorage",
        ERC721Votes => "votes",
        ERC721Royalty => "royalty",
        ERC721Wrapper => "wrapper",
        ERC721Enumerable => "enumerable",

        // ERC1155
        ERC1155Pausable => "pausable",
        ERC1155Burnable => "burnable",
        ERC1155Supply => "supply",
        ERC1155URIStorage => "uristorage",
    }
    .to_string()
}
