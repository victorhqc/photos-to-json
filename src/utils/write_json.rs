use std::path::{Path};
use serde::Serialize;
use snafu::prelude::*;
use std::fs;

pub fn write_json<T>(data: T, path: Option<&Path>, should_prettify: bool) -> Result<(), Error> where T: Serialize {
    let json = if should_prettify {
        serde_json::to_string_pretty(&data).context(FailedToSerializeSnafu)?
    } else {
        serde_json::to_string(&data).context(FailedToSerializeSnafu)?
    };

    if path.is_none() {
        println!("{}", json);

        return Ok(());
    }

    let path = path.unwrap();

    // If it's a dir, we'll add the output automatically
    let path = if path.is_dir() {
        path.join("images_output.json")
    } else if let Some(extension) = path.extension() {
        // if it's not a dir, then it must have a JSON extension.
        if extension.to_str().context(BadPathSnafu)?.to_lowercase() != "json" {
            return Err(Error::BadPath);
        }

        path.to_path_buf()
    } else {
        return Err(Error::BadPath);
    };

    let path_str = path.to_str().unwrap().to_string();
    fs::write(path, json).context(FailedToWriteSnafu { target: path_str })?;

    Ok(())
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("The target must be a valid directory or a JSON file"))]
    BadPath,

    #[snafu(display("Failed to Serialize JSON: {}", source))]
    FailedToSerialize { source: serde_json::Error },

    #[snafu(display("Failed to write JSON file at {}: {}", target, source))]
    FailedToWrite { source: std::io::Error, target: String },
}
