use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    // Creating a File requires a path argument and error handling in case the file
    // does not exist. This program crashes if a "readme.txt" is not present.
    let f = File::open("readme.md").unwrap();
    let mut reader = BufReader::new(f);

    // We’ll re-use a single String object over the lifet
    let mut line = String::new();

    // loop loops until the program encounters return, break or panics
    loop {
        // Reading from disk can fail and we need to explicitly handle this.
        // In our case, errors crash the program.
        let len = reader.read_line(&mut line).unwrap();
        if len == 0 {
            break;
        }
        println!("{} ({} bytes long)", line, len);

        // Shrink the String back to length 0, preventing lines from persisting into the
        // following ones
        line.truncate(0);
    }
}
