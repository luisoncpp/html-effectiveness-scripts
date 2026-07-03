use anyhow::Result;

use crate::cli::CliArgs;

pub fn run_compilation(args: &CliArgs) -> Result<()> {
    let markdown = std::fs::read_to_string(&args.input)?;
    let document = crate::compile(&markdown, &crate::CompileOptions::default())?;
    std::fs::write(&args.output, document.html)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::CliArgs;
    use std::path::PathBuf;

    #[test]
    fn writes_html_file_for_valid_markdown() {
        let tmp = std::env::temp_dir();
        let input_path = tmp.join("test_input.md");
        let output_path = tmp.join("test_output.html");

        std::fs::write(&input_path, "# Test\n\nHello world.").unwrap();
        let _ = std::fs::remove_file(&output_path);

        let args = CliArgs {
            input: input_path.clone(),
            output: output_path.clone(),
        };

        let result = run_compilation(&args);
        let _ = std::fs::remove_file(&input_path);

        assert!(result.is_ok());

        let output = std::fs::read_to_string(&output_path).unwrap();
        let _ = std::fs::remove_file(&output_path);

        assert!(output.contains("<h1>Test</h1>"));
        assert!(output.contains("<p>Hello world.</p>"));
    }

    #[test]
    fn returns_error_for_missing_input() {
        let args = CliArgs {
            input: PathBuf::from("nonexistent-file.md"),
            output: PathBuf::from("out.html"),
        };
        let result = run_compilation(&args);
        assert!(result.is_err());
    }
}
