use std::fs::File;
use std::io::Read;
use std::path::Path;

pub struct Counter {
    contents: String
}

impl Counter {
    pub fn new(path: &Path) -> Counter {
        let mut file = File::open(&path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        Counter{ contents: contents }
    }

    pub fn words(&self) -> usize {
        let mut word_count = 0;
        for line in self.contents.lines() {
            word_count += line.split_whitespace().count();
        }

        word_count
    }
}
