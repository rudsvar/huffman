use serde::{Deserialize, Serialize};
use std::cmp;
use std::collections::BinaryHeap;
use std::collections::HashMap;

#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize)]
pub enum HuffmanTree {
    Node {
        zero: Box<HuffmanTree>,
        one: Box<HuffmanTree>,
    },
    Leaf(char, usize),
}

impl<'a> Ord for HuffmanTree {
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
    pub fn weight(&self) -> usize {
        match self {
            Self::Node { zero, one } => zero.weight() + one.weight(),
            Self::Leaf(_, w) => *w,
        }
    }

    pub fn from(counts: &HashMap<char, usize>) -> HuffmanTree {
        let mut trees: BinaryHeap<HuffmanTree> = BinaryHeap::new();

        // If there is only one character, a special tree is made.
        if counts.len() == 1 {
            let (&k, &v) = counts.iter().nth(0).unwrap();
            return HuffmanTree::Node {
                zero: Box::new(HuffmanTree::Leaf(k, v)),
                one: Box::new(HuffmanTree::Leaf(k, v)),
            };
        }

        // Add the initial trees.
        for (k, v) in counts {
            trees.push(Self::Leaf(*k, *v));
        }

        // Construct the complete tree.
        loop {
            if trees.len() == 1 {
                return trees.pop().expect("No trees");
            }

            let a = trees.pop().expect("No least tree");
            let b = trees.pop().expect("No second least tree");

            trees.push(HuffmanTree::Node {
                zero: Box::new(a),
                one: Box::new(b),
            });
        }
    }

    pub fn encode(&self, input: &str) -> Option<(Vec<u8>, usize)> {
        let mut output = Vec::new();
        let mut idx = 0;
        let codes = self.codes();
        for chr in input.chars() {
            let code = codes.get(&chr)?;
            for &c in code {
                set_bit(&mut output, idx, c);
                idx += 1
            }
        }
        Some((output, idx))
    }

    /// Return a map of the generated encodings
    pub fn codes(&self) -> HashMap<char, Vec<bool>> {
        let mut char_to_code = HashMap::new();
        self.codes_helper(&mut char_to_code, &mut Vec::new());
        char_to_code
    }

    fn codes_helper(&self, char_to_code: &mut HashMap<char, Vec<bool>>, path: &mut Vec<bool>) {
        match self {
            Self::Leaf(c, __) => {
                char_to_code.insert(*c, path.clone());
            }
            Self::Node { zero, one } => {
                path.push(false);
                zero.codes_helper(char_to_code, path);
                path.pop();
                path.push(true);
                one.codes_helper(char_to_code, path);
                path.pop();
            }
        }
    }

    pub fn decode(&self, input: &[u8], n_bits: usize) -> Option<String> {
        let mut res = String::new();
        let mut idx = 0;
        while idx < n_bits {
            let (c, new_idx) = self.decode_one(input, n_bits, idx)?;
            idx = new_idx;
            res.push(c);
        }
        Some(res)
    }

    fn decode_one(&self, input: &[u8], n_bits: usize, idx: usize) -> Option<(char, usize)> {
        match self {
            Self::Leaf(c, _) => Some((*c, idx)),
            Self::Node { zero, one } => match get_bit(input, idx) {
                false => zero.decode_one(input, n_bits, idx + 1),
                true => one.decode_one(input, n_bits, idx + 1),
            },
        }
    }
}

/// Set the bit at index `idx`.
/// The vector will grow as necessary.
///
pub fn set_bit(buf: &mut Vec<u8>, idx: usize, value: bool) {
    let byte_idx = idx / 8;
    let bit_idx = 7 - idx % 8;
    while !(byte_idx < buf.len()) {
        buf.push(0);
    }
    let mask = 1 << bit_idx;
    if value {
        buf[byte_idx] |= mask;
    } else {
        buf[byte_idx] &= !mask;
    }
}

/// Get the bit at index `idx`.
///
pub fn get_bit(buf: &[u8], idx: usize) -> bool {
    let byte_idx = idx / 8;
    let bit_idx = 7 - idx % 8;
    (buf[byte_idx] & (1 << bit_idx)) == (1 << bit_idx)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn set_bit_1() {
        let mut buf = vec![];
        set_bit(&mut buf, 0, true);
        set_bit(&mut buf, 2, true);
        set_bit(&mut buf, 9, true);

        assert!(get_bit(&buf, 0));
        assert!(!get_bit(&buf, 1));
        assert!(get_bit(&buf, 2));
        assert!(get_bit(&buf, 9));
    }

    #[test]
    fn set_bit_2() {
        let mut buf = vec![255];
        set_bit(&mut buf, 0, false);
        set_bit(&mut buf, 2, false);
        set_bit(&mut buf, 9, false);

        assert!(!get_bit(&buf, 0));
        assert!(get_bit(&buf, 1));
        assert!(!get_bit(&buf, 2));
        assert!(!get_bit(&buf, 9));
    }

    #[test]
    fn get_bit_test() {
        let buf = vec![5];

        assert!(get_bit(&buf, 7));
        assert!(!get_bit(&buf, 6));
        assert!(get_bit(&buf, 5));
        assert!(!get_bit(&buf, 4));
    }
}
