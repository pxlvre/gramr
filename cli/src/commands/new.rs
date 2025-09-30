use gramr::{
    AbstractContractGenerator, ConfigGenerator, ContractType, GenericContractGenerator, GramrError,
    InterfaceGenerator, Language, LibraryGenerator, ProjectType, Result, ScriptGenerator,
    TestGenerator, TokenExtension,
};

pub fn execute_new(
    resource_type: &str,
    name: String,
    solidity: bool,
    rust_stylus: bool,
    oz_erc20: bool,
    oz_erc721: bool,
    oz_erc1155: bool,
    upgradeable: bool,
    extensions: Vec<String>,
    with_test: bool,
    with_script: bool,
    pragma: String,
    license: String,
    with_section_markers: bool,
) -> Result<()> {
    let language = Language::from_flags(solidity, rust_stylus)?;
    let project = ProjectType::detect(&language)?;

    match resource_type {
        "contract" => {
            let contract_type = determine_contract_type(oz_erc20, oz_erc721, oz_erc1155, upgradeable, &extensions, &language)?;
            let generator = GenericContractGenerator::new(
                project,
                language,
                name,
                contract_type,
                with_test,
                with_script,
                Some(pragma),
                Some(license),
            );
            generator.generate()
        }
        "library" => {
            if oz_erc20 || oz_erc721 || oz_erc1155 || upgradeable || !extensions.is_empty() || with_test || with_script || with_section_markers {
                return Err(GramrError::Other(
                    "Library generation doesn't support contract-specific flags".to_string()
                ));
            }
            let generator = LibraryGenerator::new(
                project,
                language,
                name,
                Some(pragma),
                Some(license),
            );
            generator.generate()
        }
        "script" => {
            if rust_stylus {
                return Err(GramrError::Other(
                    "Script generation is not supported for Rust/Stylus projects".to_string()
                ));
            }
            if oz_erc20 || oz_erc721 || oz_erc1155 || upgradeable || !extensions.is_empty() || with_test || with_script || with_section_markers {
                return Err(GramrError::Other(
                    "Script generation doesn't support contract-specific flags".to_string()
                ));
            }
            match project {
                ProjectType::Foundry(foundry_project) => {
                    let generator = ScriptGenerator::new(foundry_project, name, pragma, license);
                    generator.generate()
                }
                _ => Err(GramrError::Other("Script generation is only supported for Foundry projects".to_string()))
            }
        }
        "test" => {
            if rust_stylus {
                return Err(GramrError::Other(
                    "Test generation is not supported for Rust/Stylus projects".to_string()
                ));
            }
            if oz_erc20 || oz_erc721 || oz_erc1155 || upgradeable || !extensions.is_empty() || with_test || with_script || with_section_markers {
                return Err(GramrError::Other(
                    "Test generation doesn't support contract-specific flags".to_string()
                ));
            }
            match project {
                ProjectType::Foundry(foundry_project) => {
                    let generator = TestGenerator::new(foundry_project, name, pragma, license);
                    generator.generate()
                }
                _ => Err(GramrError::Other("Test generation is only supported for Foundry projects".to_string()))
            }
        }
        "interface" => {
            if oz_erc20 || oz_erc721 || oz_erc1155 || upgradeable || !extensions.is_empty() || with_test || with_script || with_section_markers {
                return Err(GramrError::Other(
                    "Interface generation doesn't support contract-specific flags".to_string()
                ));
            }
            let generator = InterfaceGenerator::new(
                project,
                language,
                name,
                Some(pragma),
                Some(license),
            );
            generator.generate()
        }
        "abstract" => {
            if oz_erc20 || oz_erc721 || oz_erc1155 || upgradeable || !extensions.is_empty() || with_test || with_script {
                return Err(GramrError::Other(
                    "Abstract contract generation doesn't support contract-specific flags (except --with-section-markers)".to_string()
                ));
            }
            let generator = AbstractContractGenerator::new(
                project,
                language,
                name,
                Some(pragma),
                Some(license),
            );
            generator.generate()
        }
        "config" => {
            if oz_erc20 || oz_erc721 || oz_erc1155 || upgradeable || !extensions.is_empty() || with_test || with_script || with_section_markers {
                return Err(GramrError::Other(
                    "Config generation doesn't support contract-specific flags".to_string()
                ));
            }
            let generator = ConfigGenerator::new(
                project,
                language,
                name,
            );
            generator.generate()
        }
        _ => Err(GramrError::Other(
            format!("Unsupported resource type: {}. Supported types: contract, library, script, test, interface, abstract, config", resource_type)
        )),
    }
}

