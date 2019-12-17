pub mod huffman_tree;

use huffman_tree::HuffmanTree;
use std::collections::HashMap;

pub fn encode(input: String) -> Vec<u8> {
    if input.is_empty() {
        return Vec::new();
    }

    let counts = counts(&input);
    let ht = HuffmanTree::from(&counts);
    let encoded = ht.encode(&input).unwrap();

    encoded
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

// Huffman encoding format:
//
// In helper file to create tree:
//
//  a: 0
//  b: 10
//  ...
//  `bit length`
//
// In compressed file:
//
//  010100101...
//
fn string_to_bytes(s: &String) -> (Vec<u8>, usize) {
    let buflen = (s.len() as f64 / 8.0).ceil() as usize;
    let mut buf = vec![0u8; buflen];
    let mut idx = 0;
    println!("{}", s);

    for c in s.chars() {
        let byte_idx = idx / 8;
        let bit_idx = 7 - idx % 8;
        let digit = c.to_digit(2).expect("Expected 0 or 1") as u8;
        buf[byte_idx] |= digit << bit_idx;
        idx += 1;
    }

    for i in &buf {
        print!("{:08b}", i);
    }
    println!();

    (buf, s.len())
}
