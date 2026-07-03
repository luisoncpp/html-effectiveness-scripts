use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::Parser;
use mdyaml2html::{CompileOptions, compile};

#[derive(Parser, Debug)]
struct CliArgs {
    #[arg(short = 'i', long = "input")]
    input: PathBuf,

    #[arg(short = 'o', long = "output")]
    output: PathBuf,
}

fn main() {
    if let Err(error) = run(CliArgs::parse()) {
        eprintln!("Error: {error:#}");
        std::process::exit(1);
    }
}

fn run(args: CliArgs) -> Result<()> {
    let source = std::fs::read_to_string(&args.input)
        .with_context(|| format!("failed to read {}", args.input.display()))?;
    let document = compile(&source, &CompileOptions::default())?;
    std::fs::write(&args.output, document.html)
        .with_context(|| format!("failed to write {}", args.output.display()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_input_and_output_paths() {
        let args = CliArgs::try_parse_from([
            "mdyaml2html",
            "--input",
            "input.md",
            "--output",
            "output.html",
        ])
        .unwrap();

        assert_eq!(args.input, PathBuf::from("input.md"));
        assert_eq!(args.output, PathBuf::from("output.html"));
    }
}
