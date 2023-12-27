
use image_packer::packer::ImagePacker;

#[cfg(feature = "cli")]
use image_packer::cli::*;

fn main() {

    let mut image_packer = ImagePacker::new(None);

    #[cfg(feature = "cli")]
    cli_parse(&mut image_packer);

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

