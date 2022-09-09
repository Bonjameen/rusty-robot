use std::collections::VecDeque;

pub fn byte_slice_to_vecdeque(bytes: &[u8]) -> VecDeque<u8> {
    let mut vecd = VecDeque::new();
    for byte in bytes {
        vecd.push_back(*byte)
    }
    return vecd;
}
