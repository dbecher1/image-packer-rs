
// Loads a directory of images_vec recursively and packs it
// Algorithm taken from a blog post by David Colson
// https://www.david-colson.com/2020/03/10/exploring-rect-packing.html
// this is my third time making this sort of implementation in my 3rd language
// the short of it is this flow:
// create a packer struct with new()
// this will set up all the main variables and etc
// things can be changed before read_files() is called
// read_files will do the magic

// this is the flow:
// reads files from gfx
// any file in root gets read right into source_rects
// source_rects is a map, where the key is the file path + name
// within subdirectories, everything is loaded the same
// but if there is a .toml file, that is read as animation data
// schema for the .toml to follow
// and this will create a new association in our animations map

// goal for 12/26/23 revision: re-add inclusion of rect cache

use serde::{Serialize};
use walkdir::WalkDir;
use image::{GenericImage, GenericImageView, RgbaImage};
use std::{collections::HashMap, env, fs};
use indicatif::ProgressBar;

pub(crate) const IMAGE_DIR_NAME: &'static str = "packer/images/";
pub(crate) const SAVE_DIR_NAME: &'static str = "packer/out/";

#[derive(Debug, Serialize)]
pub(crate) struct Rect {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}


pub struct ImagePacker {
    pub cli: bool,
    dir_name: String,
    supported_formats: Vec<String>,
    print_output: bool,
    border: u32,
    source_rects: HashMap<String, Rect>,
    num_images: u32,
    save_name: String,
}
impl ImagePacker {
    pub fn new(dir: Option<String>) -> Self {

        let name: String;
        match dir {
            None => name = IMAGE_DIR_NAME.to_string(),
            Some(_) => name = dir.unwrap(),
        };
        let mut path_ = env::current_dir().unwrap();
        path_ = path_.join(name.clone());

        let count = WalkDir::new(path_)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().is_file())
            .count();

        Self {
            cli: false,
            dir_name: name,
            supported_formats: vec!["png".to_string()],
            print_output: false,
            border: 0,
            source_rects: HashMap::with_capacity(count),
            num_images: count as u32,
            save_name: "packed_image.png".parse().unwrap(),
        }
    }
    pub fn set_directory(&mut self, dir: String) {
        self.dir_name = dir.to_string();
    }

    pub fn set_border(&mut self, new_border: u8) {
        self.border = new_border.into();
    }

    pub fn add_supported_format(&mut self, fmt: String) {
        self.supported_formats.push(fmt);
    }

    pub fn set_print_output(&mut self, b:bool) {
        self.print_output = b;
    }

    pub fn set_file_save_name(&mut self, f_name: String) {
        self.save_name = f_name;
    }

    pub fn get_print_output(&self) -> bool {
        self.print_output
    }

    pub fn generate_animation_data_template(&self) {
        // note: this is replaced by a python script; leaving this code here if it's desired
        let mut path_ = env::current_dir().unwrap();
        path_ = path_.join(self.dir_name.clone());

        let mut animation_data = HashMap::new();

        let _ = WalkDir::new(&path_)
            .into_iter()
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.path().is_file())
            .for_each(|entry| {
                let name_ = entry.path().with_extension("");
                let file_name = name_.file_name().unwrap().to_string_lossy().to_string();
                animation_data.insert(file_name, 0);
            });
        let file = toml::to_string_pretty(&animation_data).unwrap();
        let f_name = SAVE_DIR_NAME.to_owned() + r"animation_data.toml";
        fs::write(f_name, file).unwrap();
    }

    pub fn read_files(&mut self) -> Result<(), &str> {

        // TODO: fully update the new file path system

        let mut images = vec![];
        let bar = ProgressBar::new(self.num_images as u64);

        if self.print_output {
            println!("Loading images: ");
        }
        let mut path_ = env::current_dir().unwrap();

        path_ = path_.join(self.dir_name.clone());

        let _ = WalkDir::new(&path_)
            .into_iter()
            .filter_map(|entry| entry.ok())
            .filter(|entry| !entry.path().is_dir())
            .for_each(|entry| {
                let name_with_path = entry.path().to_str().unwrap();
                let img = image::open(name_with_path);

                if let Ok(image) = img {
                    if self.print_output {
                        println!("Loading {}", name_with_path);
                    }
                    let name_ = entry.path().with_extension("");
                    let file_name = name_.file_name().unwrap().to_string_lossy().to_string();

                    images.push((file_name, image));
                    if self.print_output {
                        bar.inc(1);
                    }
                }
            });

        if self.print_output {
            bar.finish();
            println!("\nSorting images...");
        }
        // sorted descending by height
        images.sort_by(|a, b| b.1.dimensions().1.cmp(&a.1.dimensions().1));

        let mut x: u32 = 0;
        let mut y: u32 = 0;
        let mut max_height: u32 = 0;
        let mut final_height: u32 = 0;
        let mut boundary: u32 = 512;

        loop {
            let mut success = true;

            for i in &images {
                let w = i.1.dimensions().0;
                let h = i.1.dimensions().1;

                if (x + w ) > boundary {
                    if max_height == 0 {
                        max_height = h;
                    }
                    y += max_height + self.border;
                    x = 0;
                    max_height = 0;
                }
                if (y + h) > boundary {
                    success = false;
                    if self.print_output {
                        println!("Boundary size {} too small, growing boundary and trying again...", boundary);
                    }
                    boundary = ((boundary as f32) * 1.5) as u32;
                    self.source_rects.clear();
                    y = 0;
                    x = 0;
                    break;
                }

                let rect = Rect {
                    x,
                    y,
                    w,
                    h,
                };
                self.source_rects.insert(i.0.clone(), rect);

                x += w + self.border;

                if h > max_height {
                    max_height = h;
                    final_height = y + max_height + self.border;
                }
            }
            if success {
                break;
            }
        }

        if boundary == 0 || final_height == 0 {
            return Err("Invalid image dimensions! Image not saved.");
        }

        let mut final_img: RgbaImage = RgbaImage::new(boundary, final_height);

        for i in images {
            let r = self.source_rects.get(&i.0).unwrap().clone();
            let img = i.1;
            match final_img.copy_from(&img, r.x, r.y) {
                Err(_) => return Err("Attempted to copy invalid image data!"),
                Ok(_) => {},
            }
        }

        match toml::to_string_pretty(&self.source_rects) {
            Ok(toml_str) => fs::write(SAVE_DIR_NAME.to_owned() + "rect_data.toml", toml_str).expect("Error writing rect_data.toml!"),
            Err(_) => {},
        }

        if self.print_output {
            println!("Saving image...");
        }

        return match final_img.save(SAVE_DIR_NAME.to_owned() + &*self.save_name) {
            Ok(_) => Ok(()),
            Err(_) => Err("Could not save image!")
        }
    }
}
