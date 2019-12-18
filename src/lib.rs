mod huffman_tree;

use huffman_tree::HuffmanTree;
use std::collections::HashMap;
use std::fmt::Write;

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

    // Split into header
    let (a, b) = header_split_locations(input)?;
    let ht: HuffmanTree = serde_json::from_slice(&input[0..a]).expect("Invalid json");
    let n_bits: usize = String::from_utf8(input[a + 1..b].to_vec())
        .expect("Not valid utf8")
        .parse()
        .expect("Invalid number of bits");
    let encoded = &input[b + 1..];

    Some(ht.decode(encoded, n_bits)?)
}

/// Take from the vector until a newline character is reached.
fn header_split_locations(s: &[u8]) -> Option<(usize, usize)> {
    let mut locations = Vec::new();
    for (i, c) in s.iter().enumerate() {
        if locations.len() == 2 {
            break;
        }
        // Check if it is a line feed
        if *c == 10 {
            locations.push(i);
        }
    }

    Some((*locations.get(0)?, *locations.get(1)?))
}

/// Get the frequency of each character in the provided string.
fn counts(input: &str) -> HashMap<char, usize> {
    let mut cts = HashMap::new();
    for c in input.chars() {
        match cts.get(&c) {
            None => cts.insert(c, 1),
            Some(&prev) => cts.insert(c, prev + 1),
        };
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
