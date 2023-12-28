
use std::fs;

fn main() {
    // create the folder structure
    fs::create_dir_all("packer/images").expect("Error creating directory!");
    fs::create_dir_all("packer/out").expect("Error creating directory!");

    // python script
    let py_name = "packer/out/generate_animation_data_template.py";
    let py_src = "from os import listdir
from os.path import isfile, join

folder = 'packer/images/'
out_dir = 'packer/out/'
out_name = 'animation_data.toml'
out = ''

file_names = [f for f in listdir(folder) if isfile(join(folder, f))]
file_names_no_ext = list(map(lambda f : f.split('.')[0], file_names))

for f in file_names_no_ext:
    out += f + ' = 0\\n'
with open(out_dir + out_name, 'w') as f:
    f.write(out)";
    fs::write(py_name, py_src).expect("Error generating python script!");
    let curr_dir = std::env::current_dir().unwrap();
    let test = curr_dir.join(py_name);
    fs::write(test, "lol").unwrap();
}