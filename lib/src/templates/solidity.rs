
use super::{ContractType, TokenExtension, Template};

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
            ContractType::Interface => self.generate_interface(),
            ContractType::Abstract => self.generate_abstract_contract(),
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
    
    pub fn generate_library(&self) -> String {
        format!(
r#"// SPDX-License-Identifier: {}
pragma solidity ^{};

/// @title {}
/// @notice A library contract for reusable utility functions
/// @dev Add your custom functions here
library {} {{
    /// @notice Example function - replace with your own
    /// @param value The input value
    /// @return The processed result
    function exampleFunction(uint256 value) internal pure returns (uint256) {{
        return value * 2;
    }}
    
    /// @notice Example struct for data organization
    struct Data {{
        uint256 id;
        address owner;
        bool isActive;
    }}
    
    /// @notice Example function that works with structs
    /// @param data The input data struct
    /// @return Whether the data is valid
    function validateData(Data memory data) internal pure returns (bool) {{
        return data.owner != address(0) && data.isActive;
    }}
}}
"#,
            self.license,
            self.pragma,
            self.contract_name,
            self.contract_name
        )
    }

    pub fn generate_interface(&self) -> String {
        format!(
r#"// SPDX-License-Identifier: {}
pragma solidity ^{};

/// @title I{}
/// @notice Interface for {}
/// @dev Define your function signatures here
interface I{} {{
    // Add your function signatures here
    // Example:
    // function exampleFunction(uint256 value) external returns (bool);
}}
"#,
            self.license,
            self.pragma,
            self.contract_name,
            self.contract_name,
            self.contract_name
        )
    }

    pub fn generate_abstract_contract(&self) -> String {
        format!(
r#"// SPDX-License-Identifier: {}
pragma solidity ^{};

/// @title {}
/// @notice Abstract contract for {}
/// @dev Implement the required functions in the inheriting contract
abstract contract {} {{
    // Add your abstract functions and state variables here
    // Example:
    // function abstractFunction() public virtual returns (uint256);
    
    // You can also implement concrete functions
    function concreteFunction() public pure returns (string memory) {{
        return "This is a concrete function";
    }}
}}
"#,
            self.license,
            self.pragma,
            self.contract_name,
            self.contract_name,
            self.contract_name
        )
    }
}

impl Template for SolidityTemplate {
    fn generate_contract(&self) -> String {
        self.generate_contract()
    }
    
    fn generate_test(&self) -> String {
        self.generate_test()
    }
    
    fn generate_script(&self) -> String {
        self.generate_script()
    }
    
    fn generate_library(&self) -> String {
        self.generate_library()
    }
    
    fn generate_interface(&self) -> String {
        self.generate_interface()
    }
    
