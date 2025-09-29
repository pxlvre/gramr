use thiserror::Error;

#[derive(Error, Debug)]
pub enum NothungError {
    #[error("Not in a Foundry project. Please run this command from a Foundry project directory.")]
    NotFoundryProject,

    #[error("Foundry is not installed. Please install Foundry first: https://getfoundry.sh")]
    FoundryNotInstalled,
    
    #[error("Project not found: {0}")]
    ProjectNotFound(String),

    #[error("File already exists: {0}")]
    FileExists(String),

    #[error("Failed to create file: {0}")]
    FileCreationError(String),

    #[error("Failed to run forge command: {0}")]
    ForgeCommandError(String),

    #[error("Invalid contract name: {0}")]
    InvalidContractName(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Other error: {0}")]
    Other(String),
}

pub type Result<T> = std::result::Result<T, NothungError>;
