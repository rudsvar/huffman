use std::fs;
use std::io::{self, Read, Write};

fn main() -> io::Result<()> {
    // Read from file
    let content = fs::read_to_string("input.txt")?;
    eprintln!("Input\n{:?}", content);

    // Encode and write to file
    let encoded: Vec<u8> = huffman::encode(content);
    eprintln!("Output");
    for v in &encoded {
        eprint!("{:08b}", v);
    }
    eprintln!("");

    Ok(())
}
