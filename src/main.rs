use clap::Parser;

fn main() {
    let args = html_effectiveness::cli::CliArgs::parse();
    if let Err(e) = html_effectiveness::compiler::run_compilation(&args) {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}
