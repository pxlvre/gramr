use crate::error::{GramrError, Result};
use crate::foundry::FoundryProject;
use crate::templates::{ContractType, SolidityTemplate};
use colored::*;
use std::fs;

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
            return Err(GramrError::InvalidContractName(
                "Contract name cannot be empty".to_string(),
            ));
        }

        if !self.contract_name.chars().next().unwrap().is_alphabetic() {
            return Err(GramrError::InvalidContractName(
                "Contract name must start with a letter".to_string(),
            ));
        }

        if !self
            .contract_name
            .chars()
            .all(|c| c.is_alphanumeric() || c == '_')
        {
            return Err(GramrError::InvalidContractName(
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
            ContractType::Basic | ContractType::Interface | ContractType::Abstract => {}
        }
        Ok(())
    }

    fn create_contract_file(&self, template: &SolidityTemplate) -> Result<()> {
        let file_path = self
            .project
            .src_dir
            .join(format!("{}.sol", self.contract_name));

        if file_path.exists() {
            return Err(GramrError::FileExists(file_path.display().to_string()));
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
            return Err(GramrError::FileExists(file_path.display().to_string()));
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
            return Err(GramrError::FileExists(file_path.display().to_string()));
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::foundry::FoundryProject;
    use crate::templates::ContractType;
    use tempfile::TempDir;
    use std::fs;

    fn create_test_foundry_project() -> (TempDir, FoundryProject) {
        let temp_dir = TempDir::new().unwrap();
        let project_path = temp_dir.path().to_path_buf();
        
        // Create foundry.toml
        fs::write(
            project_path.join("foundry.toml"),
            "[profile.default]\nsrc = \"src\"\ntest = \"test\"\nscript = \"script\"\n",
        ).unwrap();
        
        // Create directories
        fs::create_dir_all(project_path.join("src")).unwrap();
        fs::create_dir_all(project_path.join("test")).unwrap();
        fs::create_dir_all(project_path.join("script")).unwrap();
        fs::create_dir_all(project_path.join("lib")).unwrap();
        
        let project = FoundryProject {
            root: project_path.clone(),
            src_dir: project_path.join("src"),
            test_dir: project_path.join("test"),
            script_dir: project_path.join("script"),
        };
        (temp_dir, project)
    }

    #[test]
    fn test_contract_generator_new() {
        let (_temp_dir, project) = create_test_foundry_project();
        
        let generator = ContractGenerator::new(
            project,
            "TestContract".to_string(),
            ContractType::Basic,
            true,
            true,
            "0.8.30".to_string(),
            "MIT".to_string(),
        );
        
        assert_eq!(generator.contract_name, "TestContract");
        assert!(matches!(generator.contract_type, ContractType::Basic));
        assert!(generator.with_test);
        assert!(generator.with_script);
        assert_eq!(generator.pragma, "0.8.30");
        assert_eq!(generator.license, "MIT");
    }

    #[test]
    fn test_validate_name_success() {
        let (_temp_dir, project) = create_test_foundry_project();
        
        let generator = ContractGenerator::new(
            project,
            "ValidName".to_string(),
            ContractType::Basic,
            false,
            false,
            "0.8.30".to_string(),
            "MIT".to_string(),
        );
        
        assert!(generator.validate_name().is_ok());
    }

    #[test]
    fn test_validate_name_with_underscores() {
        let (_temp_dir, project) = create_test_foundry_project();
        
        let generator = ContractGenerator::new(
            project,
            "Valid_Name_123".to_string(),
            ContractType::Basic,
            false,
            false,
            "0.8.30".to_string(),
            "MIT".to_string(),
        );
        
        assert!(generator.validate_name().is_ok());
    }

    #[test]
    fn test_validate_name_empty() {
        let (_temp_dir, project) = create_test_foundry_project();
        
        let generator = ContractGenerator::new(
            project,
            "".to_string(),
            ContractType::Basic,
            false,
            false,
            "0.8.30".to_string(),
            "MIT".to_string(),
        );
        
        let result = generator.validate_name();
        assert!(result.is_err());
        if let Err(GramrError::InvalidContractName(msg)) = result {
            assert!(msg.contains("cannot be empty"));
        } else {
            panic!("Expected InvalidContractName error");
        }
    }

    #[test]
    fn test_validate_name_starts_with_number() {
        let (_temp_dir, project) = create_test_foundry_project();
        
        let generator = ContractGenerator::new(
            project,
            "123Contract".to_string(),
            ContractType::Basic,
            false,
            false,
            "0.8.30".to_string(),
            "MIT".to_string(),
        );
        
        let result = generator.validate_name();
        assert!(result.is_err());
        if let Err(GramrError::InvalidContractName(msg)) = result {
            assert!(msg.contains("must start with a letter"));
        } else {
            panic!("Expected InvalidContractName error");
        }
    }

    #[test]
    fn test_validate_name_invalid_characters() {
        let (_temp_dir, project) = create_test_foundry_project();
        
        let generator = ContractGenerator::new(
            project,
            "Contract-Name".to_string(),
            ContractType::Basic,
            false,
            false,
            "0.8.30".to_string(),
            "MIT".to_string(),
        );
        
        let result = generator.validate_name();
        assert!(result.is_err());
        if let Err(GramrError::InvalidContractName(msg)) = result {
            assert!(msg.contains("can only contain letters, numbers, and underscores"));
        } else {
            panic!("Expected InvalidContractName error");
        }
    }

    #[test]
    fn test_create_contract_file_success() {
        let (_temp_dir, project) = create_test_foundry_project();
        let project_src = project.src_dir.clone();
        
        let generator = ContractGenerator::new(
            project,
            "TestContract".to_string(),
            ContractType::Basic,
            false,
            false,
            "0.8.30".to_string(),
            "MIT".to_string(),
        );
        
        let template = SolidityTemplate::new(
            "TestContract".to_string(),
            ContractType::Basic,
            "0.8.30".to_string(),
            "MIT".to_string(),
        );
        
        assert!(generator.create_contract_file(&template).is_ok());
        
        let contract_path = project_src.join("TestContract.sol");
        assert!(contract_path.exists());
        
        let content = fs::read_to_string(contract_path).unwrap();
        assert!(content.contains("TestContract"));
        assert!(content.contains("0.8.30"));
        assert!(content.contains("MIT"));
    }

    #[test]
    fn test_create_contract_file_already_exists() {
        let (_temp_dir, project) = create_test_foundry_project();
        
        // Create the file first
        let contract_path = project.src_dir.join("TestContract.sol");
        fs::write(&contract_path, "existing content").unwrap();
        
        let generator = ContractGenerator::new(
            project,
            "TestContract".to_string(),
            ContractType::Basic,
            false,
            false,
            "0.8.30".to_string(),
            "MIT".to_string(),
        );
        
        let template = SolidityTemplate::new(
            "TestContract".to_string(),
            ContractType::Basic,
            "0.8.30".to_string(),
            "MIT".to_string(),
        );
        
        let result = generator.create_contract_file(&template);
        assert!(result.is_err());
        if let Err(GramrError::FileExists(path)) = result {
            assert!(path.contains("TestContract.sol"));
        } else {
            panic!("Expected FileExists error");
        }
    }

    #[test]
    fn test_create_test_file_success() {
        let (_temp_dir, project) = create_test_foundry_project();
        let project_test = project.test_dir.clone();
        
        let generator = ContractGenerator::new(
            project,
            "TestContract".to_string(),
            ContractType::Basic,
            true,
            false,
            "0.8.30".to_string(),
            "MIT".to_string(),
        );
        
        let template = SolidityTemplate::new(
            "TestContract".to_string(),
            ContractType::Basic,
            "0.8.30".to_string(),
            "MIT".to_string(),
        );
        
        assert!(generator.create_test_file(&template).is_ok());
        
        let test_path = project_test.join("TestContract.t.sol");
        assert!(test_path.exists());
        
        let content = fs::read_to_string(test_path).unwrap();
        assert!(content.contains("TestContract"));
    }

    #[test]
    fn test_create_script_file_success() {
        let (_temp_dir, project) = create_test_foundry_project();
        let project_script = project.script_dir.clone();
        
        let generator = ContractGenerator::new(
            project,
            "TestContract".to_string(),
            ContractType::Basic,
            false,
            true,
            "0.8.30".to_string(),
            "MIT".to_string(),
        );
        
        let template = SolidityTemplate::new(
            "TestContract".to_string(),
            ContractType::Basic,
            "0.8.30".to_string(),
            "MIT".to_string(),
        );
        
        assert!(generator.create_script_file(&template).is_ok());
        
        let script_path = project_script.join("TestContract.s.sol");
        assert!(script_path.exists());
        
        let content = fs::read_to_string(script_path).unwrap();
        assert!(content.contains("TestContract"));
    }

    #[test]
    fn test_generate_basic_contract() {
        let (_temp_dir, project) = create_test_foundry_project();
        let project_src = project.src_dir.clone();
        
        let generator = ContractGenerator::new(
            project,
            "BasicContract".to_string(),
            ContractType::Basic,
            false,
            false,
            "0.8.30".to_string(),
            "MIT".to_string(),
        );
        
        assert!(generator.generate().is_ok());
        
        let contract_path = project_src.join("BasicContract.sol");
        assert!(contract_path.exists());
    }

    #[test]
    fn test_generate_with_test_and_script() {
        let (_temp_dir, project) = create_test_foundry_project();
        let project_src = project.src_dir.clone();
        let project_test = project.test_dir.clone();
        let project_script = project.script_dir.clone();
        
        let generator = ContractGenerator::new(
            project,
            "FullContract".to_string(),
            ContractType::Basic,
            true,
            true,
            "0.8.30".to_string(),
            "MIT".to_string(),
        );
        
        assert!(generator.generate().is_ok());
        
        let contract_path = project_src.join("FullContract.sol");
        let test_path = project_test.join("FullContract.t.sol");
        let script_path = project_script.join("FullContract.s.sol");
        
        assert!(contract_path.exists());
        assert!(test_path.exists());
        assert!(script_path.exists());
    }

    #[test]
    fn test_generate_invalid_name() {
        let (_temp_dir, project) = create_test_foundry_project();
        
        let generator = ContractGenerator::new(
            project,
            "".to_string(),
            ContractType::Basic,
            false,
            false,
            "0.8.30".to_string(),
            "MIT".to_string(),
        );
        
        let result = generator.generate();
        assert!(result.is_err());
    }

    #[test]
    fn test_dependency_checking_erc20() {
        let (_temp_dir, project) = create_test_foundry_project();
        
        let generator = ContractGenerator::new(
            project,
            "Token".to_string(),
            ContractType::ERC20,
            false,
            false,
            "0.8.30".to_string(),
            "MIT".to_string(),
        );
        
        // This should not fail even if OpenZeppelin is not installed
        // The method should handle installation internally
        assert!(generator.check_and_install_dependencies().is_ok());
    }

    #[test]
    fn test_dependency_checking_basic() {
        let (_temp_dir, project) = create_test_foundry_project();
        
        let generator = ContractGenerator::new(
            project,
            "Basic".to_string(),
            ContractType::Basic,
            false,
            false,
            "0.8.30".to_string(),
            "MIT".to_string(),
        );
        
        // Basic contracts should not require any dependencies
        assert!(generator.check_and_install_dependencies().is_ok());
    }

    #[test]
    fn test_contract_generator_builder_pattern() {
        let (_temp_dir, project) = create_test_foundry_project();
        
        let generator = ContractGenerator::new(
            project,
            "MyContract".to_string(),
            ContractType::ERC721,
            true,
            false,
            "0.8.25".to_string(),
            "Apache-2.0".to_string(),
        );
        
        assert_eq!(generator.contract_name, "MyContract");
        assert!(matches!(generator.contract_type, ContractType::ERC721));
        assert!(generator.with_test);
        assert!(!generator.with_script);
        assert_eq!(generator.pragma, "0.8.25");
        assert_eq!(generator.license, "Apache-2.0");
    }
}
