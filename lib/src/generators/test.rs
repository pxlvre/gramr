use crate::error::{GramrError, Result};
use crate::foundry::FoundryProject;
use colored::*;
use std::fs;

pub struct TestGenerator {
    project: FoundryProject,
    test_name: String,
    pragma: String,
    license: String,
}

impl TestGenerator {
    pub fn new(
        project: FoundryProject,
        test_name: String,
        pragma: String,
        license: String,
    ) -> Self {
        Self {
            project,
            test_name,
            pragma,
            license,
        }
    }

    pub fn generate(&self) -> Result<()> {
        self.validate_name()?;
        self.project.ensure_directories()?;
        self.create_test_file()?;
        self.print_success();
        Ok(())
    }

    fn validate_name(&self) -> Result<()> {
        if self.test_name.is_empty() {
            return Err(GramrError::InvalidContractName(
                "Test name cannot be empty".to_string(),
            ));
        }

        if !self.test_name.chars().next().unwrap().is_alphabetic() {
            return Err(GramrError::InvalidContractName(
                "Test name must start with a letter".to_string(),
            ));
        }

        if !self
            .test_name
            .chars()
            .all(|c| c.is_alphanumeric() || c == '_')
        {
            return Err(GramrError::InvalidContractName(
                "Test name can only contain letters, numbers, and underscores".to_string(),
            ));
        }

        Ok(())
    }

    fn create_test_file(&self) -> Result<()> {
        let file_path = self
            .project
            .test_dir
            .join(format!("{}.t.sol", self.test_name));

        if file_path.exists() {
            return Err(GramrError::FileExists(file_path.display().to_string()));
        }

        let content = self.generate_test_content();
        fs::write(&file_path, content)?;

        println!("{} Created test: {}", "✓".green(), file_path.display());
        Ok(())
    }

    fn generate_test_content(&self) -> String {
        format!(
            r#"// SPDX-License-Identifier: {}
pragma solidity ^{};

import "forge-std/Test.sol";

contract {} is Test {{
    function setUp() public {{
        // Setup test environment
    }}

    function test_Example() public {{
        // Write your test here
        assertTrue(true);
    }}

    function testFuzz_Example(uint256 value) public {{
        // Write your fuzz test here
        assertGe(value, 0);
    }}
}}"#,
            self.license, self.pragma, self.test_name
        )
    }

    fn print_success(&self) {
        println!("\n{} Test generation complete!", "🎉".green());
        println!("\n{}", "Next steps:".bold());
        println!("  1. Write your test cases");
        println!("  2. Run {} to execute tests", "forge test".cyan());
        println!("  3. Run {} for verbose output", "forge test -vvv".cyan());
    }
}
