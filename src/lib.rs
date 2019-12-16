pub mod huffman_tree;

use std::collections::HashMap;

pub fn encode(input: String) -> Option<String> {
    let counts = counts(&input);
    let ht = huffman_tree::HuffmanTree::from(&counts)?;
    let mut output = String::new();
    for c in input.chars() {
        let code = ht.encode_char(c).expect("Borked code?");
        println!("Code for {} is {}", c, code);
        output.push_str(&(code + " "));
    }

    Some(output)
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
