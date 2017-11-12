use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;

fn main() {
    let args = env::args();
    let path_arg = args.skip(1).nth(0).unwrap();
    let path = Path::new(&path_arg);

    let mut file = File::open(&path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut word_count = 0;
    for line in contents.lines() {
        word_count += line.split_whitespace().count();
    }

    println!("{} words found", word_count);
}
