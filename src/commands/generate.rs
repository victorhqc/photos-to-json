use std::path::Path;
use snafu::prelude::*;
use path_absolutize::*;
use walkdir::WalkDir;
use crate::models::image::Image;

pub fn generate(path: & Path, target: & Path) -> Result<(), Error> {
    let absolute_path = path.absolutize().context(AbsolutePathSnafu)?;
    println!("absolute path: {}", absolute_path.to_str().context(PathToStrSnafu)?);
    let absolute_target = target.absolutize().context(AbsolutePathSnafu)?;
    println!("absolute target: {}", absolute_target.to_str().context(PathToStrSnafu)?);

    println!("generate from '{}' to '{}'", path.to_str().context(PathToStrSnafu)?, target.to_str().context(PathToStrSnafu)?);


    let results: Vec<Image> = WalkDir::new(path).into_iter().map(|entry| {
        let entry = entry.context(BadEntrySnafu)?;
        let path = entry.path();

        Ok(Image::build(path).context(BadImageSnafu)?)
    }).filter_map(|e: Result<Image, Error>| e.ok())
        .collect();

    println!("Results: {:?}", results);

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
}
