use crate::error::{NothungError, Result};
use crate::foundry::FoundryProject;
use crate::templates::{ContractType, SolidityTemplate};
use colored::*;
use std::fs;
use std::path::PathBuf;

pub struct ContractGenerator {
    project: FoundryProject,
    contract_name: String,
    contract_type: ContractType,
    with_test: bool,
    with_script: bool,
    pragma: String,
    license: String,
}

impl ContractGenerator {
    pub fn new(
        project: FoundryProject,
        contract_name: String,
        contract_type: ContractType,
        with_test: bool,
        with_script: bool,
        pragma: String,
        license: String,
    ) -> Self {
        Self {
            project,
            contract_name,
            contract_type,
            with_test,
            with_script,
            pragma,
            license,
        }
    }

    pub fn generate(&self) -> Result<()> {
        self.validate_name()?;

        self.project.ensure_directories()?;

        self.check_and_install_dependencies()?;

        let template = SolidityTemplate::new(
            self.contract_name.clone(),
            self.contract_type.clone(),
            self.pragma.clone(),
            self.license.clone(),
        );

        self.create_contract_file(&template)?;

        if self.with_test {
            self.create_test_file(&template)?;
        }

        if self.with_script {
            self.create_script_file(&template)?;
        }

        self.print_success();

        Ok(())
    }

    fn validate_name(&self) -> Result<()> {
        if self.contract_name.is_empty() {
            return Err(NothungError::InvalidContractName(
                "Contract name cannot be empty".to_string(),
            ));
        }

        if !self.contract_name.chars().next().unwrap().is_alphabetic() {
            return Err(NothungError::InvalidContractName(
                "Contract name must start with a letter".to_string(),
            ));
        }

        if !self
            .contract_name
            .chars()
            .all(|c| c.is_alphanumeric() || c == '_')
        {
            return Err(NothungError::InvalidContractName(
                "Contract name can only contain letters, numbers, and underscores".to_string(),
            ));
        }

        Ok(())
    }

    fn check_and_install_dependencies(&self) -> Result<()> {
        match &self.contract_type {
            ContractType::ERC20 | ContractType::ERC721 | ContractType::ERC1155 => {
                if !self.project.has_openzeppelin() {
                    println!("{} Installing OpenZeppelin contracts...", "→".yellow());
                    self.project.install_openzeppelin()?;
                    println!("{} OpenZeppelin contracts installed", "✓".green());
                }
            }
            ContractType::ERC20Upgradeable
            | ContractType::ERC721Upgradeable
            | ContractType::ERC1155Upgradeable => {
                if !self.project.has_openzeppelin_upgradeable() {
                    println!(
                        "{} Installing OpenZeppelin upgradeable contracts...",
                        "→".yellow()
                    );
                    self.project.install_openzeppelin_upgradeable()?;
                    println!(
                        "{} OpenZeppelin upgradeable contracts installed",
                        "✓".green()
                    );
                }
            }
            ContractType::MultiInheritance { base_type, .. } => match **base_type {
                ContractType::ERC20 | ContractType::ERC721 | ContractType::ERC1155 => {
                    if !self.project.has_openzeppelin() {
                        println!("{} Installing OpenZeppelin contracts...", "→".yellow());
                        self.project.install_openzeppelin()?;
                        println!("{} OpenZeppelin contracts installed", "✓".green());
                    }
                }
                ContractType::ERC20Upgradeable
                | ContractType::ERC721Upgradeable
                | ContractType::ERC1155Upgradeable => {
                    if !self.project.has_openzeppelin_upgradeable() {
                        println!(
                            "{} Installing OpenZeppelin upgradeable contracts...",
                            "→".yellow()
                        );
                        self.project.install_openzeppelin_upgradeable()?;
                        println!(
                            "{} OpenZeppelin upgradeable contracts installed",
                            "✓".green()
                        );
                    }
                }
                _ => {}
            },
            ContractType::Basic => {}
        }
        Ok(())
    }

    fn create_contract_file(&self, template: &SolidityTemplate) -> Result<()> {
        let file_path = self
            .project
            .src_dir
            .join(format!("{}.sol", self.contract_name));

        if file_path.exists() {
            return Err(NothungError::FileExists(file_path.display().to_string()));
        }

        let content = template.generate_contract();
        fs::write(&file_path, content)?;

        println!("{} Created contract: {}", "✓".green(), file_path.display());
        Ok(())
    }

    fn create_test_file(&self, template: &SolidityTemplate) -> Result<()> {
        let file_path = self
            .project
            .test_dir
            .join(format!("{}.t.sol", self.contract_name));

        if file_path.exists() {
            return Err(NothungError::FileExists(file_path.display().to_string()));
        }

        let content = template.generate_test();
        fs::write(&file_path, content)?;

        println!("{} Created test: {}", "✓".green(), file_path.display());
        Ok(())
    }

    fn create_script_file(&self, template: &SolidityTemplate) -> Result<()> {
        let file_path = self
            .project
            .script_dir
            .join(format!("{}.s.sol", self.contract_name));

        if file_path.exists() {
            return Err(NothungError::FileExists(file_path.display().to_string()));
        }

        let content = template.generate_script();
        fs::write(&file_path, content)?;

        println!("{} Created script: {}", "✓".green(), file_path.display());
        Ok(())
    }

    fn print_success(&self) {
        println!("\n{} Contract generation complete!", "🎉".green());
        println!("\n{}", "Next steps:".bold());
        println!("  1. Review the generated files");
        println!("  2. Run {} to compile", "forge build".cyan());
        if self.with_test {
            println!("  3. Run {} to test", "forge test".cyan());
        }
    }
}
