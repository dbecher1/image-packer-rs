
// Loads a directory of images_vec recursively and packs it
// Algorithm taken from a blog post by David Colson
// https://www.david-colson.com/2020/03/10/exploring-rect-packing.html
// this is my third time making this sort of implementation in my 3rd language
// the short of it is this flow:
// create a packer struct with new()
// this will set up all the main variables and etc
// things can be changed before read_files() is called
// read_files will do the magic

// this is the schema:
// reads files from gfx
// any file in root gets read right into source_rects
// source_rects is a map, where the key is the file path + name
// within subdirectories, everything is loaded the same
// but if there is a .toml file, that is read as animation data
// schema for the .toml to follow
// and this will create a new association in our animations map

use walkdir::WalkDir;
use image::{DynamicImage, GenericImage, GenericImageView, RgbaImage};
use std::collections::HashMap;
use std::fs;
use indicatif::ProgressBar;
use crate::definitions::*;

pub struct Packer {
    dir_name: String,
    supported_formats: Vec<String>,
    image_data: Option<DynamicImage>,
    load_tiled_map_data: bool, // FUTURE IMPL
    save_image_to_file: bool,
    store_image_data: bool,
    print_output: bool,
    border: u32,
    source_rects: HashMap<String, Rect>,
    num_images: u32,
    animation_data: HashMap<String, Animation>,
}
// structure for animation data will be: PARENT-ANIMATION NAME maps to animation frames
// ie Player-_Attack maps to the data for _Attack

impl Packer {
    pub fn new(dir: Option<String>) -> Self {

        // to be more memory efficient we'll crawl through and count how many images we have
        let mut count: u32 = 0;
        let name: String;
        match dir {
            None => name = IMAGE_DIR_NAME.to_string(),
            Some(_) => name = dir.unwrap(),
        };
        for entry in WalkDir::new(&name)
            .into_iter()
            .filter_map(|e| e.ok()) {
            if entry.path().is_file() {
                count += 1;
            }
            // println!("{}", count);
        }

        Self {
            dir_name: name,
            supported_formats: vec!["png".to_string()],
            image_data: None,
            load_tiled_map_data: false,
            save_image_to_file: true,
            store_image_data: true,
            print_output: true,
            border: 0,
            source_rects: HashMap::with_capacity(count as usize),
            num_images: count,
            animation_data: HashMap::new(),
        }
    }

    pub fn set_directory(&mut self, dir: &str) {
        self.dir_name = dir.to_string();
    }

    pub fn add_supported_format(&mut self, fmt: &str) {
        self.supported_formats.push(fmt.to_string());
    }

    pub fn set_store_image_data(&mut self, b: bool) {
        self.store_image_data = b;
    }

    pub fn set_load_tiled_map_data(&mut self, b: bool) {
        self.load_tiled_map_data = b;
    }

    pub fn set_save_image_to_file(&mut self, b: bool) {
        self.save_image_to_file = b;
    }

    pub fn set_print_output(&mut self, b:bool) {
        self.print_output = b;
    }

    fn read_animation_data<'a>(&self) -> HashMap<String, HashMap<String, AnimationData>> {
        let mut map: HashMap<String, HashMap<String, AnimationData>> = HashMap::new();
        println!("Reading animation data files...");

        for entry in WalkDir::new(ANIMATION_DIR_NAME)
            .into_iter()
            .filter_map(|e| e.ok()) {
            if entry.path().is_dir() {
                continue;
            }
            let f_name = entry.path();
            // read toml files
            let file_contents = fs::read_to_string(f_name)
                .expect("Error reading what should be a .toml file!");
            let file: AnimationDataFile = toml::from_str(&file_contents)
                .expect("Error parsing toml file!");

            for f in file.animation {
                let folder_name_ =entry.path().with_extension("");
                let folder_name = folder_name_.file_name().unwrap().to_string_lossy().to_string();
                if !map.contains_key(&folder_name) {
                    map.insert(
                        folder_name.clone(),
                        HashMap::new()
                    );
                }
                let ad = AnimationData {
                    speed: f.speed,
                    columns: f.columns,
                    rows: f.rows,
                };
                map.get_mut(&folder_name)
                    .unwrap()
                    .insert(f.name.clone(), ad);
            }
        }
        println!("Animation data successfully read!");
        return map
    }

    pub fn read_files<'a>(&mut self) -> Result<(), ()> {

        if !self.save_image_to_file && !self.store_image_data {
            return Err(())
        }
        let animation_data = self.read_animation_data();
        let mut animation_to_parent: HashMap<String, String> = HashMap::new();
        // maps animation name to parent directory

        let mut images = vec![];
        let bar = ProgressBar::new(self.num_images as u64);

        if self.print_output {
            println!("Loading images: ");
        }

        for entry in WalkDir::new(IMAGE_DIR_NAME)
            .into_iter()
            .filter_map(|e| e.ok()) {
            if !entry.path().is_dir() {
                let name_with_path = entry.path().to_str().unwrap();
                let img = image::open(name_with_path);

                if let Err(e) = &img {
                    eprintln!("{}",e);
                    continue;
                }
                let name_ = entry.path().with_extension("");
                let parent = entry.path().parent().unwrap().file_name().unwrap().to_string_lossy().to_string();
                let file_name = name_.file_name().unwrap().to_string_lossy().to_string();
                animation_to_parent.insert(file_name.clone(), parent);

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
                    self.animation_data.clear();
                    y = 0;
                    x = 0;
                    break;
                }
                // animation stuff
                let find_parent = animation_to_parent.get(&i.name);
                if let Some(parent) = find_parent {
                    if let Some(name_to_data) = animation_data.get(parent) {
                        if let Some(frame_data) = name_to_data.get(&i.name) {
                            let speed = frame_data.speed;
                            let columns = frame_data.columns;
                            let rows = frame_data.rows;
                            let frame_width = w / columns;
                            let frame_height = h / rows;

                            let mut data: Vec<Rect> = vec![];

                            for y_ in (y..rows).step_by(frame_height as usize) {
                                for x_ in (x..columns).step_by(frame_width as usize) {
                                    data.push(Rect {
                                        x: x_,
                                        y: y_,
                                        w: frame_width,
                                        h: frame_height,
                                    });
                                }
                            }
                            let key = parent.to_string() + "-" + &i.name;
                            self.animation_data.insert(key, Animation { data, speed});
                        }
                    }
                }
                // end animation stuff
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

        if self.save_image_to_file {
            if self.print_output {
                println!("Saving image...");
            }
            final_img.save("packed_image.png")
                .expect("Error saving image!");

            if self.print_output {
                println!("Done!");
            }
        }
        if self.store_image_data {
            self.image_data = Some(DynamicImage::from(final_img));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::packer::{AnimationDataFile};

    #[test]
    fn test_toml_reader() {
        let file = "[[animations]]
name = '_Attack'
speed = 150
columns = 4
rows = 1

[[animations]]
name = '_AttackDown'
speed = 200
columns = 4
rows = 1";
        let result: AnimationDataFile = toml::from_str(file)
            .expect("Error reading file!");
        assert_eq!(result.animation[0].speed, 150);
        assert_eq!(result.animation[1].speed, 200);
    }
}