use anyhow::Result;
use colored::*;
use inquire::{Select, Text, MultiSelect, Confirm, validator::Validation};
use nothung::{Language, ContractType, TokenExtension};

pub struct WizardState {
    pub resource_type: String,
    pub name: String,
    pub language: Language,
    pub contract_type: Option<ContractType>,
    pub extensions: Vec<TokenExtension>,
    pub with_test: bool,
    pub with_script: bool,
    pub pragma: String,
    pub license: String,
}

impl WizardState {
    pub fn new() -> Self {
        Self {
            resource_type: String::new(),
            name: String::new(),
            language: Language::Solidity,
            contract_type: None,
            extensions: Vec::new(),
            with_test: false,
            with_script: false,
            pragma: "0.8.30".to_string(),
            license: "UNLICENSED".to_string(),
        }
    }
}

pub struct ContractWizard;

impl ContractWizard {
    pub fn new() -> Self {
        Self
    }

    pub fn run(&self) -> Result<WizardState> {
        self.print_welcome();
        
        let mut state = WizardState::new();

        // Step 1: Choose resource type
        state.resource_type = self.choose_resource_type()?;

        // Step 2: Enter name
        state.name = self.get_name(&state.resource_type)?;

        // Step 3: Choose language
        state.language = self.choose_language()?;

        // Step 4: Contract-specific configuration
        if state.resource_type == "contract" {
            self.configure_contract(&mut state)?;
        } else if state.resource_type == "library" {
            self.configure_library(&mut state)?;
        } else {
            // Scripts and tests are Solidity-only
            self.configure_script_or_test(&mut state)?;
        }

        // Step 5: Generation options (language-dependent)
        self.configure_generation_options(&mut state)?;

        // Step 6: Solidity-specific options
        if state.language == Language::Solidity {
            self.configure_solidity_options(&mut state)?;
        }

        // Step 7: Confirm and summarize
        self.confirm_generation(&state)?;

        Ok(state)
    }

    fn print_welcome(&self) {
        println!("\n{}", "🧙‍♂️ Welcome to Wotan - The Nothung Smart Contract Wizard!".bold().cyan());
        println!("{}", "⚔️  Let's forge your smart contract step by step...\n");
    }

    fn choose_resource_type(&self) -> Result<String> {
        let options = vec![
            "contract - Smart contract with optional token standards",
            "library - Reusable utility functions", 
            "script - Deployment script (Solidity only)",
            "test - Test file (Solidity only)",
        ];

        let answer = Select::new("What would you like to create?", options).prompt()?;
        Ok(answer.split(" - ").next().unwrap().to_string())
    }

    fn get_name(&self, resource_type: &str) -> Result<String> {
        let prompt = format!("Enter the {} name:", resource_type);
        let validator = |input: &str| -> Result<Validation, Box<dyn std::error::Error + Send + Sync>> {
            if input.is_empty() {
                return Ok(Validation::Invalid("Name cannot be empty".into()));
            }
            if !input.chars().next().unwrap().is_alphabetic() {
                return Ok(Validation::Invalid("Name must start with a letter".into()));
            }
            if !input.chars().all(|c| c.is_alphanumeric() || c == '_') {
                return Ok(Validation::Invalid("Name can only contain letters, numbers, and underscores".into()));
            }
            Ok(Validation::Valid)
        };

        Text::new(&prompt)
            .with_validator(validator)
            .with_placeholder("e.g., MyToken, MathUtils, DeployScript")
            .prompt()
            .map_err(Into::into)
    }

    fn choose_language(&self) -> Result<Language> {
        let options = vec![
            "Solidity - Full-featured (Foundry projects)",
            "Rust/Stylus - Experimental (Arbitrum Stylus)",
        ];

        let answer = Select::new("Choose your language:", options).prompt()?;
        
        if answer.starts_with("Solidity") {
            Ok(Language::Solidity)
        } else {
            println!("\n{}", "ℹ️  Note: Rust/Stylus support is experimental with limited features".yellow());
            Ok(Language::RustStylus)
        }
    }

