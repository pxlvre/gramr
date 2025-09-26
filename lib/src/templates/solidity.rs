
#[derive(Clone, Debug)]
pub enum ContractType {
    Basic,
    ERC20,
    ERC721,
    ERC1155,
    ERC20Upgradeable,
    ERC721Upgradeable,
    ERC1155Upgradeable,
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

pub struct SolidityTemplate {
    contract_name: String,
    contract_type: ContractType,
    pragma: String,
    license: String,
}

impl SolidityTemplate {
    pub fn new(contract_name: String, contract_type: ContractType, pragma: String, license: String) -> Self {
        Self {
            contract_name,
            contract_type,
            pragma,
            license,
        }
    }

    pub fn generate_contract(&self) -> String {
        match &self.contract_type {
            ContractType::Basic => self.generate_basic_contract(),
            ContractType::ERC20 => self.generate_erc20_contract(),
            ContractType::ERC721 => self.generate_erc721_contract(),
            ContractType::ERC1155 => self.generate_erc1155_contract(),
            ContractType::ERC20Upgradeable => self.generate_erc20_upgradeable_contract(),
            ContractType::ERC721Upgradeable => self.generate_erc721_upgradeable_contract(),
            ContractType::ERC1155Upgradeable => self.generate_erc1155_upgradeable_contract(),
            ContractType::MultiInheritance { base_type, extensions } => {
                self.generate_multi_inheritance_contract(base_type, extensions)
            }
        }
    }

    pub fn generate_test(&self) -> String {
        let constructor_args = match &self.contract_type {
            ContractType::ERC20 | ContractType::ERC20Upgradeable => "(1000000 * 10 ** 18)",
            ContractType::MultiInheritance { base_type, .. } => {
                match **base_type {
                    ContractType::ERC20 | ContractType::ERC20Upgradeable => "(1000000 * 10 ** 18)",
                    _ => "()",
                }
            }
            _ => "()",
        };

        format!(
r#"// SPDX-License-Identifier: {}
pragma solidity ^{};

import "forge-std/Test.sol";
import "../src/{}.sol";

contract {}Test is Test {{
    {} public instance;

    function setUp() public {{
        instance = new {}{};
    }}

    function test_Deployment() public view {{
        assertNotEq(address(instance), address(0));
    }}
}}"#,
            self.license,
            self.pragma,
            self.contract_name,
            self.contract_name,
            self.contract_name,
            self.contract_name,
            constructor_args
        )
    }

    pub fn generate_script(&self) -> String {
        let constructor_args = match &self.contract_type {
            ContractType::ERC20 | ContractType::ERC20Upgradeable => "(1000000 * 10 ** 18)",
            ContractType::MultiInheritance { base_type, .. } => {
                match **base_type {
                    ContractType::ERC20 | ContractType::ERC20Upgradeable => "(1000000 * 10 ** 18)",
                    _ => "()",
                }
            }
            _ => "()",
        };

        format!(
r#"// SPDX-License-Identifier: {}
pragma solidity ^{};

import "forge-std/Script.sol";
import "../src/{}.sol";

contract Deploy{} is Script {{
    function run() external returns ({}) {{
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        
        vm.startBroadcast(deployerPrivateKey);
        
        {} instance = new {}{};
        
        vm.stopBroadcast();
        
        return instance;
    }}
}}"#,
            self.license,
            self.pragma,
            self.contract_name,
            self.contract_name,
            self.contract_name,
            self.contract_name,
            self.contract_name,
            constructor_args
        )
    }

    fn generate_basic_contract(&self) -> String {
        format!(
r#"// SPDX-License-Identifier: {}
pragma solidity ^{};

contract {} {{
    constructor() {{}}
}}"#,
            self.license,
            self.pragma,
            self.contract_name
        )
    }

    fn generate_erc20_contract(&self) -> String {
        format!(
r#"// SPDX-License-Identifier: {}
pragma solidity ^{};

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract {} is ERC20 {{
    constructor(uint256 initialSupply) ERC20("{}", "{}") {{
        _mint(msg.sender, initialSupply);
    }}
}}"#,
            self.license,
            self.pragma,
            self.contract_name,
            self.contract_name,
            self.get_symbol()
        )
    }

