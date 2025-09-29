use super::{ContractType, TokenExtension, Template};

pub struct StylusTemplate {
    contract_name: String,
    contract_type: ContractType,
}

impl StylusTemplate {
    pub fn new(contract_name: String, contract_type: ContractType) -> Self {
        Self {
            contract_name,
            contract_type,
        }
    }
    
    fn generate_basic_contract(&self) -> String {
        format!(
r#"#![cfg_attr(not(feature = "export-abi"), no_std)]

extern crate alloc;

use stylus_sdk::{{prelude::*, msg}};
use alloc::string::String;
use alloc::vec::Vec;

/// Entrypoint for the {} contract
#[entrypoint]
#[storage]
pub struct {} {{
    owner: StorageAddress,
}}

#[public]
impl {} {{
    /// Constructor
    pub fn init(&mut self) -> Result<(), Vec<u8>> {{
        self.owner.set(msg::sender());
        Ok(())
    }}
    
    /// Get the owner
    pub fn owner(&self) -> Address {{
        self.owner.get()
    }}
}}
"#,
            self.contract_name,
            self.contract_name,
            self.contract_name
        )
    }
    
    fn generate_erc20_contract(&self) -> String {
        let symbol = self.get_symbol();
        format!(
r#"#![cfg_attr(not(feature = "export-abi"), no_std)]

extern crate alloc;

use openzeppelin_stylus::{{
    token::erc20::{{
        Erc20, IErc20, IErc20Metadata,
        extensions::{{Burnable, IBurnable}},
    }},
}};
use stylus_sdk::{{
    msg,
    prelude::*,
}};
use alloc::string::String;

/// {} ERC20 Token
#[entrypoint]
#[storage]
struct {} {{
    #[borrow]
    erc20: Erc20,
}}

#[public]
#[inherit(Erc20)]
impl {} {{
    /// Initialize the contract with an initial supply
    pub fn init(&mut self, initial_supply: U256) -> Result<(), Vec<u8>> {{
        self.erc20._metadata.name.set(String::from("{}"));
        self.erc20._metadata.symbol.set(String::from("{}"));
        
        // Mint initial supply to deployer
        let deployer = msg::sender();
        self.erc20._mint(deployer, initial_supply)?;
        
        Ok(())
    }}
    
    /// Mint new tokens
    pub fn mint(&mut self, to: Address, amount: U256) -> Result<(), Vec<u8>> {{
        // Add access control as needed
        self.erc20._mint(to, amount)?;
        Ok(())
    }}
}}
"#,
            self.contract_name,
            self.contract_name,
            self.contract_name,
            self.contract_name,
            symbol
        )
    }
    
    fn generate_erc721_contract(&self) -> String {
        let symbol = self.get_symbol();
        format!(
r#"#![cfg_attr(not(feature = "export-abi"), no_std)]

extern crate alloc;

use openzeppelin_stylus::{{
    token::erc721::{{
        Erc721, IErc721, IErc721Metadata,
        extensions::{{enumerable::{{Erc721Enumerable, IErc721Enumerable}}}},
    }},
}};
use stylus_sdk::{{
    msg,
    prelude::*,
}};
use alloc::string::String;

/// {} NFT Collection
#[entrypoint]
#[storage]
struct {} {{
    #[borrow]
    erc721: Erc721,
    #[borrow]
    enumerable: Erc721Enumerable,
    next_token_id: StorageU256,
}}

#[public]
#[inherit(Erc721, Erc721Enumerable)]
impl {} {{
    /// Initialize the NFT collection
    pub fn init(&mut self) -> Result<(), Vec<u8>> {{
        self.erc721._metadata.name.set(String::from("{}"));
        self.erc721._metadata.symbol.set(String::from("{}"));
        self.next_token_id.set(U256::from(1));
        Ok(())
    }}
    
    /// Mint a new NFT
    pub fn mint(&mut self, to: Address) -> Result<U256, Vec<u8>> {{
        let token_id = self.next_token_id.get();
        self.erc721._mint(to, token_id)?;
        self.next_token_id.set(token_id + U256::from(1));
        Ok(token_id)
    }}
    
    /// Set token URI
    pub fn set_token_uri(&mut self, token_id: U256, uri: String) -> Result<(), Vec<u8>> {{
        // Implement URI storage as needed
        Ok(())
    }}
}}
"#,
            self.contract_name,
            self.contract_name,
            self.contract_name,
            self.contract_name,
            symbol
        )
    }
    