fn determine_contract_type(
    oz_erc20: bool,
    oz_erc721: bool,
    oz_erc1155: bool,
    upgradeable: bool,
    extensions: &[String],
    language: &Language,
) -> Result<ContractType> {
    // Count how many base token types are specified
    let base_count = [oz_erc20, oz_erc721, oz_erc1155]
        .iter()
        .filter(|&&x| x)
        .count();

    if base_count > 1 {
        return Err(GramrError::Other(
            "Cannot use multiple base token types together (--oz-erc20, --oz-erc721, --oz-erc1155)"
                .to_string(),
        ));
    }

    if upgradeable && base_count == 0 {
        return Err(GramrError::Other(
            "Must specify a base token type (--oz-erc20, --oz-erc721, or --oz-erc1155) when using --upgradeable".to_string()
        ));
    }

    // Check if upgradeable is supported for the language
    if upgradeable && *language == Language::RustStylus {
        return Err(GramrError::Other(
            "Upgradeable contracts are not yet supported for Rust/Stylus. OpenZeppelin Stylus library doesn't include upgradeable patterns yet.".to_string()
        ));
    }

    // Parse extensions using the library function
    let parsed_extensions = if !extensions.is_empty() {
        gramr::parse_extensions(extensions)?
    } else {
        Vec::new()
    };

    // If extensions are specified, we need a base type
    if !parsed_extensions.is_empty() && base_count == 0 {
        return Err(GramrError::Other(
            "Must specify a base token type (--oz-erc20, --oz-erc721, or --oz-erc1155) when using --extensions".to_string()
        ));
    }

    // Determine base contract type
    let base_type = match (oz_erc20, oz_erc721, oz_erc1155, upgradeable) {
        (true, false, false, false) => ContractType::ERC20,
        (true, false, false, true) => ContractType::ERC20Upgradeable,
        (false, true, false, false) => ContractType::ERC721,
        (false, true, false, true) => ContractType::ERC721Upgradeable,
        (false, false, true, false) => ContractType::ERC1155,
        (false, false, true, true) => ContractType::ERC1155Upgradeable,
        (false, false, false, false) => ContractType::Basic,
        _ => unreachable!(), // We've already checked for multiple base types
    };

    // If no extensions, return the base type
    if parsed_extensions.is_empty() {
        Ok(base_type)
    } else {
        // Validate extensions are compatible with base type
        validate_extensions_compatibility(&base_type, &parsed_extensions)?;
        Ok(ContractType::MultiInheritance {
            base_type: Box::new(base_type),
            extensions: parsed_extensions,
        })
    }
}

