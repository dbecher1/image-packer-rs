
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

use walkdir::WalkDir;
use image::{GenericImage, GenericImageView, RgbaImage};
use std::{collections::HashMap, env};
use indicatif::ProgressBar;
use crate::definitions::*;

pub struct ImagePacker {
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

        // to be a wee more memory efficient we'll crawl through and count how many images we have
        let mut count: u32 = 0;
        let name: String;
        match dir {
            None => name = IMAGE_DIR_NAME.to_string(),
            Some(_) => name = dir.unwrap(),
        };
        let mut path_ = env::current_exe().unwrap();
        let _ = path_.pop();
        path_ = path_.join(name.clone());

        for entry in WalkDir::new(path_)
            .into_iter()
            .filter_map(|e| e.ok()) {
            if entry.path().is_file() {
                count += 1;
            }
        }

        Self {
            dir_name: name,
            supported_formats: vec!["png".to_string()],
            print_output: false,
            border: 0,
            source_rects: HashMap::with_capacity(count as usize),
            num_images: count,
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

    pub fn read_files(&mut self) -> Result<(), ()> {

        let mut images = vec![];
        let bar = ProgressBar::new(self.num_images as u64);

        if self.print_output {
            println!("Loading images: ");
        }
        let mut path_ = env::current_exe().unwrap();
        let _ = path_.pop();
        path_ = path_.join(self.dir_name.clone());

        for entry in WalkDir::new(&path_)
            .into_iter()
            .filter_map(|e| e.ok()) {
            if !entry.path().is_dir() {
                let name_with_path = entry.path().to_str().unwrap();
                let img = image::open(name_with_path);

                if let Err(e) = &img {
                    eprintln!("{}",e);
                    continue;
                }
                // I'm sorry for how ugly this is
                let name_ = entry.path().with_extension("");
                let file_name = name_.file_name().unwrap().to_string_lossy().to_string();

                images.push(ImgWrapper{
                    image: img.unwrap(),
                    name: file_name,
                });
                if self.print_output {
                    bar.inc(1);
                }
            }
        }
        if self.print_output {
            bar.finish();
            println!("\nSorting images...");
        }
        images.sort_by(|a, b| b.image.dimensions().1.cmp(&a.image.dimensions().1));
        // sorted descending by height

        let mut x: u32 = 0;
        let mut y: u32 = 0;
        let mut max_height: u32 = 0;
        let mut final_height: u32 = 0;
        let mut boundary: u32 = 512;

        loop {
            let mut success = true;

            for i in &images {
                let w = i.image.dimensions().0;
                let h = i.image.dimensions().1;

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
                let name = &i.name;
                self.source_rects.insert(name.clone(), rect);

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
        let mut final_img: RgbaImage = RgbaImage::new(boundary, final_height);

        for i in images {
            let r = self.source_rects.get(&i.name).unwrap().clone();
            let img = &i.image;
            final_img.copy_from(img, r.x, r.y).unwrap();
        }

        if self.print_output {
            println!("Saving image...");
        }
        let mut path_ = env::current_exe().unwrap();
        path_.pop();
        path_ = path_.join(self.save_name.to_string());
        final_img.save(path_).expect("Error saving image!");

        if self.print_output {
            println!("Done!");
        }

        Ok(())
    }
}
