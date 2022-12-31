//! The main starling entry point.

#![warn(clippy::pedantic)]

use std::{
    path::{Path, PathBuf},
    process::exit,
};

use clap::{Parser, Subcommand};

/// Automated concurrent algorithm proof checker (second edition).
#[derive(clap::Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

/// Subcommands for the `starling` CLI.
#[derive(Subcommand)]
enum Commands {
    /// Performs basic semantic analysis on a PVC script.
    Lint {
        /// Path to the PVC script to analyse.
        #[arg(required = true)]
        path: PathBuf,
    },
}

fn main() -> eyre::Result<()> {
    color_eyre::install()?;

    let args = Cli::parse();
    let (cur_path, err) = match args.command {
        Commands::Lint { path } => (path.to_string_lossy().into_owned(), lint(&path)),
    };

    match err {
        Err(Error::Parser(starling::parser::Error::Parse(e))) => {
            eprintln!("Parse error in file {cur_path}:");
            eprintln!("{e}");
            exit(1);
        }
        e => e,
    }?;

    Ok(())
}

fn lint(path: impl AsRef<Path>) -> Result<()> {
    let contents = std::fs::read_to_string(path)?;
    let ast = starling::parser::parse(&contents)?;

    println!("{ast:#?}");

    Ok(())
}

#[derive(thiserror::Error, Debug)]
enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Parser error: {0}")]
    Parser(#[from] starling::parser::Error),
}

type Result<T> = std::result::Result<T, Error>;
