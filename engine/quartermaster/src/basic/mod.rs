use std::{fs::File, io::Read, path::Path};

use crate::qm::AssetLoader;

#[derive(Default)]
pub struct TextLoader;

impl AssetLoader<String> for TextLoader {
    fn load_asset(&self, path: &String) -> String {
        let file_path = Path::new(path);
        let display = file_path.display();

        let mut file = match File::open(&file_path) {
            Err(reason) => panic!("Could not open {}: {}", display, reason),
            Ok(file) => file,
        };

        let mut s = String::new();
        match file.read_to_string(&mut s) {
            Err(reason) => panic!("Could not read {}: {}", display, reason),
            Ok(count) => println!("Read {}: {}", display, count),
        };

        s
    }
}

