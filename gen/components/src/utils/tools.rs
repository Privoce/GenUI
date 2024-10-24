use makepad_widgets::LiveId;

pub const LIVE_ID_SEED: u64 = 0xd6e8_feb8_6659_fd93;

pub const fn from_bytes(seed: u64, id_bytes: &[u8], start: usize, end: usize) -> u64 {
    let mut x = seed;
    let mut i = start;
    while i < end {
        x = x.overflowing_add(id_bytes[i] as u64).0;
        x ^= x >> 32;
        x = x.overflowing_mul(0xd6e8_feb8_6659_fd93).0;
        x ^= x >> 32;
        x = x.overflowing_mul(0xd6e8_feb8_6659_fd93).0;
        x ^= x >> 32;
        i += 1;
    }
    // mark high bit as meaning that this is a hash id
    (x & 0x7fff_ffff_ffff_ffff) | 0x8000_0000_0000_0000
}

pub const fn from_str_unchecked(id_str: &str) -> u64 {
    let bytes = id_str.as_bytes();
    from_bytes(LIVE_ID_SEED, bytes, 0, bytes.len())
}

pub fn round_to_two_decimals(value: f64) -> f64 {
    (value * 10000.0).round() / 10000.0
}

pub trait LiveIdGenerate {
    fn to_live_id(self) -> LiveId;
}

impl LiveIdGenerate for usize {
    fn to_live_id(self) -> LiveId {
        LiveId(self as u64)
    }
}