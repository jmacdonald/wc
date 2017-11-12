use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

pub struct Counter {
    contents: String
}

impl Counter {
    pub fn new(path: &Path) -> io::Result<Counter> {
        let mut file = File::open(&path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        Ok(Counter{ contents: contents })
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

        assert_eq!(counter.unwrap().words(), 6);
    }

    #[test]
    fn words_handles_invalid_path_errors() {
        let path = Path::new("non-existent");
        let counter = Counter::new(&path);

        assert!(counter.is_err());
    }

    #[test]
    fn words_handles_invalid_file_contents() {
        let path = Path::new("tests/invalid_file");
        let counter = Counter::new(&path);

        assert!(counter.is_err());
    }
}
