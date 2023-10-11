use image::DynamicImage;
use serde::Deserialize;

pub(crate) const IMAGE_DIR_NAME: &'static str = "resources/gfx/";
pub(crate) const ANIMATION_DIR_NAME: &'static str = "resources/animations/";

#[derive(Deserialize)]
pub(crate) struct AnimationDataFile {
    pub(crate) animation: Vec<AnimationDataTOMLEntry>,
}

#[derive(Deserialize)]
pub(crate) struct AnimationDataTOMLEntry {
    // confusing but we need this to make file reading easy
    pub(crate) name: String,
    pub(crate) speed: u32,
    pub(crate) columns: u32,
    pub(crate) rows: u32,
}

#[derive(Debug)]
pub(crate) struct AnimationData {
    pub(crate) speed: u32,
    pub(crate) columns: u32,
    pub(crate) rows: u32,
}

#[derive(Debug)]
pub struct Rect {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

#[derive(Debug)]
pub(crate) struct Animation {
    pub data: Vec<Rect>,
    pub speed: u32,
}

pub(crate) struct ImgWrapper {
    pub(crate) image: DynamicImage,
    pub(crate) name: String,
}
