mod commands;
mod models;
mod utils;

use clap::{Parser, Subcommand};
use std::path::Path;
use crate::commands::generate;
use snafu::prelude::*;
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

fn main() -> Result<(), Error> {
    let args = Cli::parse();

    match args.command {
        Commands::Generate { path, target, colors, should_prettify } => {
            let pb = ProgressBar::new_spinner();
            pb.enable_steady_tick(Duration::from_millis(80));
            pb.set_style(
                ProgressStyle::with_template("{spinner:.blue} {msg}")
                    .unwrap()
                    .tick_strings(&[
                        "⠋",
                        "⠙",
                        "⠹",
                        "⠸",
                        "⠼",
                        "⠴",
                        "⠦",
                        "⠧",
                        "⠇",
                        "⠏"
                    ]),
            );
            pb.set_message("Reading photos information...");
            let path = Path::new(&path);

            // Used to assign a value for Path, otherwise compiler doesn't know what to do.
            #[allow(unused_assignments)]
            let mut temp_t = String::from("");
            let target = if let Some(t) = target {
                temp_t = t.clone();
                Some(Path::new(&temp_t))
            } else {
                None
            };

            generate(path, target, colors, should_prettify).context(GenerateSnafu)?;
            pb.finish_with_message("Done");
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
    /// Generates a JSON with information about the images found in the given path
    #[command(arg_required_else_help = true)]
    Generate {
        /// Should prettify JSON result or not.
        #[arg(short = 'p', value_name = "PRETTIFY", default_value_t = false)]
        should_prettify: bool,
        /// Number of colors to extract from an image.
        #[arg(short = 'c', value_name = "COLORS", default_value_t = 3)]
        colors: u8,
        /// Target for the output JSON to be generated at, if empty the result will be passed to the stdout.
        #[arg(short = 't', value_name = "TARGET")]
        target: Option<String>,
        /// Source of the images to find.
        path: String,
    }
}

#[derive(Debug, Snafu)]
enum Error {
    #[snafu(display("Failed to generate: {}", source))]
    Generate { source: commands::Error }
}
