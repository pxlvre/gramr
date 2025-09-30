use crate::error::{GramrError, Result};

#[derive(Clone, Debug, PartialEq)]
pub enum Language {
    Solidity,
    RustStylus,
}

impl Language {
    pub fn from_flags(solidity: bool, rust_stylus: bool) -> Result<Self> {
        match (solidity, rust_stylus) {
            (true, false) => Ok(Language::Solidity),
            (false, true) => Ok(Language::RustStylus),
            (false, false) => Err(GramrError::Other(
                "Must specify either --solidity or --rust-stylus".to_string(),
            )),
            (true, true) => Err(GramrError::Other(
                "Cannot specify both --solidity and --rust-stylus".to_string(),
            )),
        }
    }
}
