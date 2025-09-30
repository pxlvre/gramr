use crate::error::{GramrError, Result};
use crate::project::Project;
use std::fs;
use std::path::{Path, PathBuf};
use which::which;

#[derive(Clone)]
pub struct FoundryProject {
    pub root: PathBuf,
    pub src_dir: PathBuf,
    pub test_dir: PathBuf,
    pub script_dir: PathBuf,
}

impl FoundryProject {
    pub fn detect() -> Result<Self> {
        let current_dir = std::env::current_dir()?;

        if !Self::has_forge()? {
            return Err(GramrError::FoundryNotInstalled);
        }

        let root = Self::find_foundry_root(&current_dir)?;

        let src_dir = root.join("src");
        let test_dir = root.join("test");
        let script_dir = root.join("script");

        Ok(Self {
            root,
            src_dir,
            test_dir,
            script_dir,
        })
    }

    fn has_forge() -> Result<bool> {
        Ok(which("forge").is_ok())
    }

    fn find_foundry_root(start_dir: &Path) -> Result<PathBuf> {
        let mut current = start_dir.to_path_buf();

        loop {
            if current.join("foundry.toml").exists() {
                return Ok(current);
            }

            if !current.pop() {
                return Err(GramrError::NotFoundryProject);
            }
        }
    }

    pub fn ensure_directories(&self) -> Result<()> {
        fs::create_dir_all(&self.src_dir)?;
        fs::create_dir_all(&self.test_dir)?;
        fs::create_dir_all(&self.script_dir)?;
        Ok(())
    }

    pub fn has_openzeppelin(&self) -> bool {
        self.root.join("lib/openzeppelin-contracts").exists()
    }

    pub fn has_openzeppelin_upgradeable(&self) -> bool {
        self.root
            .join("lib/openzeppelin-contracts-upgradeable")
            .exists()
    }

    pub fn install_openzeppelin(&self) -> Result<()> {
        use std::process::Command;

        let output = Command::new("forge")
            .args(&["install", "OpenZeppelin/openzeppelin-contracts"])
            .current_dir(&self.root)
            .output()?;

        if !output.status.success() {
            return Err(GramrError::ForgeCommandError(
                String::from_utf8_lossy(&output.stderr).to_string(),
            ));
        }

        Ok(())
    }

    pub fn install_openzeppelin_upgradeable(&self) -> Result<()> {
        use std::process::Command;

        let output = Command::new("forge")
            .args(&["install", "OpenZeppelin/openzeppelin-contracts-upgradeable"])
            .current_dir(&self.root)
            .output()?;

        if !output.status.success() {
            return Err(GramrError::ForgeCommandError(
                String::from_utf8_lossy(&output.stderr).to_string(),
            ));
        }

        Ok(())
    }
}

impl Project for FoundryProject {
    fn ensure_directories(&self) -> Result<()> {
        self.ensure_directories()
    }

    fn src_dir(&self) -> PathBuf {
        self.src_dir.clone()
    }

    fn test_dir(&self) -> PathBuf {
        self.test_dir.clone()
    }

    fn script_dir(&self) -> PathBuf {
        self.script_dir.clone()
    }

    fn has_openzeppelin(&self) -> bool {
        self.has_openzeppelin()
    }

    fn install_openzeppelin(&self) -> Result<()> {
        self.install_openzeppelin()
    }

    fn has_openzeppelin_upgradeable(&self) -> bool {
        self.has_openzeppelin_upgradeable()
    }

    fn install_openzeppelin_upgradeable(&self) -> Result<()> {
        self.install_openzeppelin_upgradeable()
    }
}