fn validate_extensions_compatibility(
    base_type: &ContractType,
    extensions: &[TokenExtension],
) -> Result<()> {
    for extension in extensions {
        let is_compatible = match (base_type, extension) {
            // ERC20 extensions
            (ContractType::ERC20 | ContractType::ERC20Upgradeable, TokenExtension::ERC20Permit) => {
                true
            }
            (
                ContractType::ERC20 | ContractType::ERC20Upgradeable,
                TokenExtension::ERC20Burnable,
            ) => true,
            (ContractType::ERC20 | ContractType::ERC20Upgradeable, TokenExtension::ERC20Capped) => {
                true
            }
            (
                ContractType::ERC20 | ContractType::ERC20Upgradeable,
                TokenExtension::ERC20Pausable,
            ) => true,
            (ContractType::ERC20 | ContractType::ERC20Upgradeable, TokenExtension::ERC20Votes) => {
                true
            }
            (
                ContractType::ERC20 | ContractType::ERC20Upgradeable,
                TokenExtension::ERC20Wrapper,
            ) => true,
            (
                ContractType::ERC20 | ContractType::ERC20Upgradeable,
                TokenExtension::ERC20FlashMint,
            ) => true,
            (
                ContractType::ERC20 | ContractType::ERC20Upgradeable,
                TokenExtension::ERC20TemporaryApproval,
            ) => true,
            (
                ContractType::ERC20 | ContractType::ERC20Upgradeable,
                TokenExtension::ERC20Bridgeable,
            ) => true,
            (ContractType::ERC20 | ContractType::ERC20Upgradeable, TokenExtension::ERC1363) => true,
            (ContractType::ERC20 | ContractType::ERC20Upgradeable, TokenExtension::ERC4626) => true,

            // ERC721 extensions
            (
                ContractType::ERC721 | ContractType::ERC721Upgradeable,
                TokenExtension::ERC721Pausable,
            ) => true,
            (
                ContractType::ERC721 | ContractType::ERC721Upgradeable,
                TokenExtension::ERC721Burnable,
            ) => true,
            (
                ContractType::ERC721 | ContractType::ERC721Upgradeable,
                TokenExtension::ERC721Consecutive,
            ) => true,
            (
                ContractType::ERC721 | ContractType::ERC721Upgradeable,
                TokenExtension::ERC721URIStorage,
            ) => true,
            (
                ContractType::ERC721 | ContractType::ERC721Upgradeable,
                TokenExtension::ERC721Votes,
            ) => true,
            (
                ContractType::ERC721 | ContractType::ERC721Upgradeable,
                TokenExtension::ERC721Royalty,
            ) => true,
            (
                ContractType::ERC721 | ContractType::ERC721Upgradeable,
                TokenExtension::ERC721Wrapper,
            ) => true,
            (
                ContractType::ERC721 | ContractType::ERC721Upgradeable,
                TokenExtension::ERC721Enumerable,
            ) => true,

            // ERC1155 extensions
            (
                ContractType::ERC1155 | ContractType::ERC1155Upgradeable,
                TokenExtension::ERC1155Pausable,
            ) => true,
            (
                ContractType::ERC1155 | ContractType::ERC1155Upgradeable,
                TokenExtension::ERC1155Burnable,
            ) => true,
            (
                ContractType::ERC1155 | ContractType::ERC1155Upgradeable,
                TokenExtension::ERC1155Supply,
            ) => true,
            (
                ContractType::ERC1155 | ContractType::ERC1155Upgradeable,
                TokenExtension::ERC1155URIStorage,
            ) => true,

            // Cross-compatible extensions (burnable, pausable, wrapper, uristorage work with multiple types)
            (
                ContractType::ERC721 | ContractType::ERC721Upgradeable,
                TokenExtension::ERC20Burnable,
            ) => true, // Will be converted to ERC721Burnable
            (
                ContractType::ERC721 | ContractType::ERC721Upgradeable,
                TokenExtension::ERC20Pausable,
            ) => true, // Will be converted to ERC721Pausable
            (
                ContractType::ERC721 | ContractType::ERC721Upgradeable,
                TokenExtension::ERC20Wrapper,
            ) => true, // Will be converted to ERC721Wrapper
            (
                ContractType::ERC1155 | ContractType::ERC1155Upgradeable,
                TokenExtension::ERC20Burnable,
            ) => true, // Will be converted to ERC1155Burnable
            (
                ContractType::ERC1155 | ContractType::ERC1155Upgradeable,
                TokenExtension::ERC20Pausable,
            ) => true, // Will be converted to ERC1155Pausable
            (
                ContractType::ERC1155 | ContractType::ERC1155Upgradeable,
                TokenExtension::ERC721URIStorage,
            ) => true, // Will be converted to ERC1155URIStorage

            _ => false,
        };

        if !is_compatible {
            return Err(GramrError::Other(format!(
                "Extension {:?} is not compatible with base type {:?}",
                extension, base_type
            )));
        }
    }

    Ok(())
}
