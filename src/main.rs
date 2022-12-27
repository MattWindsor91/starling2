#[warn(clippy::pedantic)]
mod language;
mod parser;

use std::path::Path;
use std::process::exit;

pub use pest::Parser;

fn main() -> eyre::Result<()> {
    color_eyre::install()?;

    let path = "example.sta";

    match run(path) {
        Err(Error::Parser(parser::Error::Parse(e))) => {
            eprintln!("Parse error in file {path}:");
            eprintln!("{e}");
            exit(1);
        }
        e => e,
    }?;

    Ok(())
}

fn run(path: impl AsRef<Path>) -> Result<()> {
    let contents = std::fs::read_to_string(path)?;
    let ast = parser::parse(&contents)?;

    println!("{ast:#?}");

    Ok(())
}

#[derive(thiserror::Error, Debug)]
enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Parser error: {0}")]
    Parser(#[from] parser::Error),
}

type Result<T> = std::result::Result<T, Error>;
