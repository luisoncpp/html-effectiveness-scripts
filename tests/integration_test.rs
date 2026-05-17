use html_effectiveness::cli::CliArgs;
use html_effectiveness::compiler;

use std::path::PathBuf;

#[test]
fn basic_markdown_produces_html_with_correct_tags() {
    let input = PathBuf::from("tests/fixtures/basic.md");
    let output = PathBuf::from("tests/fixtures/basic_output.html");

    let args = CliArgs {
        input,
        output: output.clone(),
    };

    let result = compiler::run_compilation(&args);
    assert!(result.is_ok());

    let html = std::fs::read_to_string(&output).unwrap();
    let _ = std::fs::remove_file(&output);

    assert!(html.contains("<h1>Hello World</h1>"));
    assert!(html.contains("<p>This is a basic test paragraph.</p>"));
    assert!(html.contains("<h2>Section Two</h2>"));
    assert!(html.contains("<strong>bold</strong>"));
    assert!(html.contains("<em>italic</em>"));
    assert!(html.contains("<ul>"));
    assert!(html.contains("<li>List item one</li>"));
}

#[test]
fn hybrid_markdown_swallows_yaml_and_renders_rest() {
    let input = PathBuf::from("tests/fixtures/hybrid.md");
    let output = PathBuf::from("tests/fixtures/hybrid_output.html");

    let args = CliArgs {
        input,
        output: output.clone(),
    };

    let result = compiler::run_compilation(&args);
    assert!(result.is_ok());

    let html = std::fs::read_to_string(&output).unwrap();
    let _ = std::fs::remove_file(&output);

    assert!(html.contains("<h1>Hello World</h1>"));
    assert!(html.contains("<p>This is a basic test paragraph.</p>"));
    assert!(html.contains("<h2>Section Two</h2>"));
    assert!(!html.contains("<code"));
    assert!(!html.contains("type: prompt-box"));
    assert!(!html.contains("label: My Prompt"));
}
