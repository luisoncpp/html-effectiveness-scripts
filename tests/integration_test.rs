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

#[test]
fn render_no_external_links() {
    let input = PathBuf::from("tests/fixtures/hybrid.md");
    let output = PathBuf::from("tests/fixtures/external_test.html");

    let args = CliArgs {
        input,
        output: output.clone(),
    };

    compiler::run_compilation(&args).unwrap();
    let html = std::fs::read_to_string(&output).unwrap();
    let _ = std::fs::remove_file(&output);

    assert!(!html.contains(r#"href="*.css""#));
    assert!(!html.contains(r#"src="*.js""#));
    assert!(!html.contains(r#".css""#));
    assert!(!html.contains(r#".js""#));
}

#[test]
fn render_inline_styles_present() {
    let input = PathBuf::from("tests/fixtures/hybrid.md");
    let output = PathBuf::from("tests/fixtures/inline_styles_test.html");

    let args = CliArgs {
        input,
        output: output.clone(),
    };

    compiler::run_compilation(&args).unwrap();
    let html = std::fs::read_to_string(&output).unwrap();
    let _ = std::fs::remove_file(&output);

    assert!(html.contains("<style>"));
    assert!(html.contains(".prompt-box"));
}

#[test]
fn notice_markdown_renders_component_and_no_raw_yaml() {
    let input = PathBuf::from("tests/fixtures/notice.md");
    let output = PathBuf::from("tests/fixtures/notice_output.html");

    let args = CliArgs {
        input,
        output: output.clone(),
    };

    let result = compiler::run_compilation(&args);
    assert!(result.is_ok());

    let html = std::fs::read_to_string(&output).unwrap();
    let _ = std::fs::remove_file(&output);

    assert!(html.contains("notice"));
    assert!(html.contains("notice--warning"));
    assert!(html.contains("Breaking Change"));
    assert!(!html.contains("type: notice"));
    assert!(!html.contains("variant: warning"));
    assert!(!html.contains("icon: alert-triangle"));
}

#[test]
fn card_markdown_renders_component_and_nested_child() {
    let input = PathBuf::from("tests/fixtures/card.md");
    let output = PathBuf::from("tests/fixtures/card_output.html");

    let args = CliArgs {
        input,
        output: output.clone(),
    };

    let result = compiler::run_compilation(&args);
    assert!(result.is_ok());

    let html = std::fs::read_to_string(&output).unwrap();
    let _ = std::fs::remove_file(&output);

    assert!(html.contains("card"));
    assert!(html.contains("card--elevation-2"));
    assert!(html.contains("Card Title"));
    assert!(html.contains("rust"));
    assert!(html.contains("urgent"));
    assert!(html.contains("Some content inside the card."));
    assert!(html.contains("Nested info"));
    assert!(!html.contains("type: card"));
    assert!(!html.contains("elevation: 2"));
}

#[test]
fn data_grid_markdown_renders_component_and_no_raw_yaml() {
    let input = PathBuf::from("tests/fixtures/data_grid.md");
    let output = PathBuf::from("tests/fixtures/data_grid_output.html");

    let args = CliArgs {
        input,
        output: output.clone(),
    };

    let result = compiler::run_compilation(&args);
    assert!(result.is_ok());

    let html = std::fs::read_to_string(&output).unwrap();
    let _ = std::fs::remove_file(&output);

    assert!(html.contains("<table class=\"data-grid\">"));
    assert!(html.contains("<th>Feature</th>"));
    assert!(html.contains("<th>Status</th>"));
    assert!(html.contains("<th>Risk</th>"));
    assert!(html.contains("<td>AST Traversal</td>"));
    assert!(html.contains("<td>Shipped</td>"));
    assert!(html.contains("<td>Low</td>"));
    assert!(html.contains("<td>Drag & Drop</td>"));
    assert!(html.contains("<td>WIP</td>"));
    assert!(html.contains("<td>High</td>"));
    assert!(html.contains("<td>Minijinja Templating</td>"));
    assert!(!html.contains("type: data-grid"));
    assert!(!html.contains("columns:"));
    assert!(!html.contains("rows:"));
}

#[test]
fn timeline_markdown_renders_component_and_no_raw_yaml() {
    let input = PathBuf::from("tests/fixtures/timeline.md");
    let output = PathBuf::from("tests/fixtures/timeline_output.html");

    let args = CliArgs {
        input,
        output: output.clone(),
    };

    let result = compiler::run_compilation(&args);
    assert!(result.is_ok());

    let html = std::fs::read_to_string(&output).unwrap();
    let _ = std::fs::remove_file(&output);

    assert!(html.contains("timeline"));
    assert!(html.contains("timeline--vertical"));
    assert!(html.contains("Initial Outage"));
    assert!(html.contains("2026-05-18 10:00"));
    assert!(html.contains("Rolled back to v1.2"));
    assert!(html.contains("2026-05-18 10:15"));
    assert!(html.contains("Monitoring restored"));
    assert!(html.contains("2026-05-18 10:30"));
    assert!(!html.contains("type: timeline"));
    assert!(!html.contains("orientation: vertical"));
    assert!(!html.contains("steps:"));
}

#[test]
fn board_layout_markdown_renders_component_and_no_raw_yaml() {
    let input = PathBuf::from("tests/fixtures/board_layout.md");
    let output = PathBuf::from("tests/fixtures/board_layout_output.html");

    let args = CliArgs {
        input,
        output: output.clone(),
    };

    let result = compiler::run_compilation(&args);
    assert!(result.is_ok());

    let html = std::fs::read_to_string(&output).unwrap();
    let _ = std::fs::remove_file(&output);

    assert!(html.contains("board-layout"));
    assert!(html.contains("board-layout--kanban"));
    assert!(html.contains("To Do"));
    assert!(html.contains("Done"));
    assert!(html.contains("Task A"));
    assert!(html.contains("Task B"));
    assert!(html.contains("Task C"));
    assert!(!html.contains("type: board-layout"));
    assert!(!html.contains("variant: kanban"));
    assert!(!html.contains("items:"));
}

#[test]
fn svg_canvas_markdown_renders_component_and_no_raw_yaml() {
    let input = PathBuf::from("tests/fixtures/svg_canvas.md");
    let output = PathBuf::from("tests/fixtures/svg_canvas_output.html");

    let args = CliArgs {
        input,
        output: output.clone(),
    };

    let result = compiler::run_compilation(&args);
    assert!(result.is_ok());

    let html = std::fs::read_to_string(&output).unwrap();
    let _ = std::fs::remove_file(&output);

    assert!(html.contains("svg-canvas"));
    assert!(html.contains("<rect"));
    assert!(html.contains("<circle"));
    assert!(html.contains("<text"));
    assert!(!html.contains("type: svg-canvas"));
    assert!(!html.contains("elements:"));
}

#[test]
fn code_panel_markdown_renders_component_and_no_raw_yaml() {
    let input = PathBuf::from("tests/fixtures/code_panel.md");
    let output = PathBuf::from("tests/fixtures/code_panel_output.html");

    let args = CliArgs {
        input,
        output: output.clone(),
    };

    let result = compiler::run_compilation(&args);
    assert!(result.is_ok());

    let html = std::fs::read_to_string(&output).unwrap();
    let _ = std::fs::remove_file(&output);

    assert!(html.contains("code-panel"));
    assert!(html.contains("code-panel__tabs"));
    assert!(html.contains("code-panel__content"));
    assert!(html.contains("src/compiler.rs"));
    assert!(html.contains("Cargo.toml"));
    assert!(html.contains("parse_single"));
    assert!(html.contains("parse_blocks"));
    assert!(html.contains("pulldown-cmark"));
    assert!(!html.contains("type: code-panel"));
    assert!(!html.contains("tabs:"));
}

#[test]
fn all_primitives_markdown_renders_every_component() {
    let input = PathBuf::from("tests/fixtures/all_primitives.md");
    let output = PathBuf::from("tests/fixtures/all_primitives_output.html");

    let args = CliArgs {
        input,
        output: output.clone(),
    };

    let result = compiler::run_compilation(&args);
    assert!(result.is_ok());

    let html = std::fs::read_to_string(&output).unwrap();
    let _ = std::fs::remove_file(&output);

    // Frontmatter
    assert!(html.contains("<title>All Primitives Demo</title>"));

    // Notice
    assert!(html.contains("notice--warning"));
    assert!(html.contains("Breaking Change"));

    // Card
    assert!(html.contains("card--elevation-2"));
    assert!(html.contains("Feature Card"));
    assert!(html.contains("Nested notice inside a card."));

    // Data Grid
    assert!(html.contains("<table class=\"data-grid\">"));
    assert!(html.contains("<th>Feature</th>"));
    assert!(html.contains("<td>AST Traversal</td>"));

    // Timeline
    assert!(html.contains("timeline--vertical"));
    assert!(html.contains("Schema & API contract"));
    assert!(html.contains("Ship to beta"));

    // Board Layout
    assert!(html.contains("board-layout--kanban"));
    assert!(html.contains("To Do"));
    assert!(html.contains("Done"));

    // Code Panel
    assert!(html.contains("code-panel"));
    assert!(html.contains("src/compiler.rs"));
    assert!(html.contains("Cargo.toml"));

    // SVG Canvas
    assert!(html.contains("svg-canvas"));
    assert!(html.contains("<rect"));
    assert!(html.contains("<circle"));

    // Flowchart
    assert!(html.contains("flowchart"));
    assert!(html.contains("git push main"));

    // Module Map
    assert!(html.contains("module-map"));
    assert!(html.contains("parser.rs"));

    // Triage Board
    assert!(html.contains("Cycle 15 triage"));

    // No raw YAML anywhere
    assert!(!html.contains("type: notice"));
    assert!(!html.contains("type: card"));
    assert!(!html.contains("type: data-grid"));
    assert!(!html.contains("type: timeline"));
    assert!(!html.contains("type: board-layout"));
    assert!(!html.contains("type: code-panel"));
    assert!(!html.contains("type: svg-canvas"));
    assert!(!html.contains("type: flowchart"));
    assert!(!html.contains("type: module-map"));
    assert!(!html.contains("type: triage-board"));
}
