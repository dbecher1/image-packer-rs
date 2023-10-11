use Packer2::packer::*;

fn main() {
    let mut p = Packer::new(None);
    p.read_files().unwrap();
}
