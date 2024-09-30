use std::path::PathBuf;
use clap::Parser;
use anyhow::Result;

#[derive(Parser)]
#[command(name = "Bro - BRacket fOrmatter")]
#[command(version = "0.1")]
#[command(about = "A really dumb code prettifier")]
struct Cli {
    #[arg(long, short, default_value_t = false, group = "output_type")]
    in_place: bool,

    #[arg(long, short, group = "output_type")]
    out_filename: Option<PathBuf>,

    #[arg(long, default_value_t = 2)]
    indent: usize,

    filename: PathBuf,
}

fn format(source: &str, indentation: usize) -> String {
    let mut result = String::new();
    let mut current_indentation = 0;
    let mut ignore_whitespace = true;
    for char in source.chars() {
        if ignore_whitespace {
            if char.is_whitespace() {
                continue;
            } else {
                ignore_whitespace = false;
            }
        }
        match char {
            '(' | '[' | '{' | '<' => {
                result.push(char);
                result.push('\n');
                current_indentation += indentation;
                result += &" ".repeat(current_indentation);
                ignore_whitespace = true;
            }
            ')' | ']' | '}' | '>' => {
                result.push('\n');
                current_indentation = current_indentation.saturating_sub(indentation);
                result += &" ".repeat(current_indentation);
                result.push(char);
                ignore_whitespace = true;
            }
            ',' => {
                result.push(char);
                result.push('\n');
                result += &" ".repeat(current_indentation);
                ignore_whitespace = true;
            }
            _ => {
                result.push(char);
            }
        }
    }
    result.trim().to_string()
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let source = std::fs::read_to_string(&cli.filename)?;
    let formatted= format(&source, cli.indent);
    if cli.in_place {
        std::fs::write(&cli.filename, formatted)?;
    } else if let Some(out_filename) = cli.out_filename {
        std::fs::write(out_filename, formatted)?;
    } else {
        println!("{formatted}");
    }
    Ok(())
}
