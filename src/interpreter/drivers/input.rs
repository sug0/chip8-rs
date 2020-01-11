use std::ops;
use std::mem;
use std::convert::TryFrom;

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum Key {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    A,
    B,
    C,
    D,
    E,
    F,
}

impl TryFrom<u8> for Key {
    type Error = &'static str;

    fn try_from(k: u8) -> Result<Key, Self::Error> {
        if k < 16 {
            unsafe { Ok(mem::transmute(k)) }
        } else {
            Err("key out of range")
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct KeySet([bool; 16]);

impl ops::Index<Key> for KeySet {
    type Output = bool;

    fn index(&self, k: Key) -> &bool {
        &self.0[k as usize]
    }
}

pub trait Input {
    fn poll_keyboard(&mut self) -> KeySet;
    fn wait_key(&mut self) -> Key;
}

impl Input for () {
    fn poll_keyboard(&mut self) -> KeySet {
        KeySet([false; 16])
    }

    fn wait_key(&mut self) -> Key {
        loop {}
    }
}
