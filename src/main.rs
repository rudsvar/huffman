use clap::Parser;
use std::fs;
use std::io;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[clap(name = "huffman", about = "Compress files using Huffman encoding")]
struct Opt {
    /// The file to read from
    input_file: PathBuf,

    /// The file to write to
    output_file: PathBuf,

    /// Decode a file instead
    #[clap(short, long)]
    decode: bool,

    /// Suppresses all output
    #[clap(short, long)]
    quiet: bool,
}

fn main() -> io::Result<()> {
    let opt = Opt::parse();

    tracing_subscriber::fmt().init();

    let mut input = fs::File::open(&opt.input_file)?;
    let mut output = fs::File::create(&opt.output_file)?;

    if opt.decode {
        huffman::decode_to(&mut input, &mut output)?;
    } else {
        huffman::encode_to(&mut input, &mut output)?;
    }

    Ok(())
}
