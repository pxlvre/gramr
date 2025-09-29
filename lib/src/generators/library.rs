use crate::error::{NothungError, Result};
use crate::language::Language;
use crate::project::{Project, ProjectType};
use crate::templates::{Template, SolidityTemplate, StylusTemplate, ContractType};
use colored::*;
use std::fs;

pub struct LibraryGenerator {
    project: ProjectType,
    language: Language,
    library_name: String,
    pragma: Option<String>,  // Only for Solidity
    license: Option<String>, // Only for Solidity
}

impl LibraryGenerator {
    pub fn new(
        project: ProjectType,
        language: Language,
        library_name: String,
        pragma: Option<String>,
        license: Option<String>,
    ) -> Self {
        Self {
            project,
            language,
            library_name,
            pragma,
            license,
        }
    }

    pub fn generate(&self) -> Result<()> {
        self.validate_name()?;
        self.project.ensure_directories()?;
        
        // Ensure libraries directory exists
        self.ensure_libraries_directory()?;

        let template: Box<dyn Template> = match self.language {
            Language::Solidity => {
                Box::new(SolidityTemplate::new(
                    self.library_name.clone(),
                    ContractType::Basic, // Libraries don't use contract types
                    self.pragma.clone().unwrap_or_else(|| "0.8.30".to_string()),
                    self.license.clone().unwrap_or_else(|| "UNLICENSED".to_string()),
                ))
            }
            Language::RustStylus => {
                Box::new(StylusTemplate::new(
                    self.library_name.clone(),
                    ContractType::Basic, // Libraries don't use contract types
                ))
            }
        };

        self.create_library_file(&*template)?;
        self.print_success();
        Ok(())
    }

    fn validate_name(&self) -> Result<()> {
        if self.library_name.is_empty() {
            return Err(NothungError::Other("Library name cannot be empty".to_string()));
        }

        if !self.library_name.chars().next().unwrap().is_alphabetic() {
            return Err(NothungError::Other(
                "Library name must start with a letter".to_string()
            ));
        }

        if !self.library_name.chars().all(|c| c.is_alphanumeric() || c == '_') {
            return Err(NothungError::Other(
                "Library name can only contain letters, numbers, and underscores".to_string()
            ));
        }

        Ok(())
    }

    fn ensure_libraries_directory(&self) -> Result<()> {
        let libraries_dir = match self.language {
            Language::Solidity => self.project.src_dir().join("libraries"),
            Language::RustStylus => self.project.src_dir().join("libraries"),
        };

        fs::create_dir_all(&libraries_dir)
            .map_err(|e| NothungError::Other(format!("Failed to create libraries directory: {}", e)))?;
        
        Ok(())
    }

    fn create_library_file(&self, template: &dyn Template) -> Result<()> {
        let content = template.generate_library();
        let (file_path, relative_path) = match self.language {
            Language::Solidity => {
                let file_name = format!("{}.sol", self.library_name);
                let path = self.project.src_dir().join("libraries").join(&file_name);
                let relative = format!("src/libraries/{}", file_name);
                (path, relative)
            }
            Language::RustStylus => {
                let file_name = format!("{}.rs", self.library_name.to_lowercase());
                let path = self.project.src_dir().join("libraries").join(&file_name);
                let relative = format!("src/libraries/{}", file_name);
                (path, relative)
            }
        };
        
        fs::write(&file_path, content)
            .map_err(|e| NothungError::Other(format!("Failed to write library file: {}", e)))?;
        
        println!("{} Created library: {}", "✓".green(), relative_path);
        Ok(())
    }

    fn print_success(&self) {
        println!("\n{} Library generation complete!", "📚".bold());
        println!("\nNext steps:");
        println!("  1. Review the generated library file");
        println!("  2. Add your custom functions and data structures");
        
        match self.language {
            Language::Solidity => {
                println!("  3. Import in contracts with: {}", format!("import \"./libraries/{}.sol\";", self.library_name).cyan());
                println!("  4. Use library functions with: {}", format!("using {} for <Type>;", self.library_name).cyan());
                println!("  5. Run {} to compile", "forge build".cyan());
            }
            Language::RustStylus => {
                println!("  3. Add to lib.rs: {}", format!("pub mod libraries;").cyan());
                println!("  4. Import in contracts with: {}", format!("use crate::libraries::{};", self.library_name.to_lowercase()).cyan());
                println!("  5. Run {} to build", "cargo build --release".cyan());
            }
        }
    }
}