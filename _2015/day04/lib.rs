use md5::compute;
use std::ops::RangeFrom;

pub const KEY: &str = "ckczppom";

pub struct Md5HexIter {
    key: String,
    range_from: RangeFrom<u32>,
    pub current: u32,
}

impl Md5HexIter {
    pub fn new(key: String, range_from: RangeFrom<u32>) -> Self {
        Self {
            key,
            range_from,
            current: 0,
        }
    }
}

impl Iterator for Md5HexIter {
    type Item = [u8; 16];

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next) = self.range_from.next() {
            self.current = next;
            Some(compute(format!("{}{}", self.key, next)).0)
        } else {
            None
        }
    }
}
