use clap::Parser;

fn main() {
    let cli = spec2fsm::cli::Cli::parse();

    if let Err(error) = spec2fsm::run(cli) {
        eprintln!("error: {error}");
        std::process::exit(1);
    }
}
