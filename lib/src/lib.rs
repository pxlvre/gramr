//! Gramr - A blazing-fast library for scaffolding smart contracts
//! 
//! This library provides the core functionality for generating Solidity and Rust/Stylus contracts,
//! tests, and deployment scripts for Foundry and Cargo projects.

pub mod error;
pub mod foundry;
pub mod generators;
pub mod templates;
pub mod language;
pub mod project;

// Re-export commonly used types
pub use error::{GramrError, Result};
pub use foundry::FoundryProject;
pub use generators::{ContractGenerator, ScriptGenerator, TestGenerator, GenericContractGenerator, LibraryGenerator, InterfaceGenerator, AbstractContractGenerator, ConfigGenerator};
pub use templates::{ContractType, TokenExtension, SolidityTemplate, StylusTemplate};
pub use language::Language;
pub use project::{Project, ProjectType, CargoProject};

// Version info
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Builder pattern for creating contracts programmatically
pub struct ContractBuilder {
    name: String,
    contract_type: ContractType,
    pragma: String,
    license: String,
}

impl ContractBuilder {
    /// Create a new contract builder
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            contract_type: ContractType::Basic,
            pragma: "0.8.30".to_string(),
            license: "UNLICENSED".to_string(),
        }
    }

    /// Set the contract type
    pub fn contract_type(mut self, contract_type: ContractType) -> Self {
        self.contract_type = contract_type;
        self
    }

    /// Set the pragma version
    pub fn pragma(mut self, pragma: impl Into<String>) -> Self {
        self.pragma = pragma.into();
        self
    }

    /// Set the license
    pub fn license(mut self, license: impl Into<String>) -> Self {
        self.license = license.into();
        self
    }

    /// Generate the contract source code
    pub fn build(self) -> String {
        let template = SolidityTemplate::new(
            self.name,
            self.contract_type,
            self.pragma,
            self.license,
        );
        template.generate_contract()
    }

    /// Generate and write the contract to a Foundry project
    pub fn generate(self, project: FoundryProject) -> Result<()> {
        let generator = ContractGenerator::new(
            project,
            self.name.clone(),
            self.contract_type,
            false,
            false,
            self.pragma,
            self.license,
        );
        generator.generate()
    }
}

