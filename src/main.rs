
use image_packer::ImagePacker;

fn main() {

    let mut image_packer = ImagePacker::new(None);

    match image_packer.read_files() {
        Ok(_) => {
            if image_packer.get_print_output() {
                println!("Image successfully saved!");
            }
        }
        Err(msg) => {
            eprintln!("An error has occurred: {}", msg);
        }
    }
}

