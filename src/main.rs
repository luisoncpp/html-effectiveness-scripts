use clap::Parser;

fn main() {
    let args = mdyaml2html::cli::CliArgs::parse();
    if let Err(e) = mdyaml2html::compiler::run_compilation(&args) {
        eprintln!("Error: {e}");
        for cause in e.chain().skip(1) {
            eprintln!("  caused by: {cause}");
        }
        std::process::exit(1);
    }
}
