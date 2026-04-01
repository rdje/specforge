use clap::Parser;

fn main() {
    let cli = specforge::cli::Cli::parse();

    if let Err(error) = specforge::run(cli) {
        eprintln!("error: {error}");
        std::process::exit(1);
    }
}
