#![allow(unused)]
use clap::{Args, Parser, Subcommand};
use polib::{catalog, po_file};
use sol_lang::{
    parser::nom::p_script,
    unparser::{print_script, render_script},
};
use std::path::{Path, PathBuf};

mod extract;
mod replace;

fn main() {
    let args = CliArgs::parse();

    match args.command {
        PrimaryAction::Translate(translate_args) => translate(
            translate_args.from,
            translate_args.translation,
            translate_args.to,
        ),
    }
}

fn translate<PSource, PTranslation, PTarget>(
    source_path: PSource,
    translation_path: PTranslation,
    target_path: PTarget,
) where
    PSource: AsRef<Path>,
    PTranslation: AsRef<Path>,
    PTarget: AsRef<Path>,
{
    let raw = std::fs::read_to_string(&source_path).expect("Error reading source `.sol` file!");
    let (_, script) = p_script(&raw).expect("Failed to parse");
    let catalog =
        po_file::parse(translation_path.as_ref()).expect("Error reading translation `.po` file!");
    let translated = replace::r_script(&script, &catalog);
    let translated_raw = render_script(&translated);
    std::fs::write(
        target_path.as_ref().with_file_name(format!(
            "{}-{}",
            source_path.as_ref().file_stem().expect("Source path was not a file...").to_string_lossy(),
            catalog.metadata.language
        )).with_extension("sol"),
        translated_raw,
    );
}

fn generate_template<PSource, PTarget>(source: PSource, target: PTarget)
where
    PSource: AsRef<Path>,
    PTarget: AsRef<Path>,
{
    let raw = std::fs::read_to_string(&source).expect("Error reading subject file!");
    let source_filename = source
        .as_ref()
        .file_name()
        .expect("Source path wasn't a file.")
        .to_string_lossy();
    let (_, script) = p_script(&raw).expect("Failed to parse");

    let (template, source) = extract::x_script(&script);
    po_file::write(
        &template,
        PathBuf::from(target.as_ref())
            .with_extension("pot")
            .as_path(),
    )
    .expect("Failed saving template!");
    po_file::write(
        &source,
        Path::new(
            PathBuf::from(target.as_ref())
                .with_file_name(format!("{}-template", source_filename))
                .with_extension("po")
                .as_path(),
        ),
    )
    .expect("Failed saving source!");
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
    Translate(TranslateArgs),
}

#[derive(Debug, Args)]
struct TranslateArgs {
    #[arg(long)]
    from: Box<Path>,
    #[arg(short, long)]
    translation: Box<Path>,
    #[arg(long)]
    to: Box<Path>,
}
