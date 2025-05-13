use std::path::PathBuf;

use thiserror::Error;

use crate::lexer::Token;

#[derive(Debug, Error)]
pub enum Errors {
    // CLI Errors
    #[error("Failed to find current working directory")]
    CWDError(),
    #[error("Failed to find project.toml in {0}")]
    ProjectFileError(PathBuf),
    #[error("Invalid project.toml")]
    InvalidProjectFile(),
    #[error("No input files provided")]
    NoInputFiles(),
    // IO
    #[error("Failed to read directory {0}")]
    ReadDirError(String),
    #[error("Failed to read file {0}")]
    ReadFileError(String),
    // Parser / AST
    #[error("Invalid token {0}")]
    InvalidToken(String),
}
