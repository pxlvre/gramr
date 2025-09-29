use crate::error::{NothungError, Result};
use crate::language::Language;
use std::path::PathBuf;
use std::process::Command;
use std::fs;

pub trait Project {
    fn ensure_directories(&self) -> Result<()>;
    fn src_dir(&self) -> PathBuf;
    fn test_dir(&self) -> PathBuf;
    fn script_dir(&self) -> PathBuf;
    fn has_openzeppelin(&self) -> bool;
    fn install_openzeppelin(&self) -> Result<()>;
    fn has_openzeppelin_upgradeable(&self) -> bool;
    fn install_openzeppelin_upgradeable(&self) -> Result<()>;
}

pub enum ProjectType {
    Foundry(crate::foundry::FoundryProject),
    Cargo(CargoProject),
}

impl Project for ProjectType {
    fn ensure_directories(&self) -> Result<()> {
        match self {
            ProjectType::Foundry(p) => p.ensure_directories(),
            ProjectType::Cargo(p) => p.ensure_directories(),
        }
    }

    fn src_dir(&self) -> PathBuf {
        match self {
            ProjectType::Foundry(p) => p.src_dir(),
            ProjectType::Cargo(p) => p.src_dir(),
        }
    }

    fn test_dir(&self) -> PathBuf {
        match self {
            ProjectType::Foundry(p) => p.test_dir(),
            ProjectType::Cargo(p) => p.test_dir(),
        }
    }

    fn script_dir(&self) -> PathBuf {
        match self {
            ProjectType::Foundry(p) => p.script_dir(),
            ProjectType::Cargo(p) => p.script_dir(),
        }
    }

    fn has_openzeppelin(&self) -> bool {
        match self {
            ProjectType::Foundry(p) => p.has_openzeppelin(),
            ProjectType::Cargo(p) => p.has_openzeppelin(),
        }
    }

    fn install_openzeppelin(&self) -> Result<()> {
        match self {
            ProjectType::Foundry(p) => p.install_openzeppelin(),
            ProjectType::Cargo(p) => p.install_openzeppelin(),
        }
    }

    fn has_openzeppelin_upgradeable(&self) -> bool {
        match self {
            ProjectType::Foundry(p) => p.has_openzeppelin_upgradeable(),
            ProjectType::Cargo(p) => p.has_openzeppelin_upgradeable(),
        }
    }

    fn install_openzeppelin_upgradeable(&self) -> Result<()> {
        match self {
            ProjectType::Foundry(p) => p.install_openzeppelin_upgradeable(),
            ProjectType::Cargo(p) => p.install_openzeppelin_upgradeable(),
        }
    }
}

impl ProjectType {
    pub fn detect(language: &Language) -> Result<Self> {
        match language {
            Language::Solidity => {
                let project = crate::foundry::FoundryProject::detect()?;
                Ok(ProjectType::Foundry(project))
            }
            Language::RustStylus => {
                let project = CargoProject::detect()?;
                Ok(ProjectType::Cargo(project))
            }
        }
    }
}

pub struct CargoProject {
    root: PathBuf,
}

impl CargoProject {
    pub fn detect() -> Result<Self> {
        let current_dir = std::env::current_dir()
            .map_err(|e| NothungError::Other(format!("Failed to get current directory: {}", e)))?;
        
        let cargo_toml = current_dir.join("Cargo.toml");
        if !cargo_toml.exists() {
            return Err(NothungError::ProjectNotFound(
                "No Cargo.toml found. Please run from a Rust project directory.".to_string()
            ));
        }

        Ok(Self {
            root: current_dir,
        })
    }

    fn cargo_toml_path(&self) -> PathBuf {
        self.root.join("Cargo.toml")
    }
}

impl Project for CargoProject {
    fn ensure_directories(&self) -> Result<()> {
        let src = self.src_dir();
        if !src.exists() {
            fs::create_dir_all(&src)
                .map_err(|e| NothungError::Other(format!("Failed to create src directory: {}", e)))?;
        }
        
        let tests = self.test_dir();
        if !tests.exists() {
            fs::create_dir_all(&tests)
                .map_err(|e| NothungError::Other(format!("Failed to create tests directory: {}", e)))?;
        }
        
        Ok(())
    }

    fn src_dir(&self) -> PathBuf {
        self.root.join("src")
    }

    fn test_dir(&self) -> PathBuf {
        self.root.join("tests")
    }

    fn script_dir(&self) -> PathBuf {
        // Stylus projects don't have a script directory like Foundry
        self.root.join("scripts")
    }

    fn has_openzeppelin(&self) -> bool {
        // Check if openzeppelin-stylus is in Cargo.toml
        if let Ok(content) = fs::read_to_string(self.cargo_toml_path()) {
            content.contains("openzeppelin-stylus")
        } else {
            false
        }
    }

    fn install_openzeppelin(&self) -> Result<()> {
        let output = Command::new("cargo")
            .args(&["add", "openzeppelin-stylus@=0.3.0"])
            .current_dir(&self.root)
            .output()
            .map_err(|e| NothungError::Other(format!("Failed to run cargo add: {}", e)))?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(NothungError::Other(
                format!("Failed to add openzeppelin-stylus: {}", stderr)
            ));
        }
        
        Ok(())
    }

    fn has_openzeppelin_upgradeable(&self) -> bool {
        // For Stylus, upgradeable contracts are part of the same package
        self.has_openzeppelin()
    }

    fn install_openzeppelin_upgradeable(&self) -> Result<()> {
        // Upgradeable contracts are not yet available for OpenZeppelin Stylus
        Err(NothungError::Other(
            "Upgradeable contracts are not yet supported for Rust/Stylus projects".to_string()
        ))
    }
}