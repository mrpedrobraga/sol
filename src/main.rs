#![allow(unused)]
use clap::{CommandFactory, Parser, Subcommand};
use miette::{Diagnostic, IntoDiagnostic};
use sol_lang::workspace::{self, BuildArgs, InitArgs, WorkspaceError};
use std::path::PathBuf;
use thiserror::Error;

// Usage: `sol <arguments>`
fn main() -> miette::Result<()> {
    let args = CliArgs::parse();

    match args.command {
        PrimaryAction::Init(args) => {
            let path = args
                .path
                .unwrap_or(std::env::current_dir().into_diagnostic()?);
            workspace::init(path).map_err(WorkspaceError::Init)?;
        }
        PrimaryAction::Build(args) => {
            let dir = args
                .path
                .unwrap_or(std::env::current_dir().into_diagnostic()?);
            workspace::build(dir).map_err(WorkspaceError::Build)?;
        }
        PrimaryAction::Test => Err(Unimplemented)?,
        PrimaryAction::Clean => Err(Unimplemented)?,
        PrimaryAction::Migrate => Err(Unimplemented)?,
        PrimaryAction::Add => Err(Unimplemented)?,
        PrimaryAction::Remove => Err(Unimplemented)?,
        PrimaryAction::Completions { shell } => {
            shell.generate(&mut CliArgs::command(), &mut std::io::stdout())
        }
    }

    Ok(())
}

#[derive(Debug, clap::Parser)]
#[command(
    name = "sol",
    about = "Sol parser, compiler and package manager.",
    version
)]
struct CliArgs {
    /// What action to execute on the workspace.
    #[command(subcommand)]
    command: PrimaryAction,
}

#[derive(Debug, Subcommand)]
enum PrimaryAction {
    /// Initialize a new workspace on a directory.
    Init(InitArgs),
    /// Build an existing workspace's assets, so that they can be imported in an app.
    Build(BuildArgs),
    /// Runs the internal tests availabe in the library.
    Test,
    /// Cleans the build artifacts generated in a build, freeing memory.
    Clean,

    /// Applies a migration file to your repository, transforming your assets if necessary.
    Migrate,

    /// Adds a new library dependency to the workspace.
    Add,
    /// Removes a previously added dependency from the workspace.
    Remove,

    /// Query completions for the given shell.
    Completions { shell: clap_complete_command::Shell },
}

#[derive(Error, Debug, Diagnostic)]
#[error(transparent)]
pub enum CliError {
    IO(#[from] std::io::Error),
    Workspace(#[from] WorkspaceError),
    Unimplemented(Unimplemented),
}

#[derive(Error, Debug, Diagnostic)]
#[error("This feature is not yet implemented.")]
pub struct Unimplemented;

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    CliArgs::command().debug_assert()
}
