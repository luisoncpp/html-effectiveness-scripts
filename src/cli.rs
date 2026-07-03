use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct CliArgs {
    #[arg(short = 'i', long = "input")]
    pub input: PathBuf,

    #[arg(short = 'o', long = "output")]
    pub output: PathBuf,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_input_output_flags() {
        let args = CliArgs::try_parse_from(["prog", "-i", "in.md", "-o", "out.html"]);
        assert!(args.is_ok());
        let args = args.unwrap();
        assert_eq!(args.input, PathBuf::from("in.md"));
        assert_eq!(args.output, PathBuf::from("out.html"));
    }

    #[test]
    fn parses_long_form_flags() {
        let args =
            CliArgs::try_parse_from(["prog", "--input", "test.md", "--output", "result.html"]);
        assert!(args.is_ok());
        let args = args.unwrap();
        assert_eq!(args.input, PathBuf::from("test.md"));
        assert_eq!(args.output, PathBuf::from("result.html"));
    }

    #[test]
    fn rejects_missing_input() {
        let args = CliArgs::try_parse_from(["prog", "-o", "out.html"]);
        assert!(args.is_err());
    }

    #[test]
    fn rejects_missing_output() {
        let args = CliArgs::try_parse_from(["prog", "-i", "in.md"]);
        assert!(args.is_err());
    }

    #[test]
    fn rejects_unknown_flags() {
        let args = CliArgs::try_parse_from(["prog", "-i", "in.md", "-o", "out.html", "--verbose"]);
        assert!(args.is_err());
    }
}
