use std::mem;
use std::time::{SystemTime, UNIX_EPOCH};

// terrible RNG
pub fn byte() -> u8 {
    let epochs: [u8; 16] = unsafe {
        mem::transmute(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        )
    };
    let i = (2 + epochs[1] % 3) as usize;
    let x = (229_u128 * epochs[0] as u128) ^ (227_u128 * epochs[i] as u128);
    (x & 0xff) as u8
}
