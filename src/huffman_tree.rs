//! A module that defines a `HuffmanTree`.
//!
//! The Huffman tree is constructed using Huffman encoding,
//! and is used to get the encoding of a given character.
//! It can then also be used to decode encoded data.

use serde::{Deserialize, Serialize};
use std::cmp;
use std::collections::BinaryHeap;
use std::io::{self, Read, Write};

use crate::bits::{BitWriter, Biterator};

/// The struct representing the Huffman tree.
#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize)]
pub enum HuffmanTree {
    Node {
        weight: usize,
        zero: Box<HuffmanTree>,
        one: Box<HuffmanTree>,
    },
    Leaf(u8, usize),
}

impl Ord for HuffmanTree {
    fn cmp(&self, other: &HuffmanTree) -> cmp::Ordering {
        other.weight().cmp(&self.weight())
    }
}

impl PartialOrd for HuffmanTree {
    fn partial_cmp(&self, other: &HuffmanTree) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl HuffmanTree {
    /// Get the weight of the tree.
    ///
    /// This is used during construction.
    pub fn weight(&self) -> usize {
        match self {
            Self::Node { weight, .. } => *weight,
            Self::Leaf(_, w) => *w,
        }
    }

    /// Construct a `HuffmanTree` from character frequencies.
    pub fn from(counts: &[usize; u8::MAX as usize]) -> HuffmanTree {
        let mut trees: BinaryHeap<HuffmanTree> = BinaryHeap::new();

        // If there is only one character, a special tree is made.
        if counts.iter().filter(|&x| *x > 0).count() == 1 {
            let (k, &v) = counts
                .iter()
                .enumerate()
                .find(|(_, &x)| x > 0)
                .expect("Should be one byte with high enough count");
            return HuffmanTree::Node {
                zero: Box::new(HuffmanTree::Leaf(k as u8, v)),
                one: Box::new(HuffmanTree::Leaf(k as u8, v)),
                weight: v,
            };
        }

        // Add the initial trees.
        for (k, v) in counts.iter().enumerate() {
            // Only add characters that are actually used
            if *v != 0 {
                trees.push(Self::Leaf(k as u8, *v));
            }
        }

        // Construct the complete tree.
        loop {
            if trees.len() == 1 {
                return trees.pop().expect("No trees");
            }

            let a = trees.pop().expect("No least tree");
            let b = trees.pop().expect("No second least tree");

            trees.push(HuffmanTree::Node {
                weight: a.weight() + b.weight(),
                zero: Box::new(a),
                one: Box::new(b),
            });
        }
    }

    /// Encode data from `input` and write it to `output`.
    pub fn encode_to<A, B>(&self, input: &mut A, output: &mut B) -> io::Result<usize>
    where
        A: Read,
        B: Write,
    {
        let mut n_bits = 0;
        let codes = self.codes();

        let mut buf = BitWriter::new(output);

        // Encode bytes
        for byte in input.bytes() {
            let code = &codes[byte? as usize];

            // Add bits to buffer
            for &b in code {
                buf.push(b)?;
                n_bits += 1;
            }
        }

        buf.flush()?;

        Ok(n_bits)
    }

    /// Return a map of the generated encodings
    pub fn codes(&self) -> Vec<Vec<bool>> {
        let mut char_to_code = vec![Vec::new(); u8::MAX as usize];
        self.codes_helper(&mut char_to_code, &mut Vec::new());
        char_to_code
    }

    fn codes_helper(&self, char_to_code: &mut Vec<Vec<bool>>, path: &mut Vec<bool>) {
        match self {
            Self::Leaf(c, _) => {
                char_to_code[*c as usize] = path.clone();
            }
            Self::Node { zero, one, .. } => {
                path.push(false);
                zero.codes_helper(char_to_code, path);
                path.pop();
                path.push(true);
                one.codes_helper(char_to_code, path);
                path.pop();
            }
        }
    }

    /// Decode data from `input` and write it to `output`.
    pub fn decode_to<A, B>(&self, input: &mut A, output: &mut B, n_bits: usize) -> io::Result<()>
    where
        A: Read,
        B: Write,
    {
        let mut bits_read = 0;
        let mut biterator = Biterator::new(input);

        while let Some((c, count)) = self.decode_one_to(&mut biterator, 0) {
            output.write_all(&[c])?;
            bits_read += count;
            if bits_read >= n_bits {
                break;
            }
        }

        Ok(())
    }

    /// Decode a single character from `input`.
    fn decode_one_to<A>(&self, input: &mut A, bits_read: usize) -> Option<(u8, usize)>
    where
        A: Iterator<Item = bool>,
    {
        match self {
            Self::Leaf(c, _) => Some((*c, bits_read)),
            Self::Node { zero, one, .. } => {
                if input.next()? {
                    one.decode_one_to(input, bits_read + 1)
                } else {
                    zero.decode_one_to(input, bits_read + 1)
                }
            }
        }
    }
}
