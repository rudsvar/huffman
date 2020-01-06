pub mod bits;
pub mod huffman_tree;

use huffman_tree::HuffmanTree;
use log::*;
use std::collections::HashMap;
use std::fs;
use std::io::{self, BufRead, BufReader, BufWriter, Read, Seek, Write};

/// A wrapper function that encodes a string
/// and returns a vector of encoded data.
///
/// # Examples
///
/// ```
/// let input = "foo bar baz";
/// let encoded = huffman::encode(input);
/// assert!(encoded.is_ok());
/// ```
///
pub fn encode(input: &str) -> io::Result<Vec<u8>> {
    if input.is_empty() {
        return Ok(Vec::new());
    }

    let mut input = io::Cursor::new(input);
    let mut output = Vec::new();
    encode_to(&mut input, &mut output)?;

    Ok(output)
}

/// A memory efficient function for encoding.
///
/// This function will read from the input as necessary, and
/// write continuously to the output rather than load
/// it all into memory.
///
pub fn encode_to<A, B>(input: &mut A, output: &mut B) -> io::Result<()>
where
    A: io::Read + io::Seek,
    B: io::Write,
{
    let mut input = BufReader::new(input);
    let mut output = BufWriter::new(output);

    debug!("Counting");
    // Count character frequencies, then return to start
    let counts = counts(&mut input);
    input.seek(std::io::SeekFrom::Start(0))?;

    debug!("Constructing tree");
    // Generate a tree using the counts
    let ht = HuffmanTree::from(&counts);
    let ht_json = serde_json::to_string(&ht)?;

    debug!("Encode and write to tmp");
    // Write encoded data to temporary file
    let tmp_path = mktemp::Temp::new_file()?;
    let tmp = fs::File::create(&tmp_path)?;
    let mut tmp = BufWriter::new(tmp);
    let n_bits = ht.encode_to(&mut input, &mut tmp)?;
    tmp.flush()?;

    debug!("Write metadata");
    // Write metadata
    write!(&mut output, "{}\n{}\n", ht_json, n_bits)?;

    debug!("Append encoded data to output");
    // Append encoded data to output
    let mut tmp = fs::File::open(&tmp_path)?;
    io::copy(&mut tmp, &mut output)?;
    debug!("Done");

    Ok(())
}

/// Get the frequency of each byte in the provided input.
fn counts<T: BufRead>(input: &mut T) -> HashMap<u8, usize> {
    let mut cts = HashMap::new();
    for byte in input.bytes() {
        let c = byte.unwrap();
        *cts.entry(c).or_insert(0) += 1;
    }
    cts
}

/// A memory efficient function for decoding.
///
/// This function will read from the input as necessary, and
/// write continuously to the output rather than load
/// it all into memory.
///
pub fn decode_to<A, B>(input: &mut A, output: &mut B) -> io::Result<()>
where
    A: io::Read + io::Seek,
    B: io::Write,
{
    let mut input = BufReader::new(input);
    let mut output = BufWriter::new(output);

    // Read serialized Huffman tree
    let mut ht_str = String::new();
    input.read_line(&mut ht_str).expect("Could not read json");
    let ht: HuffmanTree = serde_json::from_str(&ht_str).expect("Invalid json");

    // Read `n_bits`
    let mut n_bits_str = String::new();
    input
        .read_line(&mut n_bits_str)
        .expect("Could not read n_bits");
    let n_bits: usize = n_bits_str
        .trim_end()
        .parse()
        .expect("Invalid number of bits");

    // Read the encoded data
    ht.decode_to(&mut input, &mut output, n_bits)?;

    Ok(())
}

/// A wrapper function for decoding.
///
/// This function decodes data produced by `encode`.
/// Given a slice of `u8`, it will decode it if possible,
/// and return the decoded string.
///
/// # Examples
///
/// ```
/// let input = "foo bar baz";
/// let encoded = huffman::encode(input).unwrap();
/// let decoded = huffman::decode(&encoded).unwrap();
/// assert_eq!(input, decoded);
/// ```
///
pub fn decode(input: &[u8]) -> io::Result<String> {
    if input.is_empty() {
        return Ok(String::new());
    }

    let mut input = io::Cursor::new(input);
    let mut output = Vec::new();
    decode_to(&mut input, &mut output)?;

    Ok(String::from_utf8_lossy(&output).to_string())
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::io::Cursor;

    #[test]
    fn counts_test() {
        let cts = counts(&mut BufReader::new(Cursor::new(b"aaabbc\n")));
        assert_eq!(cts.get(&b'a'), Some(&3));
        assert_eq!(cts.get(&b'b'), Some(&2));
        assert_eq!(cts.get(&b'c'), Some(&1));
        assert_eq!(cts.get(&b'\n'), Some(&1));
        assert_eq!(cts.get(&b'x'), None);
    }

    fn encode_decode(input: &str) {
        let input = String::from(input);
        let encoded = encode(&input).unwrap();
        let decoded = decode(&encoded).unwrap();
        assert_eq!(decoded, input);
    }

    #[test]
    fn encode_decode_empty() {
        encode_decode("");
    }

    #[test]
    fn encode_decode_char() {
        encode_decode("x");
        encode_decode("@");
        encode_decode("\n");
    }

    #[test]
    fn encode_decode_string() {
        encode_decode("abbccc");
        encode_decode("abcde");
        encode_decode("aaaaaaaaaaaa");
    }

    #[test]
    fn encode_decode_whitespace() {
        encode_decode("This is a test string.\nIt has two lines.\n");
        encode_decode("This is also a test string, but this one is longer.");
    }

    #[test]
    fn encode_decode_special() {
        encode_decode("!!!@##$$%%%^&&**(_)");
    }
}