    fn configure_contract(&self, state: &mut WizardState) -> Result<()> {
        // Choose token standard
        let token_options = vec![
            "Basic contract - No token functionality",
            "ERC20 - Fungible token",
            "ERC721 - Non-fungible token (NFT)",
            "ERC1155 - Multi-token standard",
        ];

        let token_choice = Select::new("Choose token standard:", token_options).prompt()?;
        
        let base_type = match token_choice.split(" - ").next().unwrap() {
            "Basic contract" => ContractType::Basic,
            "ERC20" => ContractType::ERC20,
            "ERC721" => ContractType::ERC721,
            "ERC1155" => ContractType::ERC1155,
            _ => unreachable!(),
        };

        // Check for upgradeable (Solidity only)
        let mut is_upgradeable = false;
        if state.language == Language::Solidity && base_type != ContractType::Basic {
            is_upgradeable = Confirm::new("Make this contract upgradeable?")
                .with_default(false)
                .prompt()?;
        } else if state.language == Language::RustStylus && base_type != ContractType::Basic {
            println!("{}", "ℹ️  Upgradeable contracts are not yet supported for Rust/Stylus".yellow());
        }

        // Set contract type
        state.contract_type = Some(match (base_type.clone(), is_upgradeable) {
            (ContractType::ERC20, true) => ContractType::ERC20Upgradeable,
            (ContractType::ERC721, true) => ContractType::ERC721Upgradeable,
            (ContractType::ERC1155, true) => ContractType::ERC1155Upgradeable,
            (base, false) => base,
            _ => unreachable!(),
        });

        // Choose extensions (Solidity only for now)
        if state.language == Language::Solidity && base_type != ContractType::Basic {
            self.choose_extensions(state, &base_type)?;
        } else if state.language == Language::RustStylus && base_type != ContractType::Basic {
            println!("{}", "ℹ️  Token extensions are not yet supported for Rust/Stylus".yellow());
        }

        Ok(())
    }

    fn choose_extensions(&self, state: &mut WizardState, base_type: &ContractType) -> Result<()> {
        let available_extensions = match base_type {
            ContractType::ERC20 => vec![
                "burnable - Token burning capability",
                "pausable - Emergency pause functionality", 
                "permit - Gasless approvals (EIP-2612)",
                "capped - Maximum supply limit",
                "votes - On-chain voting & delegation",
                "wrapper - Wrap other ERC20 tokens",
                "flashmint - Flash loan support",
            ],
            ContractType::ERC721 => vec![
                "burnable - NFT burning capability",
                "pausable - Emergency pause functionality",
                "enumerable - Token enumeration",
                "uristorage - Dynamic metadata URIs", 
                "royalty - ERC2981 royalty standard",
                "votes - NFT-based voting",
            ],
            ContractType::ERC1155 => vec![
                "burnable - Multi-token burning",
                "pausable - Emergency pause functionality",
                "supply - Track token supplies",
                "uristorage - Per-token URI storage",
            ],
            _ => vec![],
        };

        if available_extensions.is_empty() {
            return Ok(());
        }

        let add_extensions = Confirm::new("Add token extensions?")
            .with_default(false)
            .prompt()?;

        if add_extensions {
            let selected = MultiSelect::new("Select extensions:", available_extensions)
                .prompt()?;

            for extension in selected {
                let ext_name = extension.split(" - ").next().unwrap();
                if let Ok(parsed_ext) = self.parse_extension(ext_name, base_type) {
                    state.extensions.push(parsed_ext);
                }
            }
        }

        Ok(())
    }

