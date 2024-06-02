use clap::{Args, Subcommand};
use miette::Diagnostic;
use std::{fs::File, path::PathBuf};
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
#[error(transparent)]
pub enum WorkspaceError {
    Init(#[from] InitError),
    Build(#[from] BuildError),
}

#[derive(Debug, Args)]
pub struct InitArgs {
    // If unspecified, use CWD.
    pub path: Option<PathBuf>,
}

#[derive(Error, Debug, Diagnostic)]
#[error(transparent)]
pub enum InitError {
    IO(#[from] std::io::Error),
    AlreadyExists(#[from] WorkspaceAlreadyExists),
    Canceled(#[from] ActionCanceled),
}

fn confirm(prompt: String) -> Result<bool, std::io::Error> {
    asky::Confirm::new(prompt.as_str()).prompt()
}

#[derive(Error, Debug, Diagnostic)]
#[error("Workspace already exists.")]
pub struct WorkspaceAlreadyExists;

#[derive(Error, Debug, Diagnostic)]
#[error("Canceled.")]
pub struct ActionCanceled;

pub const SOL_MANIFEST_FILENAME: &'static str = "index.sol";

pub fn init(path: PathBuf) -> Result<(), InitError> {
    let manifest_path = path.join(SOL_MANIFEST_FILENAME);

    if manifest_path.exists() {
        Err(WorkspaceAlreadyExists)?;
    }

    let mut dir = std::fs::read_dir(path.clone())?;

    let prompt = format!("Initialize workspace at {:?}?", path.display());
    if !confirm(prompt)? {
        Err(ActionCanceled)?;
    }

    let manifest = File::create_new(manifest_path)?;

    Ok(())
}

#[derive(Debug, Args)]
pub struct BuildArgs {
    // Override of the workspace path to be built.
    // Defaults to the CWD.
    pub path: Option<PathBuf>,

    /// Watches over the workspace and rebuilds assets as they change.
    ///
    /// This option is more efficient than calling build over and over,
    /// because the program will keep the parsed assets partially in memory.
    #[arg(short, long)]
    pub watch: bool,

    /// Log every little thing that happens into the stdout. This is useful for debugging.
    #[arg(short, long)]
    pub verbose: bool,
}

#[derive(Error, Debug, Diagnostic)]
#[error(transparent)]
pub enum BuildError {
    IO(#[from] std::io::Error),
}

pub fn build(path: PathBuf) -> Result<(), BuildError> {
    let mut dir = std::fs::read_dir(path)?;

    Ok(())
}