    fn generate_erc721_contract(&self) -> String {
        format!(
r#"// SPDX-License-Identifier: {}
pragma solidity ^{};

import "@openzeppelin/contracts/token/ERC721/ERC721.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

contract {} is ERC721, Ownable {{
    uint256 private _tokenIdCounter;

    constructor() ERC721("{}", "{}") Ownable(msg.sender) {{}}

    function mint(address to) public onlyOwner {{
        uint256 tokenId = _tokenIdCounter++;
        _safeMint(to, tokenId);
    }}
}}"#,
            self.license,
            self.pragma,
            self.contract_name,
            self.contract_name,
            self.get_symbol()
        )
    }

    fn generate_erc20_upgradeable_contract(&self) -> String {
        format!(
r#"// SPDX-License-Identifier: {}
pragma solidity ^{};

import "@openzeppelin/contracts-upgradeable/token/ERC20/ERC20Upgradeable.sol";
import "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol";

contract {} is Initializable, ERC20Upgradeable, OwnableUpgradeable, UUPSUpgradeable {{
    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {{
        _disableInitializers();
    }}

    function initialize(uint256 initialSupply) public initializer {{
        __ERC20_init("{}", "{}");
        __Ownable_init(msg.sender);
        __UUPSUpgradeable_init();
        
        _mint(msg.sender, initialSupply);
    }}

    function _authorizeUpgrade(address newImplementation) internal override onlyOwner {{}}
}}"#,
            self.license,
            self.pragma,
            self.contract_name,
            self.contract_name,
            self.get_symbol()
        )
    }

    fn generate_erc721_upgradeable_contract(&self) -> String {
        format!(
r#"// SPDX-License-Identifier: {}
pragma solidity ^{};

import "@openzeppelin/contracts-upgradeable/token/ERC721/ERC721Upgradeable.sol";
import "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol";

contract {} is Initializable, ERC721Upgradeable, OwnableUpgradeable, UUPSUpgradeable {{
    uint256 private _tokenIdCounter;

    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {{
        _disableInitializers();
    }}

    function initialize() public initializer {{
        __ERC721_init("{}", "{}");
        __Ownable_init(msg.sender);
        __UUPSUpgradeable_init();
    }}

    function mint(address to) public onlyOwner {{
        uint256 tokenId = _tokenIdCounter++;
        _safeMint(to, tokenId);
    }}

    function _authorizeUpgrade(address newImplementation) internal override onlyOwner {{}}
}}"#,
            self.license,
            self.pragma,
            self.contract_name,
            self.contract_name,
            self.get_symbol()
        )
    }

    fn generate_erc1155_contract(&self) -> String {
        format!(
r#"// SPDX-License-Identifier: {}
pragma solidity ^{};

import "@openzeppelin/contracts/token/ERC1155/ERC1155.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

contract {} is ERC1155, Ownable {{
    constructor() ERC1155("https://api.example.com/tokens/{{id}}.json") Ownable(msg.sender) {{}}

    function mint(address to, uint256 id, uint256 amount, bytes memory data) public onlyOwner {{
        _mint(to, id, amount, data);
    }}

    function mintBatch(address to, uint256[] memory ids, uint256[] memory amounts, bytes memory data) public onlyOwner {{
        _mintBatch(to, ids, amounts, data);
    }}
}}"#,
            self.license,
            self.pragma,
            self.contract_name
        )
    }

    fn generate_erc1155_upgradeable_contract(&self) -> String {
        format!(
r#"// SPDX-License-Identifier: {}
pragma solidity ^{};

import "@openzeppelin/contracts-upgradeable/token/ERC1155/ERC1155Upgradeable.sol";
import "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol";

contract {} is Initializable, ERC1155Upgradeable, OwnableUpgradeable, UUPSUpgradeable {{
    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {{
        _disableInitializers();
    }}

    function initialize() public initializer {{
        __ERC1155_init("https://api.example.com/tokens/{{id}}.json");
        __Ownable_init(msg.sender);
        __UUPSUpgradeable_init();
    }}

    function mint(address to, uint256 id, uint256 amount, bytes memory data) public onlyOwner {{
        _mint(to, id, amount, data);
    }}

    function mintBatch(address to, uint256[] memory ids, uint256[] memory amounts, bytes memory data) public onlyOwner {{
        _mintBatch(to, ids, amounts, data);
    }}

    function _authorizeUpgrade(address newImplementation) internal override onlyOwner {{}}
}}"#,
            self.license,
            self.pragma,
            self.contract_name
        )
    }

