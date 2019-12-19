mod huffman_tree;

use huffman_tree::HuffmanTree;
use std::collections::HashMap;
use std::fmt::Write;
use std::io::{BufRead, BufReader, Read};

/// Encode the given string using Huffman coding,
/// and return a vector of `u8`.
///
/// # Examples
///
/// ```
/// let input = "foo bar baz";
/// let encoded = huffman::encode(input);
/// assert!(encoded.is_some());
/// ```
///
pub fn encode(input: &str) -> Option<Vec<u8>> {
    if input.is_empty() {
        return Some(Vec::new());
    }

    let counts = counts(&input);
    let ht = HuffmanTree::from(&counts);
    let ht_json = serde_json::to_string(&ht).expect("Could not convert to json");
    let (mut encoded, n_bits) = ht.encode(&input)?;

    let mut header = String::new();
    write!(header, "{}\n{}\n", ht_json, n_bits).expect("Could not write to string");

    let mut output = Vec::from(header.as_bytes());
    output.append(&mut encoded);

    Some(output)
}

/// Decode the slice of `u8` that was
/// produced by `huffman::encode`.
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
pub fn decode(input: &[u8]) -> Option<String> {
    if input.is_empty() {
        return Some(String::new());
    }

    let mut br = BufReader::new(input);

    // Read serialized Huffman tree
    let mut ht_str = String::new();
    br.read_line(&mut ht_str).expect("Could not read json");
    let ht: HuffmanTree = serde_json::from_str(&ht_str).expect("Invalid json");

    // Read `n_bits`
    let mut n_bits_str = String::new();
    br.read_line(&mut n_bits_str)
        .expect("Could not read n_bits");
    let n_bits: usize = n_bits_str
        .trim_end()
        .parse()
        .expect("Invalid number of bits");

    // Read the encoded data
    let mut encoded = Vec::new();
    br.read_to_end(&mut encoded)
        .expect("Could not read encoded data");

    Some(ht.decode(&encoded, n_bits)?)
}

/// Get the frequency of each character in the provided string.
fn counts(input: &str) -> HashMap<char, usize> {
    let mut cts = HashMap::new();
    for c in input.chars() {
        *cts.entry(c).or_insert(0) += 1;
    }
    cts
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn counts_test() {
        let cts = counts("aaabbc\n");
        assert_eq!(cts.get(&'a'), Some(&3));
        assert_eq!(cts.get(&'b'), Some(&2));
        assert_eq!(cts.get(&'c'), Some(&1));
        assert_eq!(cts.get(&'\n'), Some(&1));
        assert_eq!(cts.get(&'x'), None);
    }

    fn encode_decode(input: &str) {
        let input = String::from(input);
        let encoded = encode(&input).unwrap();
        let decoded = decode(&encoded).unwrap();
        assert_eq!(input, decoded);
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
