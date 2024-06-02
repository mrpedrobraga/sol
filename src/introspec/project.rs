use miette::Diagnostic;
use std::{fs, path::PathBuf};
use thiserror::Error;

/// Compilation of `.sol` can only happen within
/// a valid project.
pub struct Project {
    pub index_directory: PathBuf,
}

#[derive(Error, Debug, Diagnostic)]
#[error("Error loading project.")]
pub enum ProjectLoadError {
    IO(#[from] std::io::Error),
}

impl Project {
    pub fn load_from_directory(path: PathBuf) -> Result<(), ProjectLoadError> {
        let mut dir = fs::read_dir(path)?;

        if dir.any(|entry| {
            entry
                .map(|entry| entry.file_name() == "index.sol")
                .unwrap_or(false)
        }) {
            println!("Some index.sol");
        } else {
            println!("No index.sol");
        }

        Ok(())
    }
}

pub struct File {
    uri: String,
}
