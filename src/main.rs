mod error;
mod config;
mod cli;
mod site;
mod build;

fn main() -> error::Result<()> {
    let cli = cli::Cli::parse();
    cli.run()
}