use clap::Clap;
use std::fs;
use std::io;
use std::path::PathBuf;

#[derive(Debug, Clap)]
#[clap(name = "huffman", about = "Compress files using Huffman encoding")]
struct Opt {
    /// The file to read from
    #[clap(parse(from_os_str))]
    input_file: PathBuf,

    /// The file to write to
    #[clap(parse(from_os_str))]
    output_file: PathBuf,

    /// Decode a file instead
    #[clap(short, long)]
    decode: bool,

    /// Suppresses all output
    #[clap(short, long)]
    quiet: bool,

    /// Select how much debug information to print
    #[clap(short = 'v', long = "verbose", parse(from_occurrences))]
    verbose: usize,
}

fn main() -> io::Result<()> {
    let opt = Opt::parse();

    stderrlog::new()
        .module(module_path!())
        .quiet(opt.quiet)
        .verbosity(opt.verbose)
        .timestamp(stderrlog::Timestamp::Millisecond)
        .init()
        .unwrap();

    let mut input = fs::File::open(&opt.input_file)?;
    let mut output = fs::File::create(&opt.output_file)?;

    if opt.decode {
        huffman::decode_to(&mut input, &mut output)?;
    } else {
        huffman::encode_to(&mut input, &mut output)?;
    }

    Ok(())
}
