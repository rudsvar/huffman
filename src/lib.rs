pub mod huffman_tree;

use huffman_tree::HuffmanTree;
use std::collections::HashMap;
use std::fmt::Write;

pub fn encode(input: &String) -> Option<Vec<u8>> {
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

pub fn decode(input: &Vec<u8>) -> Option<String> {
    if input.is_empty() {
        return Some(String::new());
    }

    // Split into three parts
    let newline = 10;
    let mut idx = 0;

    // Read json
    let mut json = Vec::new();
    loop {
        if input[idx] == newline {
            idx += 1;
            break;
        }
        json.push(input[idx]);
        idx += 1;
    }
    let ht: HuffmanTree = serde_json::from_slice(&json).expect("Could not read json");

    // Read num
    let mut num_str = Vec::new();
    loop {
        if input[idx] == newline {
            idx += 1;
            break;
        }
        num_str.push(input[idx]);
        idx += 1;
    }
    let n_bits: usize = String::from_utf8(num_str)
        .expect("Could not read ")
        .parse()
        .unwrap();

    // Read content
    let encoded = &input[idx..];

    let decoded = ht.decode(encoded, n_bits).unwrap();

    Some(decoded)
}

fn counts(input: &String) -> HashMap<char, usize> {
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

    use super::{decode, encode};

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
    fn encode_decode_single() {
        encode_decode("x");
    }

    #[test]
    fn encode_decode_1() {
        encode_decode("abbccc");
    }

    #[test]
    fn encode_decode_2() {
        encode_decode("This is a test string.\nIt has two lines.");
    }
}