    fn generate_multi_inheritance_contract(&self, base_type: &ContractType, extensions: &[TokenExtension]) -> String {
        let (imports, inheritance_chain, _initializers, additional_functions) = self.build_inheritance_components(base_type, extensions);
        
        let constructor_params = match base_type {
            ContractType::ERC20 | ContractType::ERC20Upgradeable => "uint256 initialSupply",
            _ => "",
        };

        let has_wrapper = extensions.iter().any(|ext| matches!(ext, TokenExtension::ERC721Wrapper | TokenExtension::ERC20Wrapper));
        
        let constructor_call = match base_type {
            ContractType::ERC20 => format!("ERC20(\"{}\", \"{}\")", self.contract_name, self.get_symbol()),
            ContractType::ERC721 => {
                let base_call = format!("ERC721(\"{}\", \"{}\")", self.contract_name, self.get_symbol());
                if has_wrapper {
                    format!("{} ERC721Wrapper(IERC721(UNDERLYING_TOKEN))", base_call)
                } else {
                    base_call
                }
            },
            ContractType::ERC1155 => "ERC1155(\"https://api.example.com/tokens/{id}.json\")".to_string(),
            _ => "".to_string(),
        };

        let constructor_body = match base_type {
            ContractType::ERC20 => "        _mint(msg.sender, initialSupply);",
            _ => "",
        };

        format!(
r#"// SPDX-License-Identifier: {}
pragma solidity ^{};

{}

contract {} is {} {{
{}

    constructor({}) {} Ownable(msg.sender) {{
{}
    }}

{}
}}"#,
            self.license,
            self.pragma,
            imports,
            self.contract_name,
            inheritance_chain,
            self.generate_state_variables(extensions),
            constructor_params,
            constructor_call,
            constructor_body,
            additional_functions
        )
    }

    fn build_inheritance_components(&self, base_type: &ContractType, extensions: &[TokenExtension]) -> (String, String, String, String) {
        let mut imports = Vec::new();
        let mut inheritance = Vec::new();
        let mut initializers = Vec::new();
        let mut functions = Vec::new();

        // Base imports and inheritance
        match base_type {
            ContractType::ERC20 => {
                imports.push("import \"@openzeppelin/contracts/token/ERC20/ERC20.sol\";".to_string());
                inheritance.push("ERC20".to_string());
            },
            ContractType::ERC721 => {
                imports.push("import \"@openzeppelin/contracts/token/ERC721/ERC721.sol\";".to_string());
                inheritance.push("ERC721".to_string());
            },
            ContractType::ERC1155 => {
                imports.push("import \"@openzeppelin/contracts/token/ERC1155/ERC1155.sol\";".to_string());
                inheritance.push("ERC1155".to_string());
            },
            _ => {}
        }

        // Always add Ownable as it's needed for most extensions
        imports.push("import \"@openzeppelin/contracts/access/Ownable.sol\";".to_string());
        inheritance.push("Ownable".to_string());

        // Add extension-specific imports and inheritance
        for extension in extensions {
            // Convert cross-compatible extensions based on base type
            let converted_extension = self.convert_extension_for_base_type(base_type, extension);
            let (import, inherit, init, func) = self.get_extension_components(&converted_extension);
            if !import.is_empty() { imports.push(import); }
            if !inherit.is_empty() { inheritance.push(inherit); }
            if !init.is_empty() { initializers.push(init); }
            if !func.is_empty() { functions.push(func); }
        }

        (
            imports.join("\n"),
            inheritance.join(", "),
            initializers.join("\n        "),
            functions.join("\n\n")
        )
    }

    fn convert_extension_for_base_type(&self, base_type: &ContractType, extension: &TokenExtension) -> TokenExtension {
        match (base_type, extension) {
            // Convert ERC20Burnable to appropriate token type
            (ContractType::ERC721 | ContractType::ERC721Upgradeable, TokenExtension::ERC20Burnable) => TokenExtension::ERC721Burnable,
            (ContractType::ERC1155 | ContractType::ERC1155Upgradeable, TokenExtension::ERC20Burnable) => TokenExtension::ERC1155Burnable,
            
            // Convert ERC20Pausable to appropriate token type
            (ContractType::ERC721 | ContractType::ERC721Upgradeable, TokenExtension::ERC20Pausable) => TokenExtension::ERC721Pausable,
            (ContractType::ERC1155 | ContractType::ERC1155Upgradeable, TokenExtension::ERC20Pausable) => TokenExtension::ERC1155Pausable,
            
            // Convert wrapper extensions
            (ContractType::ERC721 | ContractType::ERC721Upgradeable, TokenExtension::ERC20Wrapper) => TokenExtension::ERC721Wrapper,
            
            // Convert uristorage extensions
            (ContractType::ERC1155 | ContractType::ERC1155Upgradeable, TokenExtension::ERC721URIStorage) => TokenExtension::ERC1155URIStorage,
            
            // By default, keep the extension as-is
            _ => extension.clone(),
        }
    }

