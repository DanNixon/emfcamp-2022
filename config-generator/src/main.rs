mod generator;
mod schedule;

use crate::schedule::Schedule;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    List,
    GenerateRubricContent,
    GeneratePrinterContent,
}

fn main() {
    let cli = Cli::parse();

    let schedule = Schedule::load().unwrap();

    match cli.command {
        Command::List => {
            for event in schedule.events {
                println!("{}", event);
            }
        }
        Command::GenerateRubricContent => {
            for msg in generator::dapnet_rubric::generate(&schedule) {
                println!("{}", msg);
            }
        }
        Command::GeneratePrinterContent => {
            for msg in generator::printer::generate(&schedule) {
                println!("{}", msg);
            }
        }
    }
}
