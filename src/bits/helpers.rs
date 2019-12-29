/// Convert the index to byte- and bit-index.
fn byte_idx_and_mask(idx: usize) -> (usize, u8) {
    let byte_idx = idx / 8;
    let bit_idx = 7 - idx % 8;
    let mask = 1 << bit_idx;
    (byte_idx, mask)
}

/// Set the specified bit to the given value.
pub fn set_bit(buf: &mut [u8], idx: usize, value: bool) {
    let (byte_idx, mask) = byte_idx_and_mask(idx);
    if value {
        buf[byte_idx] |= mask;
    } else {
        buf[byte_idx] &= !mask;
    }
}

/// Read the bit at the given location.
pub fn get_bit(buf: &[u8], idx: usize) -> bool {
    let (byte_idx, mask) = byte_idx_and_mask(idx);
    buf[byte_idx] & mask == mask
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn set_bit_1() {
        let mut buf = [0; 2];
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
        let mut buf = [255; 2];
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
