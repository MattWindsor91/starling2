mod language;
mod parser;
mod typing;

pub use pest::Parser;

fn main() -> eyre::Result<()> {
    color_eyre::install()?;

    let contents = std::fs::read_to_string("example.sta")?;
    let ast = parser::Parser::parse(parser::Rule::document, &contents)?;

    Ok(())
}
