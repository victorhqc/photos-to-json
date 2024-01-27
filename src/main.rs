mod commands;
mod models;

use clap::{Parser, Subcommand};
use std::path::Path;
use crate::commands::generate;
use snafu::prelude::*;

fn main() -> Result<(), Error> {
    let args = Cli::parse();

    match args.command {
        Commands::Generate { path, target } => {
            let path = Path::new(&path);
            let target = Path::new(&target);
            generate(path, target).context(GenerateSnafu)?;
        }
    }

    Ok(())
}

#[derive(Parser, Debug)]
#[command(name = "photos-to-json")]
#[command(about = "Reads a directory with images and outputs a JSON with their information", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[command(arg_required_else_help = true)]
    Generate {
        path: String,
        target: String
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {

}

#[derive(Debug, Snafu)]
enum Error {
    #[snafu(display("Failed to generate: {}", source))]
    Generate { source: commands::Error }
}
