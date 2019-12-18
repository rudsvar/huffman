use std::fs;
use std::io::{self, Read};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Huffman",
    about = "A program for compressing files using Huffman encoding"
)]
struct Opt {
    #[structopt(parse(from_os_str))]
    input_file: PathBuf,

    #[structopt(parse(from_os_str))]
    output_file: PathBuf,
}

fn main() -> io::Result<()> {
    let opt = Opt::from_args();

    // Read from file
    let input = fs::read_to_string(opt.input_file)?;

    // Encode and write to file
    let encoded = huffman::encode(&input).unwrap();
    let output_file = opt.output_file;
    fs::write(output_file, &encoded)?;

    // Read from file and decode
    let mut encoded_file = fs::File::open("encoded.txt")?;
    let mut content = Vec::new();
    encoded_file.read_to_end(&mut content)?;

    // Decode and verify
    let decoded = huffman::decode(encoded).unwrap();
    assert_eq!(input, decoded);

    Ok(())
}
