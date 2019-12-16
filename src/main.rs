use std::fs;
use std::io::{self, Read, Write};

fn main() -> io::Result<()> {
    // Read from file
    let mut input = fs::File::open("input.txt")?;
    let mut content = String::new();
    input.read_to_string(&mut content)?;
    println!("- Input -\n{}", content);

    // Encode and write to file
    let result: String = huffman::encode(content).expect("Borked encoding");
    let mut output = fs::File::create("output.txt")?;
    write!(output, "{}", result)?;
    println!("- Output -\n{}", result);

    Ok(())
}
