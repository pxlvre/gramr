use crate::{Result, GramrError, ProjectType, Language, templates::SolidityTemplate, project::Project};

/// Interface generator for creating empty interfaces
pub struct InterfaceGenerator {
    project: ProjectType,
    language: Language,
    name: String,
    pragma: Option<String>,
    license: Option<String>,
}

impl InterfaceGenerator {
    /// Create a new interface generator
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

    /// Generate the interface
    pub fn generate(&self) -> Result<()> {
        match self.language {
            Language::Solidity => self.generate_solidity_interface(),
            Language::RustStylus => self.generate_rust_interface(),
        }
    }

    fn generate_solidity_interface(&self) -> Result<()> {
        let pragma = self.pragma.as_deref().unwrap_or("0.8.30");
        let license = self.license.as_deref().unwrap_or("UNLICENSED");

        let template = SolidityTemplate::new(
            self.name.clone(),
            crate::templates::ContractType::Interface,
            pragma.to_string(),
            license.to_string(),
        );

        let content = template.generate_interface();

        match &self.project {
            ProjectType::Foundry(foundry) => {
                let interfaces_dir = foundry.src_dir().join("interfaces");
                std::fs::create_dir_all(&interfaces_dir)?;
                
                let file_path = interfaces_dir.join(format!("I{}.sol", self.name));
                std::fs::write(file_path, content)?;
                
                println!("✅ Interface I{} created successfully!", self.name);
                Ok(())
            }
            ProjectType::Cargo(_) => Err(GramrError::Other(
                "Interface generation for Rust/Stylus projects is not yet supported".to_string()
            ))
        }
    }

    fn generate_rust_interface(&self) -> Result<()> {
        // For now, return an error since Rust/Stylus interfaces might work differently
        Err(GramrError::Other(
            "Interface generation for Rust/Stylus projects is not yet supported. Use traits instead.".to_string()
        ))
    }
}