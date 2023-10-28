use image::DynamicImage;

pub(crate) const IMAGE_DIR_NAME: &'static str = "images/";

#[derive(Debug)]
pub struct Rect {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

pub(crate) struct ImgWrapper {
    pub(crate) image: DynamicImage,
    pub(crate) name: String,
}
