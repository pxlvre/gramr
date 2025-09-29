pub mod solidity;
pub mod stylus;

pub use solidity::SolidityTemplate;
pub use stylus::StylusTemplate;

// Move these to a common location since they're shared
#[derive(Clone, Debug)]
pub enum ContractType {
    Basic,
    ERC20,
    ERC721,
    ERC1155,
    ERC20Upgradeable,
    ERC721Upgradeable,
    ERC1155Upgradeable,
    Interface,
    Abstract,
    MultiInheritance {
        base_type: Box<ContractType>,
        extensions: Vec<TokenExtension>,
    },
}

#[derive(Clone, Debug)]
pub enum TokenExtension {
    // ERC20 Extensions
    ERC20Permit,
    ERC20Burnable,
    ERC20Capped,
    ERC20Pausable,
    ERC20Votes,
    ERC20Wrapper,
    ERC20FlashMint,
    ERC20TemporaryApproval,
    ERC20Bridgeable,
    ERC1363,
    ERC4626,
    
    // ERC721 Extensions
    ERC721Pausable,
    ERC721Burnable,
    ERC721Consecutive,
    ERC721URIStorage,
    ERC721Votes,
    ERC721Royalty,
    ERC721Wrapper,
    ERC721Enumerable,
    
    // ERC1155 Extensions
    ERC1155Pausable,
    ERC1155Burnable,
    ERC1155Supply,
    ERC1155URIStorage,
}

pub trait Template {
    fn generate_contract(&self) -> String;
    fn generate_test(&self) -> String;
    fn generate_script(&self) -> String;
    fn generate_library(&self) -> String;
    fn generate_interface(&self) -> String;
    fn generate_abstract_contract(&self) -> String;
}