/// Helper function to parse extensions from strings
pub fn parse_extensions(extensions: &[String]) -> Result<Vec<TokenExtension>> {
    use templates::TokenExtension;
    
    let mut result = Vec::new();
    
    for ext in extensions {
        let extension = match ext.to_lowercase().as_str() {
            // ERC20 Extensions
            "permit" => TokenExtension::ERC20Permit,
            "burnable" => TokenExtension::ERC20Burnable,
            "capped" => TokenExtension::ERC20Capped,
            "pausable" => TokenExtension::ERC20Pausable,
            "votes" => TokenExtension::ERC20Votes,
            "wrapper" => TokenExtension::ERC20Wrapper,
            "flashmint" => TokenExtension::ERC20FlashMint,
            "temporaryapproval" => TokenExtension::ERC20TemporaryApproval,
            "bridgeable" => TokenExtension::ERC20Bridgeable,
            "erc1363" => TokenExtension::ERC1363,
            "erc4626" => TokenExtension::ERC4626,
            
            // ERC721 Extensions  
            "consecutive" => TokenExtension::ERC721Consecutive,
            "uristorage" => TokenExtension::ERC721URIStorage,
            "royalty" => TokenExtension::ERC721Royalty,
            "enumerable" => TokenExtension::ERC721Enumerable,
            
            // ERC1155 Extensions
            "supply" => TokenExtension::ERC1155Supply,
            
            _ => return Err(GramrError::Other(
                format!("Unknown extension: {}. Available extensions: permit, burnable, capped, pausable, votes, wrapper, flashmint, temporaryapproval, bridgeable, erc1363, erc4626, consecutive, uristorage, royalty, wrapper, enumerable, supply", ext)
            )),
        };
        
        result.push(extension);
    }
    
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_constant() {
        assert!(!VERSION.is_empty());
        assert!(VERSION.chars().any(|c| c.is_ascii_digit()));
    }

    mod contract_builder_tests {
        use super::*;

        #[test]
        fn test_new_contract_builder() {
            let builder = ContractBuilder::new("TestContract");
            assert_eq!(builder.name, "TestContract");
            assert!(matches!(builder.contract_type, ContractType::Basic));
            assert_eq!(builder.pragma, "0.8.30");
            assert_eq!(builder.license, "UNLICENSED");
        }

        #[test]
        fn test_contract_builder_chain() {
            let builder = ContractBuilder::new("MyToken")
                .contract_type(ContractType::ERC20)
                .pragma("0.8.25")
                .license("MIT");
            
            assert_eq!(builder.name, "MyToken");
            assert!(matches!(builder.contract_type, ContractType::ERC20));
            assert_eq!(builder.pragma, "0.8.25");
            assert_eq!(builder.license, "MIT");
        }

        #[test]
        fn test_contract_builder_with_different_types() {
            let erc20_builder = ContractBuilder::new("Token").contract_type(ContractType::ERC20);
            assert!(matches!(erc20_builder.contract_type, ContractType::ERC20));

            let erc721_builder = ContractBuilder::new("NFT").contract_type(ContractType::ERC721);
            assert!(matches!(erc721_builder.contract_type, ContractType::ERC721));

            let erc1155_builder = ContractBuilder::new("Multi").contract_type(ContractType::ERC1155);
            assert!(matches!(erc1155_builder.contract_type, ContractType::ERC1155));
        }

        #[test]
        fn test_build_basic_contract() {
            let contract = ContractBuilder::new("BasicContract")
                .pragma("0.8.20")
                .license("MIT")
                .build();
            
            assert!(contract.contains("BasicContract"));
            assert!(contract.contains("0.8.20"));
            assert!(contract.contains("MIT"));
            assert!(contract.contains("pragma solidity"));
        }

        #[test]
        fn test_build_erc20_contract() {
            let contract = ContractBuilder::new("MyToken")
                .contract_type(ContractType::ERC20)
                .build();
            
            assert!(contract.contains("MyToken"));
            assert!(contract.contains("ERC20"));
            assert!(contract.contains("import"));
        }

        #[test]
        fn test_build_with_empty_name() {
            let contract = ContractBuilder::new("")
                .build();
            
            // Should still generate valid solidity
            assert!(contract.contains("pragma solidity"));
        }

        #[test]
        fn test_build_with_special_characters_in_name() {
            let contract = ContractBuilder::new("My_Token_123")
                .build();
            
            assert!(contract.contains("My_Token_123"));
        }
    }

    mod parse_extensions_tests {
        use super::*;

        #[test]
        fn test_parse_empty_extensions() {
            let result = parse_extensions(&[]);
            assert!(result.is_ok());
            assert!(result.unwrap().is_empty());
        }

        #[test]
        fn test_parse_single_erc20_extension() {
            let result = parse_extensions(&["burnable".to_string()]);
            assert!(result.is_ok());
            let extensions = result.unwrap();
            assert_eq!(extensions.len(), 1);
            assert!(matches!(extensions[0], TokenExtension::ERC20Burnable));
        }

        #[test]
        fn test_parse_multiple_erc20_extensions() {
            let result = parse_extensions(&[
                "burnable".to_string(),
                "pausable".to_string(),
                "permit".to_string(),
            ]);
            assert!(result.is_ok());
            let extensions = result.unwrap();
            assert_eq!(extensions.len(), 3);
        }

        #[test]
        fn test_parse_erc721_extensions() {
            let result = parse_extensions(&[
                "enumerable".to_string(),
                "uristorage".to_string(),
                "royalty".to_string(),
            ]);
            assert!(result.is_ok());
            let extensions = result.unwrap();
            assert_eq!(extensions.len(), 3);
            assert!(matches!(extensions[0], TokenExtension::ERC721Enumerable));
            assert!(matches!(extensions[1], TokenExtension::ERC721URIStorage));
            assert!(matches!(extensions[2], TokenExtension::ERC721Royalty));
        }

        #[test]
        fn test_parse_erc1155_extensions() {
            let result = parse_extensions(&["supply".to_string()]);
            assert!(result.is_ok());
            let extensions = result.unwrap();
            assert_eq!(extensions.len(), 1);
            assert!(matches!(extensions[0], TokenExtension::ERC1155Supply));
        }

        #[test]
        fn test_parse_case_insensitive() {
            let result = parse_extensions(&[
                "BURNABLE".to_string(),
                "Pausable".to_string(),
                "peRmIt".to_string(),
            ]);
            assert!(result.is_ok());
            let extensions = result.unwrap();
            assert_eq!(extensions.len(), 3);
        }

        #[test]
        fn test_parse_unknown_extension() {
            let result = parse_extensions(&["unknown".to_string()]);
            assert!(result.is_err());
            
            if let Err(GramrError::Other(msg)) = result {
                assert!(msg.contains("Unknown extension: unknown"));
                assert!(msg.contains("Available extensions:"));
            } else {
                panic!("Expected GramrError::Other");
            }
        }

        #[test]
        fn test_parse_mixed_valid_invalid() {
            let result = parse_extensions(&[
                "burnable".to_string(),
                "invalid".to_string(),
            ]);
            assert!(result.is_err());
        }

        #[test]
        fn test_parse_all_erc20_extensions() {
            let extensions = vec![
                "permit".to_string(),
                "burnable".to_string(),
                "capped".to_string(),
                "pausable".to_string(),
                "votes".to_string(),
                "wrapper".to_string(),
                "flashmint".to_string(),
                "temporaryapproval".to_string(),
                "bridgeable".to_string(),
                "erc1363".to_string(),
                "erc4626".to_string(),
            ];
            
            let result = parse_extensions(&extensions);
            assert!(result.is_ok());
            assert_eq!(result.unwrap().len(), 11);
        }

        #[test]
        fn test_parse_duplicate_extensions() {
            let result = parse_extensions(&[
                "burnable".to_string(),
                "burnable".to_string(),
            ]);
            assert!(result.is_ok());
            let extensions = result.unwrap();
            assert_eq!(extensions.len(), 2); // Duplicates allowed
        }

        #[test]
        fn test_parse_whitespace_extensions() {
            let result = parse_extensions(&[" burnable ".to_string()]);
            assert!(result.is_err()); // Whitespace not trimmed
        }
    }

    #[test]
    fn test_module_exports() {
        // Test that key types are properly exported
        let _error: GramrError = GramrError::Other("test".to_string());
        let _lang = Language::Solidity;
        let _contract_type = ContractType::Basic;
        let _token_ext = TokenExtension::ERC20Burnable;
        
        // This test ensures the public API is accessible
        assert!(true);
    }

    #[test]
    fn test_contract_builder_string_conversions() {
        let builder = ContractBuilder::new("Test".to_string())
            .pragma("0.8.19".to_string())
            .license("Apache-2.0".to_string());
        
        assert_eq!(builder.name, "Test");
        assert_eq!(builder.pragma, "0.8.19");
        assert_eq!(builder.license, "Apache-2.0");
    }

    #[test]
    fn test_contract_builder_str_conversions() {
        let builder = ContractBuilder::new("Test")
            .pragma("0.8.19")
            .license("Apache-2.0");
        
        assert_eq!(builder.name, "Test");
        assert_eq!(builder.pragma, "0.8.19");
        assert_eq!(builder.license, "Apache-2.0");
    }
}