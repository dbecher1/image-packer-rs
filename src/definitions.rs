use image::DynamicImage;
use serde::{Deserialize, Serialize};

pub(crate) const IMAGE_DIR_NAME: &'static str = "images/";

#[derive(Debug, Serialize)]
pub struct Rect {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

#[derive(Serialize)]
pub(crate) struct ImgWrapper {
    pub(crate) image: DynamicImage,
    pub(crate) name: String,
}
