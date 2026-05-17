use html_effectiveness::cli::CliArgs;
use html_effectiveness::compiler;
use std::path::PathBuf;

#[test]
fn snapshot_basic_markdown() {
    let input = PathBuf::from("tests/fixtures/basic.md");
    let output = PathBuf::from("tests/fixtures/basic_snapshot.html");

    let args = CliArgs {
        input,
        output: output.clone(),
    };

    compiler::run_compilation(&args).unwrap();
    let html = std::fs::read_to_string(&output).unwrap();
    let _ = std::fs::remove_file(&output);

    insta::assert_snapshot!(html);
}

#[test]
fn snapshot_hybrid_markdown() {
    let input = PathBuf::from("tests/fixtures/hybrid.md");
    let output = PathBuf::from("tests/fixtures/hybrid_snapshot.html");

    let args = CliArgs {
        input,
        output: output.clone(),
    };

    compiler::run_compilation(&args).unwrap();
    let html = std::fs::read_to_string(&output).unwrap();
    let _ = std::fs::remove_file(&output);

    insta::assert_snapshot!(html);
}

#[test]
fn snapshot_triage_board() {
    let input = PathBuf::from("tests/fixtures/triage_board.md");
    let output = PathBuf::from("tests/fixtures/triage_board_snapshot.html");

    let args = CliArgs {
        input,
        output: output.clone(),
    };

    compiler::run_compilation(&args).unwrap();
    let html = std::fs::read_to_string(&output).unwrap();
    let _ = std::fs::remove_file(&output);

    insta::assert_snapshot!(html);
}
