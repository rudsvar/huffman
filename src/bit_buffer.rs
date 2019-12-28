//! A module defining a buffer that can store bits.
//! This removes the need for manual bit-fiddling.

use std::io::{Read, Write};

/// The size of the internal bit-buffer.
const BUF_SIZE: usize = 10;

/// A struct containing the internal bit-buffer.
#[derive(Default)]
pub struct BitBuffer {
    buffer: [u8; BUF_SIZE],
    pos: usize,
    length: usize,
}

impl BitBuffer {
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

    /// Set the specified bit to the given value.
    pub fn set_bit(&mut self, idx: usize, value: bool) {
        let (byte_idx, bit_idx) = byte_bit_idx(idx);
        let mask = 1 << bit_idx;
        if value {
            self.buffer[byte_idx] |= mask;
        } else {
            self.buffer[byte_idx] &= !mask;
        }
        eprintln!("len {}: {:08b}", self.len(), self.buffer[byte_idx]);
    }

    /// Read the bit at the given location.
    pub fn get_bit(&self, idx: usize) -> bool {
        let (byte_idx, bit_idx) = byte_bit_idx(idx);
        let mask = 1 << bit_idx;
        self.buffer[byte_idx] & mask == mask
    }

    /// Push a new value into the buffer.
    pub fn push(&mut self, value: bool) {
        self.set_bit(self.length, value);
        self.length += 1;
    }

    /// Pop the last value pushed into the buffer.
    fn pop(&mut self) -> Option<bool> {
        if self.length == 0 {
            return None;
        }

        self.length -= 1;
        let value = self.get_bit(self.length);
        Some(value)
    }
}

impl Iterator for BitBuffer {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.len() {
            return None;
        }

        let value = self.get_bit(self.pos);
        self.pos += 1;
        Some(value)
    }
}

/// Convert the index to byte- and bit-index.
fn byte_bit_idx(idx: usize) -> (usize, usize) {
    let byte_idx = idx / 8;
    let bit_idx = 7 - idx % 8;
    (byte_idx, bit_idx)
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