    fn parse_extension(&self, name: &str, base_type: &ContractType) -> Result<TokenExtension> {
        use TokenExtension::*;
        
        let extension = match (name, base_type) {
            ("burnable", ContractType::ERC20) => ERC20Burnable,
            ("pausable", ContractType::ERC20) => ERC20Pausable,
            ("permit", ContractType::ERC20) => ERC20Permit,
            ("capped", ContractType::ERC20) => ERC20Capped,
            ("votes", ContractType::ERC20) => ERC20Votes,
            ("wrapper", ContractType::ERC20) => ERC20Wrapper,
            ("flashmint", ContractType::ERC20) => ERC20FlashMint,
            
            ("burnable", ContractType::ERC721) => ERC721Burnable,
            ("pausable", ContractType::ERC721) => ERC721Pausable,
            ("enumerable", ContractType::ERC721) => ERC721Enumerable,
            ("uristorage", ContractType::ERC721) => ERC721URIStorage,
            ("royalty", ContractType::ERC721) => ERC721Royalty,
            ("votes", ContractType::ERC721) => ERC721Votes,
            
            ("burnable", ContractType::ERC1155) => ERC1155Burnable,
            ("pausable", ContractType::ERC1155) => ERC1155Pausable,
            ("supply", ContractType::ERC1155) => ERC1155Supply,
            ("uristorage", ContractType::ERC1155) => ERC1155URIStorage,
            
            _ => return Err(anyhow::anyhow!("Unknown extension: {}", name)),
        };
        
        Ok(extension)
    }

    fn configure_library(&self, _state: &mut WizardState) -> Result<()> {
        println!("{}", "ℹ️  Libraries will contain basic utility functions and data structures");
        Ok(())
    }

    fn configure_script_or_test(&self, state: &mut WizardState) -> Result<()> {
        if state.language == Language::RustStylus {
            return Err(anyhow::anyhow!("Scripts and tests are only supported for Solidity projects"));
        }
        println!("{}", "ℹ️  Basic script/test file will be generated");
        Ok(())
    }

    fn configure_generation_options(&self, state: &mut WizardState) -> Result<()> {
        if state.language == Language::Solidity && state.resource_type == "contract" {
            state.with_test = Confirm::new("Generate test file?")
                .with_default(true)
                .prompt()?;

            state.with_script = Confirm::new("Generate deployment script?")
                .with_default(false) 
                .prompt()?;
        } else if state.language == Language::RustStylus {
            println!("{}", "ℹ️  Test and script generation not supported for Rust/Stylus (use cargo test and stylus deploy)".yellow());
        }

        Ok(())
    }

    fn configure_solidity_options(&self, state: &mut WizardState) -> Result<()> {
        if state.language != Language::Solidity {
            return Ok(());
        }

        let change_defaults = Confirm::new("Customize Solidity options (pragma, license)?")
            .with_default(false)
            .prompt()?;

        if change_defaults {
            state.pragma = Text::new("Solidity pragma version:")
                .with_default(&state.pragma)
                .with_placeholder("e.g., 0.8.30")
                .prompt()?;

            let license_options = vec![
                "UNLICENSED",
                "MIT", 
                "Apache-2.0",
                "GPL-3.0",
                "BSD-3-Clause",
                "Custom...",
            ];

            let license_choice = Select::new("SPDX License Identifier:", license_options).prompt()?;
            
            if license_choice == "Custom..." {
                state.license = Text::new("Enter custom license:")
                    .with_placeholder("e.g., GPL-2.0")
                    .prompt()?;
            } else {
                state.license = license_choice.to_string();
            }
        }

        Ok(())
    }

    fn confirm_generation(&self, state: &WizardState) -> Result<()> {
        println!("\n{}", "📋 Generation Summary:".bold());
        println!("  {} {}", "Type:".bold(), state.resource_type);
        println!("  {} {}", "Name:".bold(), state.name);
        println!("  {} {:?}", "Language:".bold(), state.language);
        
        if let Some(ref contract_type) = state.contract_type {
            println!("  {} {:?}", "Contract Type:".bold(), contract_type);
        }
        
        if !state.extensions.is_empty() {
            println!("  {} {:?}", "Extensions:".bold(), state.extensions);
        }
        
        if state.language == Language::Solidity {
            println!("  {} {}", "Pragma:".bold(), state.pragma);
            println!("  {} {}", "License:".bold(), state.license);
            
            if state.with_test {
                println!("  {} Yes", "Generate Test:".bold());
            }
            if state.with_script {
                println!("  {} Yes", "Generate Script:".bold());
            }
        }

        let confirmed = Confirm::new("\nGenerate files?")
            .with_default(true)
            .prompt()?;

        if !confirmed {
            return Err(anyhow::anyhow!("Generation cancelled"));
        }

        Ok(())
    }
}