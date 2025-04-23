use derive_builder::Builder;
use std::cmp::min;
#[cfg(feature = "images")]
use std::path::Path;

use crate::error::Result;
use crate::shape::{Color, Shape};
use image;

/// Image shape. Can be created from any file, [`image`] crate can parse. Supports transparency
#[derive(Clone, Builder)]
pub struct Image {
    #[builder(setter(custom = true))]
    image: image::RgbaImage,
    #[builder(default = "255")]
    alpha: u8,
}

impl Image {
    pub fn builder() -> ImageBuilder {
        ImageBuilder::default()
    }
    /// Create [`Image`] from file
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self> {
        Ok(Self {
            image: image::open(path)?.to_rgba(),
            alpha: 0xff,
        })
    }

    /// Create [`Image`] from in-memory buffer
    pub fn from_buffer(buffer: &[u8]) -> Result<Self> {
        Ok(Self {
            image: image::load_from_memory(buffer)?.to_rgba(),
            alpha: 0xff,
        })
    }

    pub fn alpha(&mut self, min_alpha: u8) -> &mut Self
    {
        self.alpha = min_alpha;
        self
    }
}

impl ImageBuilder
{
    pub fn from_path<P: AsRef<Path>>(&mut self, path: P) -> &mut Self
    {
        self.image = Some(image::open(path).unwrap().to_rgba());
        self
    }
    pub fn from_buffer(&mut self, buffer: &[u8]) -> &mut Self
    {
        self.image = Some(image::load_from_memory(buffer).unwrap().to_rgba());
        self
    }
}

impl Shape for Image {
    fn render(&self) -> Vec<Vec<Option<Color>>> {
        self.image
            .rows()
            .map(|row| {
                row.map(|rgba| {
                    let [r, g, b, a] = rgba.0;
                    if a == 0 {
                        None
                    } else {
                        let new_alpha = min(a, self.alpha);
                        Some((r, g, b, new_alpha).into())
                    }
                })
                .collect()
            })
            .collect()
    }
}
