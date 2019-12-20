use std::fs;
use std::io;
use std::path::PathBuf;
use structopt::StructOpt;

// #[derive(Debug, StructOpt)]
// #[structopt(name = "huffman", about = "Compress files using Huffman encoding")]
// enum Opt {
//     Encode {
//         #[structopt(parse(from_os_str))]
//         input_file: PathBuf,

//         #[structopt(parse(from_os_str))]
//         output_file: PathBuf,
//     },

//     Decode {
//         #[structopt(parse(from_os_str))]
//         input_file: PathBuf,

//         #[structopt(parse(from_os_str))]
//         output_file: PathBuf,
//     },
// }

#[derive(Debug, StructOpt)]
#[structopt(name = "huffman", about = "Compress files using Huffman encoding")]
struct Opt {
    #[structopt(parse(from_os_str))]
    input_file: PathBuf,

    #[structopt(parse(from_os_str))]
    output_file: PathBuf,

    #[structopt(short, long)]
    decode: bool,

    #[structopt(short, long)]
    quiet: bool,

    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    verbose: usize,
}

fn main() -> io::Result<()> {
    let opt = Opt::from_args();

    stderrlog::new()
        .module(module_path!())
        .quiet(opt.quiet)
        .verbosity(opt.verbose + 1)
        .timestamp(stderrlog::Timestamp::Millisecond)
        .init()
        .unwrap();

    if !opt.decode {
        // Encode
        let mut input = fs::File::open(&opt.input_file)?;
        let mut output = fs::File::create(&opt.output_file)?;
        huffman::encode_to(&mut input, &mut output)?;

    // // Read from file
    // let input = fs::read_to_string(&opt.input_file)?;

    // // Encode and write to file
    // let encoded = huffman::encode(&input).unwrap();
    // fs::write(&opt.output_file, &encoded)?;
    } else {
        // Decode
        // Read from file and decode
        let input = fs::read(&opt.input_file)?;

        // Decode and verify
        let decoded = huffman::decode(&input).unwrap();
        fs::write(&opt.output_file, &decoded)?;
    }

    Ok(())
}
