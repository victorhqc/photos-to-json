use std::path::Path;
use snafu::prelude::*;
use path_absolutize::*;
use walkdir::WalkDir;
use crate::models::image::Image;
use crate::utils::{write_json, Error as WriteJsonError};

pub fn generate(path: & Path, target: Option<&Path>, colors: u8, should_prettify: bool) -> Result<(), Error> {
    let absolute_path = path.absolutize().context(AbsolutePathSnafu)?;

    let results: Vec<Image> = WalkDir::new(absolute_path).into_iter().map(|entry| {
        let entry = entry.context(BadEntrySnafu)?;
        let path = entry.path();

        Ok(Image::build(path, colors).context(BadImageSnafu)?)
    }).filter_map(|e: Result<Image, Error>| e.ok())
        .collect();

    write_json(results, target, should_prettify).context(WriteJsonFileSnafu)?;

    Ok(())
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Could not parse path to string"))]
    PathToStr,

    #[snafu(display("Failed to get absolute path: {}", source))]
    AbsolutePath { source: std::io::Error },

    #[snafu(display("Problem while walking through directory: {}", source))]
    BadEntry { source: walkdir::Error },

    #[snafu(display("Could not build an image: {}", source))]
    BadImage { source: crate::models::image::Error },

    #[snafu(display("Failed to write JSON file: {}", source))]
    WriteJsonFile { source: WriteJsonError },
}
