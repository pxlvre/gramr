use crate::error::{NothungError, Result};
use crate::language::Language;
use crate::project::{Project, ProjectType};
use crate::templates::{ContractType, Template, SolidityTemplate, StylusTemplate};
use colored::*;
use std::fs;

pub struct GenericContractGenerator {
    project: ProjectType,
    language: Language,
    contract_name: String,
    contract_type: ContractType,
    with_test: bool,
    with_script: bool,
    pragma: Option<String>,  // Only for Solidity
    license: Option<String>, // Only for Solidity
}

impl GenericContractGenerator {
    pub fn new(
        project: ProjectType,
        language: Language,
        contract_name: String,
        contract_type: ContractType,
        with_test: bool,
        with_script: bool,
        pragma: Option<String>,
        license: Option<String>,
    ) -> Self {
        Self {
            project,
            language,
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
        self.validate_language_compatibility()?;
        self.project.ensure_directories()?;
        self.check_and_install_dependencies()?;

        let template: Box<dyn Template> = match self.language {
            Language::Solidity => {
                Box::new(SolidityTemplate::new(
                    self.contract_name.clone(),
                    self.contract_type.clone(),
                    self.pragma.clone().unwrap_or_else(|| "0.8.30".to_string()),
                    self.license.clone().unwrap_or_else(|| "UNLICENSED".to_string()),
                ))
            }
            Language::RustStylus => {
                Box::new(StylusTemplate::new(
                    self.contract_name.clone(),
                    self.contract_type.clone(),
                ))
            }
        };

        self.create_contract_file(&*template)?;

        if self.with_test {
            self.create_test_file(&*template)?;
        }

        if self.with_script {
            self.create_script_file(&*template)?;
        }

        self.print_success();
        Ok(())
    }

    fn validate_name(&self) -> Result<()> {
        if self.contract_name.is_empty() {
            return Err(NothungError::Other("Contract name cannot be empty".to_string()));
        }

        if !self.contract_name.chars().next().unwrap().is_alphabetic() {
            return Err(NothungError::Other(
                "Contract name must start with a letter".to_string()
            ));
        }

        if !self.contract_name.chars().all(|c| c.is_alphanumeric() || c == '_') {
            return Err(NothungError::Other(
                "Contract name can only contain letters, numbers, and underscores".to_string()
            ));
        }

        Ok(())
    }
    
    fn validate_language_compatibility(&self) -> Result<()> {
        if self.language == Language::RustStylus {
            if self.with_test {
                return Err(NothungError::Other(
                    "Test generation (--with-test) is not supported for Rust/Stylus projects. Use 'cargo test' instead.".to_string()
                ));
            }
            
            if self.with_script {
                return Err(NothungError::Other(
                    "Script generation (--with-script) is not supported for Rust/Stylus projects. Use deployment tools like 'stylus deploy' instead.".to_string()
                ));
            }
        }
        
        Ok(())
    }

    fn check_and_install_dependencies(&self) -> Result<()> {
        match &self.contract_type {
            ContractType::ERC20 | ContractType::ERC721 | ContractType::ERC1155 => {
                if !self.project.has_openzeppelin() {
                    let lib_name = match self.language {
                        Language::Solidity => "OpenZeppelin contracts",
                        Language::RustStylus => "OpenZeppelin Stylus",
                    };
                    println!("{} Installing {}...", "→".yellow(), lib_name);
                    self.project.install_openzeppelin()?;
                    println!("{} {} installed", "✓".green(), lib_name);
                }
            }
            ContractType::ERC20Upgradeable
            | ContractType::ERC721Upgradeable
            | ContractType::ERC1155Upgradeable => {
                if !self.project.has_openzeppelin_upgradeable() {
                    let lib_name = match self.language {
                        Language::Solidity => "OpenZeppelin upgradeable contracts",
                        Language::RustStylus => "OpenZeppelin Stylus (upgradeable)",
                    };
                    println!("{} Installing {}...", "→".yellow(), lib_name);
                    self.project.install_openzeppelin_upgradeable()?;
                    println!("{} {} installed", "✓".green(), lib_name);
                }
            }
            ContractType::MultiInheritance { base_type, .. } => {
                match **base_type {
                    ContractType::ERC20 | ContractType::ERC721 | ContractType::ERC1155 => {
                        if !self.project.has_openzeppelin() {
                            let lib_name = match self.language {
                                Language::Solidity => "OpenZeppelin contracts",
                                Language::RustStylus => "OpenZeppelin Stylus",
                            };
                            println!("{} Installing {}...", "→".yellow(), lib_name);
                            self.project.install_openzeppelin()?;
                            println!("{} {} installed", "✓".green(), lib_name);
                        }
                    }
                    ContractType::ERC20Upgradeable
                    | ContractType::ERC721Upgradeable
                    | ContractType::ERC1155Upgradeable => {
                        if !self.project.has_openzeppelin_upgradeable() {
                            let lib_name = match self.language {
                                Language::Solidity => "OpenZeppelin upgradeable contracts",
                                Language::RustStylus => "OpenZeppelin Stylus (upgradeable)",
                            };
                            println!("{} Installing {}...", "→".yellow(), lib_name);
                            self.project.install_openzeppelin_upgradeable()?;
                            println!("{} {} installed", "✓".green(), lib_name);
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }

        Ok(())
    }

    fn create_contract_file(&self, template: &dyn Template) -> Result<()> {
        let content = template.generate_contract();
        let file_extension = match self.language {
            Language::Solidity => "sol",
            Language::RustStylus => "rs",
        };
        let file_path = self.project.src_dir().join(format!("{}.{}", self.contract_name, file_extension));
        
        fs::write(&file_path, content)
            .map_err(|e| NothungError::Other(format!("Failed to write contract file: {}", e)))?;
        
        println!("{} Created contract: {}", "✓".green(), file_path.display());
        Ok(())
    }

    fn create_test_file(&self, template: &dyn Template) -> Result<()> {
        let content = template.generate_test();
        let (_file_name, file_path) = match self.language {
            Language::Solidity => {
                let name = format!("{}.t.sol", self.contract_name);
                let path = self.project.test_dir().join(&name);
                (name, path)
            }
            Language::RustStylus => {
                let name = format!("{}_test.rs", self.contract_name.to_lowercase());
                let path = self.project.test_dir().join(&name);
                (name, path)
            }
        };
        
        fs::write(&file_path, content)
            .map_err(|e| NothungError::Other(format!("Failed to write test file: {}", e)))?;
        
        println!("{} Created test: {}", "✓".green(), file_path.display());
        Ok(())
    }

    fn create_script_file(&self, template: &dyn Template) -> Result<()> {
        let content = template.generate_script();
        
        match self.language {
            Language::Solidity => {
                let file_name = format!("Deploy{}.s.sol", self.contract_name);
                let file_path = self.project.script_dir().join(file_name);
                
                fs::write(&file_path, content)
                    .map_err(|e| NothungError::Other(format!("Failed to write script file: {}", e)))?;
                
                println!("{} Created script: {}", "✓".green(), file_path.display());
            }
            Language::RustStylus => {
                // For Stylus, we create a deployment instructions file
                let file_name = format!("{}_deploy.md", self.contract_name.to_lowercase());
                let file_path = self.project.script_dir().join(file_name);
                
                // Ensure scripts directory exists
                fs::create_dir_all(self.project.script_dir())
                    .map_err(|e| NothungError::Other(format!("Failed to create scripts directory: {}", e)))?;
                
                fs::write(&file_path, content)
                    .map_err(|e| NothungError::Other(format!("Failed to write deployment instructions: {}", e)))?;
                
                println!("{} Created deployment instructions: {}", "✓".green(), file_path.display());
            }
        }
        
        Ok(())
    }

    fn print_success(&self) {
        println!("\n{} Contract generation complete!", "🎉".bold());
        println!("\nNext steps:");
        println!("  1. Review the generated files");
        
        match self.language {
            Language::Solidity => {
                println!("  2. Run {} to compile", "forge build".cyan());
                if self.with_test {
                    println!("  3. Run {} to test", "forge test".cyan());
                }
                if self.with_script {
                    println!("  4. Deploy with {}", "forge script".cyan());
                }
            }
            Language::RustStylus => {
                println!("  2. Run {} to build", "cargo build --release".cyan());
                if self.with_test {
                    println!("  3. Run {} to test", "cargo test".cyan());
                }
                println!("  4. Deploy with {}", "stylus deploy".cyan());
            }
        }
    }
}