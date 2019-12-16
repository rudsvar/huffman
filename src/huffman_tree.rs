use std::cmp;
use std::collections::BinaryHeap;
use std::collections::HashMap;

#[derive(PartialEq, Eq, Debug, Clone)]
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

    pub fn from(counts: &HashMap<char, usize>) -> Option<HuffmanTree> {
        let mut trees: BinaryHeap<HuffmanTree> = BinaryHeap::new();

        for (k, v) in counts {
            trees.push(Self::Leaf(*k, *v));
        }

        loop {
            if trees.len() <= 1 {
                return trees.pop();
            }

            let a = trees.pop().expect("No least tree");
            let b = trees.pop().expect("No second least tree");

            trees.push(HuffmanTree::Node {
                zero: Box::new(a),
                one: Box::new(b),
            });
        }
    }

    pub fn encode_char(&self, c: char) -> Option<String> {
        match self {
            Self::Leaf(x, _) if *x == c => Some(String::new()),
            Self::Leaf(_, _) => None,
            Self::Node { zero, one } => {
                let left = zero.encode_char(c).map(|s| String::from("0") + &s);
                let right = one.encode_char(c).map(|s| String::from("1") + &s);
                left.or(right)
            }
        }
    }
}
