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
