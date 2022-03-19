use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    // Creating a File requires a path argument and error handling in case the file
    // does not exist. This program crashes if a "readme.txt" is not present.
    let f = File::open("readme.md").unwrap();
    let reader = BufReader::new(f);

    // There is a subtle behavior change here.
    // BufReader::lines() removes the trailing newline character
    // from each line
    for line_ in reader.lines() {
        // Unwrapping the Result possible error at each line is
        // still required, as with the manual version
        let line = line_.unwrap();
        println!("{} ({} bytes long)", line, line.len());
    }
}
