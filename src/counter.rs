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

#[cfg(test)]
mod tests {
    use super::Counter;
    use std::path::Path;

    #[test]
    fn words_returns_the_correct_number_of_words_in_the_specified_path_contents() {
        let path = Path::new("tests/sample_file");
        let counter = Counter::new(&path);

        assert_eq!(counter.words(), 6);
    }
}
