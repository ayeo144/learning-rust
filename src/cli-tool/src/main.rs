use std::path::Path;
use std::fs::File;
use std::io::{BufWriter, Write};


fn main() {
    // Get the arguments from the command-line
    let _program_name = std::env::args().nth(0);
    let filepath = std::env::args().nth(1).expect("No filepath given");
    let message = std::env::args().nth(2).expect("No message given");

    // Create a Path from the input filepath
    let path = Path::new(&filepath);

    // Write to the file
    let f = File::create(&path).expect("Unable to create file!");
    let mut f = BufWriter::new(f);
    f.write_all(message.as_bytes()).expect("Failed to write to file!");
}
