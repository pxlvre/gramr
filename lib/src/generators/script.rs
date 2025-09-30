use crate::error::{GramrError, Result};
use crate::foundry::FoundryProject;
use colored::*;
use std::fs;

pub struct ScriptGenerator {
    project: FoundryProject,
    script_name: String,
    pragma: String,
    license: String,
}

impl ScriptGenerator {
    pub fn new(
        project: FoundryProject,
        script_name: String,
        pragma: String,
        license: String,
    ) -> Self {
        Self {
            project,
            script_name,
            pragma,
            license,
        }
    }

    pub fn generate(&self) -> Result<()> {
        self.validate_name()?;
        self.project.ensure_directories()?;
        self.create_script_file()?;
        self.print_success();
        Ok(())
    }

    fn validate_name(&self) -> Result<()> {
        if self.script_name.is_empty() {
            return Err(GramrError::InvalidContractName(
                "Script name cannot be empty".to_string(),
            ));
        }

        if !self.script_name.chars().next().unwrap().is_alphabetic() {
            return Err(GramrError::InvalidContractName(
                "Script name must start with a letter".to_string(),
            ));
        }

        if !self
            .script_name
            .chars()
            .all(|c| c.is_alphanumeric() || c == '_')
        {
            return Err(GramrError::InvalidContractName(
                "Script name can only contain letters, numbers, and underscores".to_string(),
            ));
        }

        Ok(())
    }

    fn create_script_file(&self) -> Result<()> {
        let file_path = self
            .project
            .script_dir
            .join(format!("{}.s.sol", self.script_name));

        if file_path.exists() {
            return Err(GramrError::FileExists(file_path.display().to_string()));
        }

        let content = self.generate_script_content();
        fs::write(&file_path, content)?;

        println!("{} Created script: {}", "✓".green(), file_path.display());
        Ok(())
    }

    fn generate_script_content(&self) -> String {
        format!(
            r#"// SPDX-License-Identifier: {}
pragma solidity ^{};

import "forge-std/Script.sol";

contract {} is Script {{
    function setUp() public {{}}

    function run() public {{
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        vm.startBroadcast(deployerPrivateKey);

        // Deploy contracts here

        vm.stopBroadcast();
    }}
}}"#,
            self.license, self.pragma, self.script_name
        )
    }

    fn print_success(&self) {
        println!("\n{} Script generation complete!", "🎉".green());
        println!("\n{}", "Next steps:".bold());
        println!("  1. Add your deployment logic to the run() function");
        println!("  2. Set the PRIVATE_KEY environment variable");
        println!(
            "  3. Run {} to execute",
            format!("forge script script/{}.s.sol", self.script_name).cyan()
        );
    }
}
