extern crate ring;
extern crate data_encoding;

use yaml_rust::YamlLoader;

use std::io::Read;

mod hashing;

fn main() {
    hashing::hashing_pswd();

    let mut file = std::fs::File::open("config.example.yaml").expect("Unable to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read file");
    let d = YamlLoader::load_from_str(&contents).unwrap();
    println!("Read YAML string: {:?}", d);
}
