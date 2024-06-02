#![allow(unused)]
use miette::{highlighters::Highlighter, Diagnostic, NamedSource, Result, SourceSpan};
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
#[error("Language.")]
#[diagnostic(
    severity(Error),
    code("standard::nico::error"),
    help("Try washing your mouth with soap.")
)]
pub struct OopsieDoopsie {
    #[source_code]
    pub src: NamedSource<String>,

    #[label("In this sentence.")]
    pub at: SourceSpan,

    #[label(collection, "Because of this directive")]
    pub disallow_span: Vec<SourceSpan>,
}

fn main() -> Result<()> {
    miette::set_hook(Box::new(|_| {
        Box::new(
            miette::MietteHandlerOpts::new()
                .terminal_links(true)
                .unicode(true)
                .context_lines(3)
                .build(),
        )
    }));

    let source = include_str!("./faulty.sol").to_owned();

    Err(OopsieDoopsie {
        src: NamedSource::new("./faulty.sol", source),
        at: (30, 4).into(),
        disallow_span: vec![(0, 23).into(), (25, 3).into()],
    })?;

    Ok(())
}