    fn generate_abstract_contract(&self) -> String {
        self.generate_abstract_contract()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_template(contract_type: ContractType) -> SolidityTemplate {
        SolidityTemplate::new(
            "TestContract".to_string(),
            contract_type,
            "0.8.30".to_string(),
            "MIT".to_string(),
        )
    }

    #[test]
    fn test_solidity_template_new() {
        let template = SolidityTemplate::new(
            "MyContract".to_string(),
            ContractType::Basic,
            "0.8.25".to_string(),
            "Apache-2.0".to_string(),
        );
        
        assert_eq!(template.contract_name, "MyContract");
        assert!(matches!(template.contract_type, ContractType::Basic));
        assert_eq!(template.pragma, "0.8.25");
        assert_eq!(template.license, "Apache-2.0");
    }

    #[test]
    fn test_generate_basic_contract() {
        let template = create_test_template(ContractType::Basic);
        let contract = template.generate_contract();
        
        assert!(contract.contains("// SPDX-License-Identifier: MIT"));
        assert!(contract.contains("pragma solidity ^0.8.30"));
        assert!(contract.contains("contract TestContract {"));
        assert!(contract.contains("constructor() {}"));
        assert!(!contract.contains("import"));
    }

    #[test]
    fn test_generate_erc20_contract() {
        let template = create_test_template(ContractType::ERC20);
        let contract = template.generate_contract();
        
        assert!(contract.contains("// SPDX-License-Identifier: MIT"));
        assert!(contract.contains("pragma solidity ^0.8.30"));
        assert!(contract.contains("import \"@openzeppelin/contracts/token/ERC20/ERC20.sol\""));
        assert!(contract.contains("contract TestContract is ERC20"));
        assert!(contract.contains("constructor(uint256 initialSupply)"));
        assert!(contract.contains("_mint(msg.sender, initialSupply)"));
    }

    #[test]
    fn test_generate_erc721_contract() {
        let template = create_test_template(ContractType::ERC721);
        let contract = template.generate_contract();
        
        assert!(contract.contains("import \"@openzeppelin/contracts/token/ERC721/ERC721.sol\""));
        assert!(contract.contains("contract TestContract is ERC721"));
        assert!(contract.contains("constructor() ERC721(\"TestContract\", \"TST\")"));
        assert!(contract.contains("function safeMint"));
    }

    #[test]
    fn test_generate_erc1155_contract() {
        let template = create_test_template(ContractType::ERC1155);
        let contract = template.generate_contract();
        
        assert!(contract.contains("import \"@openzeppelin/contracts/token/ERC1155/ERC1155.sol\""));
        assert!(contract.contains("contract TestContract is ERC1155"));
        assert!(contract.contains("function mint"));
        assert!(contract.contains("function mintBatch"));
    }

    #[test]
    fn test_generate_interface() {
        let template = create_test_template(ContractType::Interface);
        let contract = template.generate_contract();
        
        assert!(contract.contains("interface ITestContract"));
        assert!(contract.contains("/// @title ITestContract"));
        assert!(contract.contains("/// @notice Interface for TestContract"));
        assert!(!contract.contains("import"));
    }

    #[test]
    fn test_generate_abstract_contract() {
        let template = create_test_template(ContractType::Abstract);
        let contract = template.generate_contract();
        
        assert!(contract.contains("abstract contract TestContract"));
        assert!(contract.contains("/// @title TestContract"));
        assert!(contract.contains("function concreteFunction"));
        assert!(!contract.contains("import"));
    }

    #[test]
    fn test_generate_library() {
        let template = create_test_template(ContractType::Basic);
        let library = template.generate_library();
        
        assert!(library.contains("library TestContract"));
        assert!(library.contains("function exampleFunction"));
        assert!(library.contains("struct Data"));
        assert!(library.contains("function validateData"));
    }

    #[test]
    fn test_generate_test_basic() {
        let template = create_test_template(ContractType::Basic);
        let test = template.generate_test();
        
        assert!(test.contains("import \"forge-std/Test.sol\""));
        assert!(test.contains("import \"../src/TestContract.sol\""));
        assert!(test.contains("contract TestContractTest is Test"));
        assert!(test.contains("TestContract public instance"));
        assert!(test.contains("instance = new TestContract()"));
        assert!(test.contains("function test_Deployment()"));
    }

    #[test]
    fn test_generate_test_erc20() {
        let template = create_test_template(ContractType::ERC20);
        let test = template.generate_test();
        
        assert!(test.contains("instance = new TestContract(1000000 * 10 ** 18)"));
    }

    #[test]
    fn test_generate_script_basic() {
        let template = create_test_template(ContractType::Basic);
        let script = template.generate_script();
        
        assert!(script.contains("import \"forge-std/Script.sol\""));
        assert!(script.contains("import \"../src/TestContract.sol\""));
        assert!(script.contains("contract DeployTestContract is Script"));
        assert!(script.contains("function run() external returns (TestContract)"));
        assert!(script.contains("vm.envUint(\"PRIVATE_KEY\")"));
        assert!(script.contains("vm.startBroadcast"));
        assert!(script.contains("vm.stopBroadcast"));
        assert!(script.contains("TestContract instance = new TestContract()"));
    }

    #[test]
    fn test_generate_script_erc20() {
        let template = create_test_template(ContractType::ERC20);
        let script = template.generate_script();
        
        assert!(script.contains("TestContract instance = new TestContract(1000000 * 10 ** 18)"));
    }

    #[test]
    fn test_multi_inheritance_contract() {
        let template = SolidityTemplate::new(
            "MultiToken".to_string(),
            ContractType::MultiInheritance {
                base_type: Box::new(ContractType::ERC20),
                extensions: vec![TokenExtension::ERC20Burnable, TokenExtension::ERC20Pausable],
            },
            "0.8.30".to_string(),
            "MIT".to_string(),
        );
        
        let contract = template.generate_contract();
        
        assert!(contract.contains("import \"@openzeppelin/contracts/token/ERC20/ERC20.sol\""));
        assert!(contract.contains("import \"@openzeppelin/contracts/token/ERC20/extensions/ERC20Burnable.sol\""));
        assert!(contract.contains("import \"@openzeppelin/contracts/token/ERC20/extensions/ERC20Pausable.sol\""));
        assert!(contract.contains("contract MultiToken is ERC20, ERC20Burnable, ERC20Pausable"));
        assert!(contract.contains("function pause() public onlyOwner"));
        assert!(contract.contains("function unpause() public onlyOwner"));
    }

    #[test]
    fn test_erc721_with_extensions() {
        let template = SolidityTemplate::new(
            "NFTCollection".to_string(),
            ContractType::MultiInheritance {
                base_type: Box::new(ContractType::ERC721),
                extensions: vec![TokenExtension::ERC721Enumerable, TokenExtension::ERC721Burnable],
            },
            "0.8.30".to_string(),
            "MIT".to_string(),
        );
        
        let contract = template.generate_contract();
        
        assert!(contract.contains("import \"@openzeppelin/contracts/token/ERC721/extensions/ERC721Enumerable.sol\""));
        assert!(contract.contains("import \"@openzeppelin/contracts/token/ERC721/extensions/ERC721Burnable.sol\""));
        assert!(contract.contains("contract NFTCollection is ERC721, ERC721Enumerable, ERC721Burnable"));
        assert!(contract.contains("uint256 private _tokenIdCounter"));
        assert!(contract.contains("function supportsInterface"));
        assert!(contract.contains("function _update"));
        assert!(contract.contains("function _increaseBalance"));
    }

    #[test]
    fn test_extension_conversion() {
        let template = SolidityTemplate::new(
            "TestNFT".to_string(),
            ContractType::MultiInheritance {
                base_type: Box::new(ContractType::ERC721),
                extensions: vec![TokenExtension::ERC20Burnable, TokenExtension::ERC20Pausable],
            },
            "0.8.30".to_string(),
            "MIT".to_string(),
        );
        
        let contract = template.generate_contract();
        
        // Should convert ERC20 extensions to ERC721 equivalents
        assert!(contract.contains("ERC721Burnable"));
        assert!(contract.contains("ERC721Pausable"));
        assert!(!contract.contains("ERC20Burnable"));
        assert!(!contract.contains("ERC20Pausable"));
    }

    #[test]
    fn test_get_symbol() {
        let template = SolidityTemplate::new(
            "MyTokenName".to_string(),
            ContractType::ERC20,
            "0.8.30".to_string(),
            "MIT".to_string(),
        );
        
        let symbol = template.get_symbol();
        assert_eq!(symbol, "MTN");
    }

    #[test]
    fn test_get_symbol_single_word() {
        let template = SolidityTemplate::new(
            "Token".to_string(),
            ContractType::ERC20,
            "0.8.30".to_string(),
            "MIT".to_string(),
        );
        
        let symbol = template.get_symbol();
        assert_eq!(symbol, "T");
    }

    #[test]
    fn test_generate_state_variables() {
        let template = create_test_template(ContractType::Basic);
        
        let state_vars = template.generate_state_variables(&[TokenExtension::ERC721Enumerable]);
        assert!(state_vars.contains("uint256 private _tokenIdCounter"));
        
        let empty_state_vars = template.generate_state_variables(&[TokenExtension::ERC20Pausable]);
        assert!(empty_state_vars.is_empty());
    }

    #[test]
    fn test_extension_components_erc20_burnable() {
        let template = create_test_template(ContractType::Basic);
        let (import, inherit, init, func) = template.get_extension_components(&TokenExtension::ERC20Burnable);
        
        assert!(import.contains("ERC20Burnable.sol"));
        assert_eq!(inherit, "ERC20Burnable");
        assert!(init.is_empty());
        assert!(func.is_empty());
    }

    #[test]
    fn test_extension_components_pausable() {
        let template = create_test_template(ContractType::Basic);
        let (import, inherit, init, func) = template.get_extension_components(&TokenExtension::ERC20Pausable);
        
        assert!(import.contains("ERC20Pausable.sol"));
        assert_eq!(inherit, "ERC20Pausable");
        assert!(init.is_empty());
        assert!(func.contains("function pause()"));
        assert!(func.contains("function unpause()"));
    }

    #[test]
    fn test_extension_components_enumerable() {
        let template = create_test_template(ContractType::Basic);
        let (import, inherit, init, func) = template.get_extension_components(&TokenExtension::ERC721Enumerable);
        
        assert!(import.contains("ERC721Enumerable.sol"));
        assert_eq!(inherit, "ERC721Enumerable");
        assert!(init.is_empty());
        assert!(func.contains("function supportsInterface"));
        assert!(func.contains("function _update"));
        assert!(func.contains("function _increaseBalance"));
    }

    #[test]
    fn test_erc1155_uristorage() {
        let template = SolidityTemplate::new(
            "MultiToken".to_string(),
            ContractType::MultiInheritance {
                base_type: Box::new(ContractType::ERC1155),
                extensions: vec![TokenExtension::ERC1155URIStorage],
            },
            "0.8.30".to_string(),
            "MIT".to_string(),
        );
        
        let contract = template.generate_contract();
        
        assert!(contract.contains("ERC1155URIStorage"));
        assert!(contract.contains("function uri(uint256 tokenId)"));
    }

    #[test]
    fn test_template_trait_implementation() {
        let template = create_test_template(ContractType::Basic);
        
        // Test that Template trait methods work
        let contract = Template::generate_contract(&template);
        let test = Template::generate_test(&template);
        let script = Template::generate_script(&template);
        let library = Template::generate_library(&template);
        let interface = Template::generate_interface(&template);
        let abstract_contract = Template::generate_abstract_contract(&template);
        
        assert!(contract.contains("contract TestContract"));
        assert!(test.contains("TestContractTest"));
        assert!(script.contains("DeployTestContract"));
        assert!(library.contains("library TestContract"));
        assert!(interface.contains("interface ITestContract"));
        assert!(abstract_contract.contains("abstract contract TestContract"));
    }

    #[test]
    fn test_upgradeable_contracts() {
        let template = create_test_template(ContractType::ERC20Upgradeable);
        let contract = template.generate_contract();
        
        assert!(contract.contains("import \"@openzeppelin/contracts-upgradeable"));
        assert!(contract.contains("Initializable"));
        assert!(contract.contains("function initialize"));
        assert!(!contract.contains("constructor"));
    }

    #[test]
    fn test_all_erc20_extensions() {
        let extensions = vec![
            TokenExtension::ERC20Permit,
            TokenExtension::ERC20Burnable,
            TokenExtension::ERC20Capped,
            TokenExtension::ERC20Pausable,
            TokenExtension::ERC20Votes,
            TokenExtension::ERC20Wrapper,
            TokenExtension::ERC20FlashMint,
            TokenExtension::ERC20TemporaryApproval,
            TokenExtension::ERC20Bridgeable,
            TokenExtension::ERC1363,
            TokenExtension::ERC4626,
        ];
        
        for extension in extensions {
            let template = SolidityTemplate::new(
                "TestToken".to_string(),
                ContractType::MultiInheritance {
                    base_type: Box::new(ContractType::ERC20),
                    extensions: vec![extension.clone()],
                },
                "0.8.30".to_string(),
                "MIT".to_string(),
            );
            
            let contract = template.generate_contract();
            assert!(contract.contains("TestToken"));
            assert!(contract.contains("import"));
        }
    }

    #[test]
    fn test_contract_name_with_special_characters() {
        let template = SolidityTemplate::new(
            "My_Token_123".to_string(),
            ContractType::Basic,
            "0.8.30".to_string(),
            "MIT".to_string(),
        );
        
        let contract = template.generate_contract();
        assert!(contract.contains("contract My_Token_123"));
    }

    #[test]
    fn test_different_pragma_versions() {
        let template = SolidityTemplate::new(
            "TestContract".to_string(),
            ContractType::Basic,
            "0.8.19".to_string(),
            "MIT".to_string(),
        );
        
        let contract = template.generate_contract();
        assert!(contract.contains("pragma solidity ^0.8.19"));
    }

    #[test]
    fn test_different_licenses() {
        let licenses = vec!["MIT", "Apache-2.0", "GPL-3.0", "UNLICENSED"];
        
        for license in licenses {
            let template = SolidityTemplate::new(
                "TestContract".to_string(),
                ContractType::Basic,
                "0.8.30".to_string(),
                license.to_string(),
            );
            
            let contract = template.generate_contract();
            assert!(contract.contains(&format!("// SPDX-License-Identifier: {}", license)));
        }
    }
}