use std::path::{Path};
use snafu::prelude::*;
use std::str::FromStr;
use color_thief::{Color as ColorThief, ColorFormat};
use serde::ser::{Serialize as SerializeTrait, Serializer, SerializeStruct};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Image {
    path: String,
    file_name: String,
    kind: ImageKind,
    width: u32,
    height: u32,
    colors: Vec<Color>,
}

impl Image {
    pub fn build(path: &Path, colors: u8) -> Result<Self, Error> {
        let extension = path
            .extension()
            .context(MissingExtensionSnafu { entry: path })?
            .to_str().context(MissingExtensionSnafu { entry: path })?;

        let file_name = path.file_name().context(MissingFilenameSnafu { entry: path})?.to_str().context(MissingFilenameSnafu { entry: path })?;

        let kind = ImageKind::from_str(extension)?;

        let img = image::open(path).context(FailedToOpenSnafu)?;
        let height = img.height();
        let width = img.width();

        let (buffer, color_type) = get_image_buffer(img);
        let colors = color_thief::get_palette(&buffer, color_type, 10, colors).context(BadPaletteSnafu)?.into_iter().map(|c| Color(c)).collect();

        Ok(Self {
            path: String::from(path.to_str().context(InvalidPathSnafu { entry: path })?),
            file_name: String::from(file_name),
            kind,
            colors,
            height,
            width,
        })
    }
}

fn get_image_buffer(img: image::DynamicImage) -> (Vec<u8>, ColorFormat) {
    match img {
        image::DynamicImage::ImageRgb8(buffer) => {
            (buffer.to_vec(), color_thief::ColorFormat::Rgb)
        }
        image::DynamicImage::ImageRgba8(buffer) => {
            (buffer.to_vec(), color_thief::ColorFormat::Rgba)
        }
        _ => unreachable!(),
    }
}

#[derive(Debug, Serialize)]
pub enum ImageKind {
    Jpeg,
    Png,
}

#[derive(Debug)]
pub struct Color(ColorThief);

impl SerializeTrait for Color {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let mut state = serializer.serialize_struct("Color", 3)?;

        state.serialize_field("r", &self.0.r)?;
        state.serialize_field("g", &self.0.g)?;
        state.serialize_field("b", &self.0.b)?;

        state.end()
    }
}

impl FromStr for ImageKind {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "jpg" => Ok(ImageKind::Jpeg),
            "jpeg" => Ok(ImageKind::Jpeg),
            "png" => Ok(ImageKind::Png),
            _ => Err(Error::InvalidExtension { ext: String::from(s) })
        }
    }
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Invalid path: {:?}", entry))]
    InvalidPath { entry: Box<Path> },

    #[snafu(display("Missing extension for: {:?}", entry))]
    MissingExtension { entry: Box<Path> },

    #[snafu(display("Missing filename for: {:?}", entry))]
    MissingFilename { entry: Box<Path> },

    #[snafu(display("Extension '{}' is invalid", ext))]
    InvalidExtension { ext: String },

    #[snafu(display("Failed to open image: {}", source))]
    FailedToOpen { source: image::error::ImageError },

    #[snafu(display("Could not get color palette: {}", source))]
    BadPalette { source: color_thief::Error },
}
