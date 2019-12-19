use std::fs;
use std::io;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "huffman", about = "Compress files using Huffman encoding")]
enum Opt {
    Encode {
        #[structopt(parse(from_os_str))]
        input_file: PathBuf,

        #[structopt(parse(from_os_str))]
        output_file: PathBuf,
    },

    Decode {
        #[structopt(parse(from_os_str))]
        input_file: PathBuf,

        #[structopt(parse(from_os_str))]
        output_file: PathBuf,
    },
}

fn main() -> io::Result<()> {
    let opt = Opt::from_args();

    match opt {
        Opt::Encode {
            input_file,
            output_file,
        } => {
            // Read from file
            let input = fs::read_to_string(input_file)?;

            // Encode and write to file
            eprintln!("Encoding");
            let encoded = huffman::encode(&input).unwrap();
            eprintln!("Done");
            fs::write(&output_file, &encoded)?;
        }

        Opt::Decode {
            input_file,
            output_file,
        } => {
            // Read from file and decode
            let input = fs::read(&input_file)?;

            // Decode and verify
            eprintln!("Decoding");
            let decoded = huffman::decode(&input).unwrap();
            eprintln!("Done");
            fs::write(&output_file, &decoded)?;
        }
    }

    Ok(())
}
