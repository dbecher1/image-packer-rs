
use walkdir::WalkDir;
use image::GenericImageView;

const DIR_NAME: &'static str = "gfx";

pub struct Packer {
    test: u8,
}

impl Packer {

    pub fn new() -> Self {
        Self {
            test: 0,
        }
    }
    pub fn read_files(&self) -> () {

        let mut temp1: bool = false;
        let mut temp2: Option<String> = None;

        for entry in WalkDir::new(DIR_NAME)
            .into_iter()
            .filter_map(|e| e.ok()) {

            let path = entry
                .path()
                .to_str()
                .unwrap();
                //.replace(DIR_NAME, "");

            let mut p3 = path.to_string();
            //p3.remove(0);
            if !temp1 && !p3.is_empty() && !entry.path().is_dir() {
                println!("{}", p3);
                temp2 = Some(path.to_string());
                temp1 = true;
            }

            //println!("{}", p3);
        }
        let temp3 = image::open(temp2.unwrap()).unwrap();
        temp3.save("test.png").unwrap();
    }
}