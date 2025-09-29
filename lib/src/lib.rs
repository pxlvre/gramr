//! Nothung - A blazing-fast library for scaffolding smart contracts
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
pub use error::{NothungError, Result};
pub use foundry::FoundryProject;
pub use generators::{ContractGenerator, ScriptGenerator, TestGenerator, GenericContractGenerator, LibraryGenerator};
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
            
            _ => return Err(NothungError::Other(
                format!("Unknown extension: {}. Available extensions: permit, burnable, capped, pausable, votes, wrapper, flashmint, temporaryapproval, bridgeable, erc1363, erc4626, consecutive, uristorage, royalty, wrapper, enumerable, supply", ext)
            )),
        };
        
        result.push(extension);
    }
    
    Ok(result)
}