    fn get_extension_components(&self, extension: &TokenExtension) -> (String, String, String, String) {
        match extension {
            TokenExtension::ERC20Burnable => (
                "import \"@openzeppelin/contracts/token/ERC20/extensions/ERC20Burnable.sol\";".to_string(),
                "ERC20Burnable".to_string(),
                "".to_string(),
                "".to_string()
            ),
            TokenExtension::ERC20Pausable => (
                "import \"@openzeppelin/contracts/token/ERC20/extensions/ERC20Pausable.sol\";".to_string(),
                "ERC20Pausable".to_string(),
                "".to_string(),
                "    function pause() public onlyOwner {\n        _pause();\n    }\n\n    function unpause() public onlyOwner {\n        _unpause();\n    }".to_string()
            ),
            TokenExtension::ERC20Votes => (
                "import \"@openzeppelin/contracts/token/ERC20/extensions/ERC20Votes.sol\";\nimport \"@openzeppelin/contracts/utils/cryptography/EIP712.sol\";".to_string(),
                "ERC20Votes, EIP712".to_string(),
                "".to_string(),
                "".to_string()
            ),
            TokenExtension::ERC20Permit => (
                "import \"@openzeppelin/contracts/token/ERC20/extensions/ERC20Permit.sol\";".to_string(),
                "ERC20Permit".to_string(),
                "".to_string(),
                "".to_string()
            ),
            TokenExtension::ERC721Burnable => (
                "import \"@openzeppelin/contracts/token/ERC721/extensions/ERC721Burnable.sol\";".to_string(),
                "ERC721Burnable".to_string(),
                "".to_string(),
                "".to_string()
            ),
            TokenExtension::ERC721Pausable => (
                "import \"@openzeppelin/contracts/token/ERC721/extensions/ERC721Pausable.sol\";".to_string(),
                "ERC721Pausable".to_string(),
                "".to_string(),
                "    function pause() public onlyOwner {\n        _pause();\n    }\n\n    function unpause() public onlyOwner {\n        _unpause();\n    }".to_string()
            ),
            TokenExtension::ERC721Enumerable => (
                "import \"@openzeppelin/contracts/token/ERC721/extensions/ERC721Enumerable.sol\";".to_string(),
                "ERC721Enumerable".to_string(),
                "".to_string(),
                "    function supportsInterface(bytes4 interfaceId) public view virtual override(ERC721, ERC721Enumerable) returns (bool) {\n        return super.supportsInterface(interfaceId);\n    }\n\n    function _update(address to, uint256 tokenId, address auth) internal virtual override(ERC721, ERC721Enumerable) returns (address) {\n        return super._update(to, tokenId, auth);\n    }\n\n    function _increaseBalance(address account, uint128 value) internal virtual override(ERC721, ERC721Enumerable) {\n        super._increaseBalance(account, value);\n    }".to_string()
            ),
            TokenExtension::ERC1155Burnable => (
                "import \"@openzeppelin/contracts/token/ERC1155/extensions/ERC1155Burnable.sol\";".to_string(),
                "ERC1155Burnable".to_string(),
                "".to_string(),
                "".to_string()
            ),
            TokenExtension::ERC1155Pausable => (
                "import \"@openzeppelin/contracts/token/ERC1155/extensions/ERC1155Pausable.sol\";".to_string(),
                "ERC1155Pausable".to_string(),
                "".to_string(),
                "    function pause() public onlyOwner {\n        _pause();\n    }\n\n    function unpause() public onlyOwner {\n        _unpause();\n    }".to_string()
            ),
            // Additional ERC20 Extensions
            TokenExtension::ERC20Capped => (
                "import \"@openzeppelin/contracts/token/ERC20/extensions/ERC20Capped.sol\";".to_string(),
                "ERC20Capped".to_string(),
                "".to_string(),
                "".to_string()
            ),
            TokenExtension::ERC20Wrapper => (
                "import \"@openzeppelin/contracts/token/ERC20/extensions/ERC20Wrapper.sol\";".to_string(),
                "ERC20Wrapper".to_string(),
                "".to_string(),
                "".to_string()
            ),
            TokenExtension::ERC20FlashMint => (
                "import \"@openzeppelin/contracts/token/ERC20/extensions/ERC20FlashMint.sol\";".to_string(),
                "ERC20FlashMint".to_string(),
                "".to_string(),
                "".to_string()
            ),
            TokenExtension::ERC20TemporaryApproval => (
                "import \"@openzeppelin/contracts/token/ERC20/extensions/draft-ERC20TemporaryApproval.sol\";".to_string(),
                "ERC20TemporaryApproval".to_string(),
                "".to_string(),
                "".to_string()
            ),
            TokenExtension::ERC20Bridgeable => (
                "import \"@openzeppelin/contracts/token/ERC20/extensions/draft-ERC20Bridgeable.sol\";".to_string(),
                "ERC20Bridgeable".to_string(),
                "".to_string(),
                "".to_string()
            ),
            TokenExtension::ERC1363 => (
                "import \"@openzeppelin/contracts/token/ERC20/extensions/ERC1363.sol\";".to_string(),
                "ERC1363".to_string(),
                "".to_string(),
                "".to_string()
            ),
            TokenExtension::ERC4626 => (
                "import \"@openzeppelin/contracts/token/ERC20/extensions/ERC4626.sol\";".to_string(),
                "ERC4626".to_string(),
                "".to_string(),
                "".to_string()
            ),
            // Additional ERC721 Extensions
            TokenExtension::ERC721Consecutive => (
                "import \"@openzeppelin/contracts/token/ERC721/extensions/ERC721Consecutive.sol\";".to_string(),
                "ERC721Consecutive".to_string(),
                "".to_string(),
                "".to_string()
            ),
            TokenExtension::ERC721URIStorage => (
                "import \"@openzeppelin/contracts/token/ERC721/extensions/ERC721URIStorage.sol\";".to_string(),
                "ERC721URIStorage".to_string(),
                "".to_string(),
                "".to_string()
            ),
            TokenExtension::ERC721Votes => (
                "import \"@openzeppelin/contracts/token/ERC721/extensions/ERC721Votes.sol\";\nimport \"@openzeppelin/contracts/utils/cryptography/EIP712.sol\";".to_string(),
                "ERC721Votes, EIP712".to_string(),
                "".to_string(),
                "".to_string()
            ),
            TokenExtension::ERC721Royalty => (
                "import \"@openzeppelin/contracts/token/ERC721/extensions/ERC721Royalty.sol\";".to_string(),
                "ERC721Royalty".to_string(),
                "".to_string(),
                "".to_string()
            ),
            TokenExtension::ERC721Wrapper => (
                "import \"@openzeppelin/contracts/token/ERC721/extensions/ERC721Wrapper.sol\";\nimport \"@openzeppelin/contracts/token/ERC721/IERC721.sol\";".to_string(),
                "ERC721Wrapper".to_string(),
                "".to_string(),
                "    // ERC721Wrapper requires an underlying token address\n    // Replace with actual token address when deploying\n    address constant UNDERLYING_TOKEN = address(0x0);".to_string()
            ),
            // Additional ERC1155 Extensions
            TokenExtension::ERC1155Supply => (
                "import \"@openzeppelin/contracts/token/ERC1155/extensions/ERC1155Supply.sol\";".to_string(),
                "ERC1155Supply".to_string(),
                "".to_string(),
                "".to_string()
            ),
            TokenExtension::ERC1155URIStorage => (
                "import \"@openzeppelin/contracts/token/ERC1155/extensions/ERC1155URIStorage.sol\";".to_string(),
                "ERC1155URIStorage".to_string(),
                "".to_string(),
                "    function uri(uint256 tokenId) public view virtual override(ERC1155, ERC1155URIStorage) returns (string memory) {\n        return super.uri(tokenId);\n    }".to_string()
            )
        }
    }

    fn generate_state_variables(&self, extensions: &[TokenExtension]) -> String {
        let mut state_vars = Vec::new();
        
        for extension in extensions {
            match extension {
                TokenExtension::ERC721Enumerable | TokenExtension::ERC721Burnable => {
                    if !state_vars.contains(&"    uint256 private _tokenIdCounter;") {
                        state_vars.push("    uint256 private _tokenIdCounter;");
                    }
                },
                _ => {}
            }
        }
        
        state_vars.join("\n")
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