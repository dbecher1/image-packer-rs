
use clap::{Parser};
use crate::packer::ImagePacker;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    file_name: Option<String>,

    #[arg(short, long, default_value_t = false)]
    verbose: bool,

    #[arg(short, long, default_value_t = 0)]
    border: u8,

    #[arg(short, long)]
    dir_name: Option<String>,

    #[arg(short, long)]
    extensions: Vec<String>,
}

pub fn cli_parse(image_packer: &mut ImagePacker) {
    image_packer.cli = true;

    let args = Cli::parse();

    if let Some(f) = args.file_name {
        image_packer.set_file_save_name(f);
    }
    if let Some(dir) = args.dir_name {
        image_packer.set_directory(dir);
    }
    if !args.extensions.is_empty() {
        for e in args.extensions {
            if e == "png" {
                continue;
            }
            image_packer.add_supported_format(e);
        }
    }
    if args.border > 0 {
        image_packer.set_border(args.border);
    }
    if args.verbose {
        image_packer.set_print_output(true);
    }
}