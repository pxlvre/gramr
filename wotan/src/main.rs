mod wizard;

use anyhow::Result;
use clap::Command;
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
    let _app = Command::new("wotan")
        .version(env!("CARGO_PKG_VERSION"))
        .about("🧙‍♂️ Interactive wizard for generating smart contracts with Gramr")
        .long_about("Wotan is the interactive wizard companion to Gramr that guides you through creating smart contracts, libraries, scripts, and tests step by step.")
        .get_matches();

    // Run the interactive wizard
    let wizard = ContractWizard::new();
    let state = wizard.run()?;

    // Generate the files based on wizard choices
    generate_from_state(state)?;

    Ok(())
}


fn generate_from_state(state: WizardState) -> Result<()> {
    println!("\n{} Generating files...", "🔨".bold());

    // Detect project
    let project = ProjectType::detect(&state.language)?;

    match state.resource_type.as_str() {
        "contract" => {
            let base_contract_type = state.contract_type.unwrap_or(ContractType::Basic);
            
            // If extensions are present, use MultiInheritance contract type
            let contract_type = if !state.extensions.is_empty() {
                ContractType::MultiInheritance {
                    base_type: Box::new(base_contract_type),
                    extensions: state.extensions,
                }
            } else {
                base_contract_type
            };

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

