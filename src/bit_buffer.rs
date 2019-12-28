//! A module defining a buffer that can store bits.
//! This removes the need for manual bit-fiddling.

use crate::bit_helpers;

/// The size of the internal bit-buffer.
const BUF_SIZE: usize = 10;

/// A struct containing the internal bit-buffer.
pub struct BitBuffer {
    buffer: [u8; BUF_SIZE],
    pos: usize,
    length: usize,
}

impl BitBuffer {
    /// Construct a new `BitBuffer` which will read from the source
    /// `src` and write to the destination `dst`.
    pub fn new() -> BitBuffer {
        BitBuffer {
            buffer: [0; BUF_SIZE],
            length: 0,
            pos: 0,
        }
    }

    /// Get the amount of bits in the buffer.
    pub fn len(&self) -> usize {
        self.length
    }

    /// Check if nothing has been stored in the buffer.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Push a new value into the buffer.
    pub fn push(&mut self, value: bool) {
        bit_helpers::set_bit(&mut self.buffer, self.length, value);
        self.length += 1;
    }

    /// Pop the last value pushed into the buffer.
    fn pop(&mut self) -> Option<bool> {
        if self.length == 0 {
            return None;
        }

        self.length -= 1;
        let value = bit_helpers::get_bit(&self.buffer, self.length);
        Some(value)
    }
}

impl Iterator for BitBuffer {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.len() {
            return None;
        }

        let value = bit_helpers::get_bit(&self.buffer, self.pos);
        self.pos += 1;
        Some(value)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn push() {
        let mut buf = BitBuffer::new();
        let input = vec![true, false, true, true];
        for &i in &input {
            buf.push(i);
        }
    }

    #[test]
    fn pop_empty() {
        let mut buf = BitBuffer::new();
        assert!(buf.pop().is_none())
    }

    #[test]
    fn push_pop() {
        let mut buf = BitBuffer::new();

        let input = vec![true, true, false, true];
        for &i in &input {
            buf.push(i);
        }

        let output: Vec<bool> = buf.collect();

        assert_eq!(input, output)
    }
}
