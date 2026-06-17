use clap::Parser;

fn main() {
    let args = html_effectiveness::cli::CliArgs::parse();
    if let Err(e) = html_effectiveness::compiler::run_compilation(&args) {
        eprintln!("Error: {e}");
        for cause in e.chain().skip(1) {
            eprintln!("  caused by: {cause}");
        }
        std::process::exit(1);
    }
}
