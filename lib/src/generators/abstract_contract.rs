use crate::{Result, NothungError, ProjectType, Language, templates::{SolidityTemplate, Template}, project::Project};

/// Abstract contract generator for creating empty abstract contracts
pub struct AbstractContractGenerator {
    project: ProjectType,
    language: Language,
    name: String,
    pragma: Option<String>,
    license: Option<String>,
}

impl AbstractContractGenerator {
    /// Create a new abstract contract generator
    pub fn new(
        project: ProjectType,
        language: Language,
        name: String,
        pragma: Option<String>,
        license: Option<String>,
    ) -> Self {
        Self {
            project,
            language,
            name,
            pragma,
            license,
        }
    }

    /// Generate the abstract contract
    pub fn generate(&self) -> Result<()> {
        match self.language {
            Language::Solidity => self.generate_solidity_abstract(),
            Language::RustStylus => self.generate_rust_abstract(),
        }
    }

    fn generate_solidity_abstract(&self) -> Result<()> {
        let pragma = self.pragma.as_deref().unwrap_or("0.8.30");
        let license = self.license.as_deref().unwrap_or("UNLICENSED");

        let template = SolidityTemplate::new(
            self.name.clone(),
            crate::templates::ContractType::Abstract,
            pragma.to_string(),
            license.to_string(),
        );

        let content = template.generate_abstract_contract();

        match &self.project {
            ProjectType::Foundry(foundry) => {
                let abstracts_dir = foundry.src_dir().join("abstracts");
                std::fs::create_dir_all(&abstracts_dir)?;
                
                let file_path = abstracts_dir.join(format!("{}.sol", self.name));
                std::fs::write(file_path, content)?;
                
                println!("✅ Abstract contract {} created successfully!", self.name);
                Ok(())
            }
            ProjectType::Cargo(_) => Err(NothungError::Other(
                "Abstract contract generation for Rust/Stylus projects is not yet supported".to_string()
            ))
        }
    }

    fn generate_rust_abstract(&self) -> Result<()> {
        // For now, return an error since Rust/Stylus abstract contracts might work differently
        Err(NothungError::Other(
            "Abstract contract generation for Rust/Stylus projects is not yet supported. Use traits for similar patterns.".to_string()
        ))
    }
}