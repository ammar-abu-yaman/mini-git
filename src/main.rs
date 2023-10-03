use argparse::Cli;
use clap::Parser;

pub mod argparse;
pub mod handler;
pub mod model;

use handler::CommandHandler;

fn main() {
    let cli = Cli::parse();
    let handler = CommandHandler::new();
    handler.handle(cli);
}
