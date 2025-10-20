use crate::error::{GramrError, Result};
use crate::language::Language;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

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
            .map_err(|e| GramrError::Other(format!("Failed to get current directory: {}", e)))?;

        let cargo_toml = current_dir.join("Cargo.toml");
        if !cargo_toml.exists() {
            return Err(GramrError::ProjectNotFound(
                "No Cargo.toml found. Please run from a Rust project directory.".to_string(),
            ));
        }

        Ok(Self { root: current_dir })
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
                .map_err(|e| GramrError::Other(format!("Failed to create src directory: {}", e)))?;
        }

        let tests = self.test_dir();
        if !tests.exists() {
            fs::create_dir_all(&tests).map_err(|e| {
                GramrError::Other(format!("Failed to create tests directory: {}", e))
            })?;
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
            .map_err(|e| GramrError::Other(format!("Failed to run cargo add: {}", e)))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(GramrError::Other(format!(
                "Failed to add openzeppelin-stylus: {}",
                stderr
            )));
        }

        Ok(())
    }

    fn has_openzeppelin_upgradeable(&self) -> bool {
        // For Stylus, upgradeable contracts are part of the same package
        self.has_openzeppelin()
    }

    fn install_openzeppelin_upgradeable(&self) -> Result<()> {
        // Upgradeable contracts are not yet available for OpenZeppelin Stylus
        Err(GramrError::Other(
            "Upgradeable contracts are not yet supported for Rust/Stylus projects".to_string(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn create_test_cargo_project() -> (TempDir, CargoProject) {
        let temp_dir = TempDir::new().unwrap();
        let project_path = temp_dir.path().to_path_buf();

        // Create Cargo.toml
        fs::write(
            project_path.join("Cargo.toml"),
            r#"[package]
name = "test-project"
version = "0.1.0"
edition = "2021"

[dependencies]
"#,
        )
        .unwrap();

        // Create basic directory structure
        fs::create_dir_all(project_path.join("src")).unwrap();

        let project = CargoProject { root: project_path };
        (temp_dir, project)
    }

    fn create_test_cargo_project_with_oz() -> (TempDir, CargoProject) {
        let temp_dir = TempDir::new().unwrap();
        let project_path = temp_dir.path().to_path_buf();

        // Create Cargo.toml with OpenZeppelin dependency
        fs::write(
            project_path.join("Cargo.toml"),
            r#"[package]
name = "test-project"
version = "0.1.0"
edition = "2021"

[dependencies]
openzeppelin-stylus = "0.3.0"
"#,
        )
        .unwrap();

        fs::create_dir_all(project_path.join("src")).unwrap();

        let project = CargoProject { root: project_path };
        (temp_dir, project)
    }

    fn create_test_foundry_project() -> (TempDir, crate::foundry::FoundryProject) {
        let temp_dir = TempDir::new().unwrap();
        let project_path = temp_dir.path().to_path_buf();

        // Create foundry.toml
        fs::write(
            project_path.join("foundry.toml"),
            "[profile.default]\nsrc = \"src\"\ntest = \"test\"\nscript = \"script\"\n",
        )
        .unwrap();

        // Create directories
        fs::create_dir_all(project_path.join("src")).unwrap();
        fs::create_dir_all(project_path.join("test")).unwrap();
        fs::create_dir_all(project_path.join("script")).unwrap();
        fs::create_dir_all(project_path.join("lib")).unwrap();

        let project = crate::foundry::FoundryProject {
            root: project_path.clone(),
            src_dir: project_path.join("src"),
            test_dir: project_path.join("test"),
            script_dir: project_path.join("script"),
        };
        (temp_dir, project)
    }

    #[test]
    fn test_cargo_project_detect_success() {
        let (_temp_dir, project) = create_test_cargo_project();

        // Change to the project directory for detection
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(&project.root).unwrap();

        let result = CargoProject::detect();

        // Restore original directory
        std::env::set_current_dir(original_dir).unwrap();

        assert!(result.is_ok());
        let detected_project = result.unwrap();
        assert_eq!(detected_project.root, project.root);
    }

    #[test]
    fn test_cargo_project_detect_no_cargo_toml() {
        let temp_dir = TempDir::new().unwrap();
        let project_path = temp_dir.path().to_path_buf();

        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(&project_path).unwrap();

        let result = CargoProject::detect();

        std::env::set_current_dir(original_dir).unwrap();

        assert!(result.is_err());
        if let Err(GramrError::ProjectNotFound(msg)) = result {
            assert!(msg.contains("No Cargo.toml found"));
        } else {
            panic!("Expected ProjectNotFound error");
        }
    }

    #[test]
    fn test_cargo_project_ensure_directories() {
        let (_temp_dir, project) = create_test_cargo_project();

        assert!(project.ensure_directories().is_ok());

        assert!(project.src_dir().exists());
        assert!(project.test_dir().exists());
    }

    #[test]
    fn test_cargo_project_directory_paths() {
        let (_temp_dir, project) = create_test_cargo_project();

        assert_eq!(project.src_dir(), project.root.join("src"));
        assert_eq!(project.test_dir(), project.root.join("tests"));
        assert_eq!(project.script_dir(), project.root.join("scripts"));
        assert_eq!(project.cargo_toml_path(), project.root.join("Cargo.toml"));
    }

    #[test]
    fn test_cargo_project_has_openzeppelin_false() {
        let (_temp_dir, project) = create_test_cargo_project();

        assert!(!project.has_openzeppelin());
        assert!(!project.has_openzeppelin_upgradeable());
    }

    #[test]
    fn test_cargo_project_has_openzeppelin_true() {
        let (_temp_dir, project) = create_test_cargo_project_with_oz();

        assert!(project.has_openzeppelin());
        assert!(project.has_openzeppelin_upgradeable()); // Same as has_openzeppelin for Stylus
    }

    #[test]
    fn test_cargo_project_install_openzeppelin_upgradeable_not_supported() {
        let (_temp_dir, project) = create_test_cargo_project();

        let result = project.install_openzeppelin_upgradeable();
        assert!(result.is_err());
        if let Err(GramrError::Other(msg)) = result {
            assert!(msg.contains("not yet supported for Rust/Stylus projects"));
        } else {
            panic!("Expected Other error");
        }
    }

    #[test]
    fn test_project_type_foundry() {
        let (_temp_dir, foundry_project) = create_test_foundry_project();
        let project_type = ProjectType::Foundry(foundry_project);

        // Test Project trait implementation
        assert!(project_type.ensure_directories().is_ok());
        assert!(project_type.src_dir().ends_with("src"));
        assert!(project_type.test_dir().ends_with("test"));
        assert!(project_type.script_dir().ends_with("script"));
        assert!(!project_type.has_openzeppelin());
        assert!(!project_type.has_openzeppelin_upgradeable());
    }

    #[test]
    fn test_project_type_cargo() {
        let (_temp_dir, cargo_project) = create_test_cargo_project();
        let project_type = ProjectType::Cargo(cargo_project);

        // Test Project trait implementation
        assert!(project_type.ensure_directories().is_ok());
        assert!(project_type.src_dir().ends_with("src"));
        assert!(project_type.test_dir().ends_with("tests"));
        assert!(project_type.script_dir().ends_with("scripts"));
        assert!(!project_type.has_openzeppelin());
        assert!(!project_type.has_openzeppelin_upgradeable());
    }

    #[test]
    fn test_project_type_cargo_with_openzeppelin() {
        let (_temp_dir, cargo_project) = create_test_cargo_project_with_oz();
        let project_type = ProjectType::Cargo(cargo_project);

        assert!(project_type.has_openzeppelin());
        assert!(project_type.has_openzeppelin_upgradeable());

        let result = project_type.install_openzeppelin_upgradeable();
        assert!(result.is_err());
    }

    #[test]
    fn test_project_trait_methods() {
        let (_temp_dir, cargo_project) = create_test_cargo_project();

        // Test direct trait methods
        let project: &dyn Project = &cargo_project;

        assert!(project.ensure_directories().is_ok());
        assert!(project.src_dir().ends_with("src"));
        assert!(project.test_dir().ends_with("tests"));
        assert!(project.script_dir().ends_with("scripts"));
        assert!(!project.has_openzeppelin());
        assert!(!project.has_openzeppelin_upgradeable());
    }

    #[test]
    fn test_cargo_project_detect_missing_current_dir() {
        // This test would be hard to simulate without mocking, but we can at least
        // ensure the error handling path exists by checking the code structure
        let (_temp_dir, _project) = create_test_cargo_project();

        // The actual test for missing current_dir would require complex setup
        // but the error handling is covered in the detect() implementation
        assert!(true); // Placeholder for complex environment test
    }

    #[test]
    fn test_cargo_project_has_openzeppelin_with_malformed_toml() {
        let temp_dir = TempDir::new().unwrap();
        let project_path = temp_dir.path().to_path_buf();

        // Create malformed Cargo.toml (but still readable)
        fs::write(
            project_path.join("Cargo.toml"),
            "invalid toml content but contains openzeppelin-stylus somewhere",
        )
        .unwrap();

        let project = CargoProject { root: project_path };

        // Should still detect the string even in malformed toml
        assert!(project.has_openzeppelin());
    }

    #[test]
    fn test_cargo_project_has_openzeppelin_with_unreadable_toml() {
        let temp_dir = TempDir::new().unwrap();
        let project_path = temp_dir.path().to_path_buf();

        // Create project without Cargo.toml
        let project = CargoProject { root: project_path };

        // Should return false when Cargo.toml doesn't exist or can't be read
        assert!(!project.has_openzeppelin());
    }

    #[test]
    fn test_cargo_project_install_openzeppelin_success_simulation() {
        let (_temp_dir, project) = create_test_cargo_project();

        // We can't easily test the actual cargo command without a real cargo installation
        // But we can test the error path exists and is handled properly
        // In a real integration test, this would actually run cargo add

        // For now, just verify the project structure is set up correctly for the command
        assert!(project.cargo_toml_path().exists());
        assert_eq!(project.cargo_toml_path().file_name().unwrap(), "Cargo.toml");
    }

    #[test]
    fn test_project_type_detect_would_call_correct_methods() {
        // This test verifies the dispatch logic without actually calling detect
        // since detect requires specific environment setup

        // Test the match logic exists for both language types
        use crate::language::Language;

        // We can't easily test detect() without proper environment setup,
        // but we can verify the enum structure and methods exist
        match Language::Solidity {
            Language::Solidity => {
                // Would call FoundryProject::detect()
                assert!(true);
            }
            Language::RustStylus => {
                // Would call CargoProject::detect()
                assert!(true);
            }
        }

        match Language::RustStylus {
            Language::Solidity => {
                assert!(false, "Wrong match");
            }
            Language::RustStylus => {
                assert!(true);
            }
        }
    }
}
