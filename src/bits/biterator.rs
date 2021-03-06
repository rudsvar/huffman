use crate::bits;
use std::io::Read;

/// The size of the internal bit-buffer.
const BUF_SIZE: usize = 2048;

/// A struct for reading bits from a `Read`.
///
/// The `Biterator` wraps anything with `impl Read`, and
/// allows the user to iterate over the the bits it contains.
pub struct Biterator<'a> {
    buffer: [u8; BUF_SIZE],
    length: usize,
    pos: usize,
    source: &'a mut dyn Read,
}

impl<'a> Biterator<'a> {
    /// Construct a new `Biterator` which will read from the source
    /// `source` and let you iterate over the bits it contains.
    pub fn new(source: &'a mut dyn Read) -> Biterator {
        Biterator {
            buffer: [0; BUF_SIZE],
            length: 0,
            pos: 0,
            source,
        }
    }
}

impl<'a> Iterator for Biterator<'a> {
    type Item = bool;
    fn next(&mut self) -> Option<Self::Item> {
        // If we run out of data, read more
        if self.pos == self.length {
            match self.source.read(&mut self.buffer) {
                Err(e) => panic!("{}", e),
                Ok(0) => return None,
                Ok(n) => {
                    self.length = 8 * n;
                    self.pos = 0;
                }
            }
            self.pos = 0;

            if self.length == 0 {
                return None;
            }
        }

        let bit = bits::get_bit(&self.buffer, self.pos);
        self.pos += 1;

        Some(bit)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn biterate() {
        let mut input = std::io::Cursor::new([41, 5, 251]);
        let output: Vec<bool> = Biterator::new(&mut input).collect();
        let expected: Vec<bool> = [
            [false, false, true, false, true, false, false, true],
            [false, false, false, false, false, true, false, true],
            [true, true, true, true, true, false, true, true],
        ]
        .iter()
        .flatten()
        .copied()
        .collect();
        assert_eq!(output, expected);
    }
}