    fn generate_erc1155_contract(&self) -> String {
        format!(
r#"#![cfg_attr(not(feature = "export-abi"), no_std)]

extern crate alloc;

use openzeppelin_stylus::{{
    token::erc1155::{{
        Erc1155, IErc1155, IErc1155MetadataUri,
        extensions::{{supply::{{Erc1155Supply, IErc1155Supply}}}},
    }},
}};
use stylus_sdk::{{
    msg,
    prelude::*,
}};
use alloc::string::String;
use alloc::vec::Vec;

/// {} Multi-Token Contract
#[entrypoint]
#[storage]
struct {} {{
    #[borrow]
    erc1155: Erc1155,
    #[borrow]
    supply: Erc1155Supply,
}}

#[public]
#[inherit(Erc1155, Erc1155Supply)]
impl {} {{
    /// Initialize the multi-token contract
    pub fn init(&mut self, uri: String) -> Result<(), Vec<u8>> {{
        self.erc1155._uri.set(uri);
        Ok(())
    }}
    
    /// Mint tokens
    pub fn mint(
        &mut self,
        to: Address,
        id: U256,
        amount: U256,
        data: Vec<u8>,
    ) -> Result<(), Vec<u8>> {{
        self.erc1155._mint(to, id, amount, &data)?;
        Ok(())
    }}
    
    /// Mint batch of tokens
    pub fn mint_batch(
        &mut self,
        to: Address,
        ids: Vec<U256>,
        amounts: Vec<U256>,
        data: Vec<u8>,
    ) -> Result<(), Vec<u8>> {{
        self.erc1155._mint_batch(to, ids, amounts, &data)?;
        Ok(())
    }}
}}
"#,
            self.contract_name,
            self.contract_name,
            self.contract_name
        )
    }
    
    fn generate_erc20_upgradeable_contract(&self) -> String {
        // Upgradeable contracts are not yet supported in OpenZeppelin Stylus
        // This should not be called due to validation in determine_contract_type
        panic!("Upgradeable contracts are not yet supported for Rust/Stylus")
    }
    
    fn generate_erc721_upgradeable_contract(&self) -> String {
        // Upgradeable contracts are not yet supported in OpenZeppelin Stylus
        panic!("Upgradeable contracts are not yet supported for Rust/Stylus")
    }
    
    fn generate_erc1155_upgradeable_contract(&self) -> String {
        // Upgradeable contracts are not yet supported in OpenZeppelin Stylus
        panic!("Upgradeable contracts are not yet supported for Rust/Stylus")
    }
    
    fn generate_multi_inheritance_contract(&self, base_type: &ContractType, extensions: &[TokenExtension]) -> String {
        // TODO: Implement multi-inheritance for Stylus
        // This will be more complex as Rust doesn't have traditional inheritance
        match base_type {
            ContractType::ERC20 | ContractType::ERC20Upgradeable => self.generate_erc20_with_extensions(extensions),
            ContractType::ERC721 | ContractType::ERC721Upgradeable => self.generate_erc721_with_extensions(extensions),
            ContractType::ERC1155 | ContractType::ERC1155Upgradeable => self.generate_erc1155_with_extensions(extensions),
            _ => self.generate_basic_contract(),
        }
    }
    
    fn generate_erc20_with_extensions(&self, _extensions: &[TokenExtension]) -> String {
        // Generate ERC20 with selected extensions
        // For now, return basic ERC20
        self.generate_erc20_contract()
    }
    
    fn generate_erc721_with_extensions(&self, _extensions: &[TokenExtension]) -> String {
        // Generate ERC721 with selected extensions  
        // For now, return basic ERC721
        self.generate_erc721_contract()
    }
    
    fn generate_erc1155_with_extensions(&self, _extensions: &[TokenExtension]) -> String {
        // Generate ERC1155 with selected extensions
        // For now, return basic ERC1155
        self.generate_erc1155_contract()
    }
    
    fn get_symbol(&self) -> String {
        self.contract_name
            .chars()
            .filter(|c| c.is_uppercase())
            .take(3)
            .collect::<String>()
            .to_uppercase()
    }
}

impl Template for StylusTemplate {
    fn generate_contract(&self) -> String {
        match &self.contract_type {
            ContractType::Basic => self.generate_basic_contract(),
            ContractType::ERC20 => self.generate_erc20_contract(),
            ContractType::ERC721 => self.generate_erc721_contract(),
            ContractType::ERC1155 => self.generate_erc1155_contract(),
            ContractType::ERC20Upgradeable => self.generate_erc20_upgradeable_contract(),
            ContractType::ERC721Upgradeable => self.generate_erc721_upgradeable_contract(),
            ContractType::ERC1155Upgradeable => self.generate_erc1155_upgradeable_contract(),
            ContractType::Interface | ContractType::Abstract => {
                panic!("Interface and Abstract contracts are not supported for Rust/Stylus")
            },
            ContractType::MultiInheritance { base_type, extensions } => {
                self.generate_multi_inheritance_contract(base_type, extensions)
            }
        }
    }
    
