#[deny(clippy::pedantic)]
mod language;
mod parser;

pub use pest::Parser;

fn main() -> eyre::Result<()> {
    color_eyre::install()?;

    let contents = std::fs::read_to_string("example.sta")?;
    let ast = parser::parse(&contents)?;

    println!("{ast:#?}");

    Ok(())
}
