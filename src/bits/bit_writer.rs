//! A module defining a buffer for pushing bits into,
//! which will then be flushed to the destination.

use crate::bits;
use std::io::{self, Write};

/// The size of the internal bit-buffer.
const BUF_SIZE: usize = 2048;

/// A struct containing the internal bit-buffer.
pub struct BitWriter<'a> {
    buffer: [u8; BUF_SIZE],
    pos: usize,
    dst: &'a mut dyn Write,
}

impl<'a> BitWriter<'a> {
    /// Construct a new `BitWriter` which will read from the source
    /// `src` and write to the destination `dst`.
    pub fn new(dst: &'a mut dyn Write) -> BitWriter<'a> {
        BitWriter {
            buffer: [0; BUF_SIZE],
            pos: 0,
            dst,
        }
    }

    /// Push a new value into the buffer.
    pub fn push(&mut self, value: bool) -> io::Result<()> {
        // Set bit and increment position
        bits::set_bit(&mut self.buffer, self.pos, value);
        self.pos += 1;

        // Flush buffer when full
        if self.pos >= 8 * BUF_SIZE {
            self.flush()?;
        }

        Ok(())
    }

    pub fn flush(&mut self) -> io::Result<()> {
        let mut bytes_used = self.pos / 8;
        if self.pos % 8 != 0 {
            bytes_used += 1;
        }
        self.dst.write_all(&self.buffer[0..bytes_used])?;
        self.pos = 0;

        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn writes_expected() {
        // Create buffer and output vector
        let mut output = vec![];
        let mut buf = BitWriter::new(&mut output);
        let input: Vec<bool> = [
            [false, false, true, false, true, false, false, true],
            [false, false, false, false, false, true, false, true],
            [true, true, true, true, true, false, true, true],
        ]
        .iter()
        .flatten()
        .copied()
        .collect();

        // Push it into the bit buffer
        for &i in &input {
            buf.push(i).expect("Could not push");
        }
        buf.flush().expect("Could not flush");

        // Check the output
        let expected = vec![0b0010_1001, 0b0000_0101, 0b1111_1011];
        assert_eq!(output, expected);
    }
}