    fn generate_test(&self) -> String {
        format!(
r#"#[cfg(test)]
mod tests {{
    use super::*;
    use stylus_sdk::{{Address, U256}};
    
    #[test]
    fn test_deployment() {{
        // Test contract deployment
        let contract = {}::default();
        // Add test assertions here
    }}
    
    #[test]
    fn test_basic_functionality() {{
        // Add functionality tests here
    }}
}}
"#,
            self.contract_name
        )
    }
    
    fn generate_script(&self) -> String {
        // Stylus doesn't have scripts like Foundry
        // Return deployment instructions instead
        format!(
r#"// {} Deployment Instructions
//
// 1. Build the contract:
//    cargo build --release --target wasm32-unknown-unknown \
//      -Z build-std=std,panic_abort \
//      -Z build-std-features=panic_immediate_abort
//
// 2. Deploy using Stylus CLI:
//    stylus deploy --private-key $PRIVATE_KEY
//
// 3. Verify the contract:
//    stylus verify --address <CONTRACT_ADDRESS>
"#,
            self.contract_name
        )
    }
    
    fn generate_library(&self) -> String {
        let snake_name = self.contract_name.chars()
            .enumerate()
            .map(|(i, c)| {
                if c.is_uppercase() && i > 0 {
                    format!("_{}", c.to_lowercase())
                } else {
                    c.to_lowercase().to_string()
                }
            })
            .collect::<String>();
            
        format!(
r#"//! {} Library
//! 
//! A reusable library for Stylus contracts providing utility functions.

#![cfg_attr(not(feature = "export-abi"), no_std)]

extern crate alloc;

use stylus_sdk::prelude::*;
use alloc::{{string::String, vec::Vec}};

/// Example trait for common functionality
pub trait {}Operations {{
    /// Example function - replace with your own
    fn example_operation(&self, value: U256) -> U256;
    
    /// Validation function
    fn is_valid(&self) -> bool;
}}

/// Example data structure for library use
#[derive(Default)]
pub struct {}Data {{
    pub id: U256,
    pub owner: Address,
    pub is_active: bool,
}}

impl {}Data {{
    /// Create new data instance
    pub fn new(id: U256, owner: Address) -> Self {{
        Self {{
            id,
            owner,
            is_active: true,
        }}
    }}
    
    /// Validate data integrity
    pub fn validate(&self) -> bool {{
        !self.owner.is_zero() && self.is_active
    }}
    
    /// Example transformation
    pub fn transform(&mut self, multiplier: U256) {{
        self.id = self.id * multiplier;
    }}
}}

/// Utility functions module
pub mod {} {{
    use super::*;
    
    /// Example pure function
    pub fn double_value(value: U256) -> U256 {{
        value * U256::from(2)
    }}
    
    /// Example validation function
    pub fn is_non_zero_address(addr: Address) -> bool {{
        !addr.is_zero()
    }}
    
    /// Example calculation function
    pub fn calculate_percentage(value: U256, percentage: u8) -> U256 {{
        if percentage > 100 {{
            return U256::ZERO;
        }}
        value * U256::from(percentage) / U256::from(100)
    }}
}}

/// Error types for the library
#[derive(Debug)]
pub enum {}Error {{
    InvalidInput,
    ZeroAddress,
    InactiveData,
}}

impl {}Error {{
    pub fn as_bytes(&self) -> Vec<u8> {{
        match self {{
            Self::InvalidInput => b"Invalid input".to_vec(),
            Self::ZeroAddress => b"Zero address".to_vec(),
            Self::InactiveData => b"Inactive data".to_vec(),
        }}
    }}
}}
"#,
            self.contract_name,
            self.contract_name,
            self.contract_name,
            self.contract_name,
            snake_name,
            self.contract_name,
            self.contract_name
        )
    }
    
    fn generate_interface(&self) -> String {
        // Interfaces are not typically used in Rust/Stylus - use traits instead
        format!(
r#"// Interfaces are not directly supported in Rust/Stylus
// Use traits instead for similar functionality
// 
// Example trait definition:
//
// pub trait {} {{
//     fn example_function(&self) -> bool;
// }}
"#,
            self.contract_name
        )
    }
    
    fn generate_abstract_contract(&self) -> String {
        // Abstract contracts don't exist in Rust - use traits instead
        format!(
r#"// Abstract contracts are not supported in Rust/Stylus
// Use traits for similar patterns
//
// Example trait with default implementations:
//
// pub trait {} {{
//     fn abstract_function(&self) -> U256;
//     
//     fn concrete_function(&self) -> String {{
//         "This is a concrete function".to_string()
//     }}
// }}
"#,
            self.contract_name
        )
    }
}