
use serde::{Serialize};

pub(crate) const IMAGE_DIR_NAME: &'static str = "packer/images/";
pub(crate) const SAVE_DIR_NAME: &'static str = "packer/out/";

#[derive(Debug, Serialize)]
pub struct Rect {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}
