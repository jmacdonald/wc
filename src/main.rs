mod counter;

use std::env;
use std::path::Path;

use counter::Counter;

fn main() {
    let args = env::args();
    let path_arg = args.skip(1).nth(0).unwrap();
    let path = Path::new(&path_arg);

    match Counter::new(&path) {
        Ok(counter) => println!("{} words found", counter.words()),
        Err(e) => println!("Error: {}", e),
    }
}
