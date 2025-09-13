use borsh::{BorshDeserialize, BorshSerialize};

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, Clone, Copy)]
pub enum Key {
    Uninitialized = 0,
}

impl From<u8> for Key {
    fn from(value: u8) -> Self {
        match value {
            0 => Key::Uninitialized,
            _ => panic!("Invalid key"),
        }
    }
}

impl From<Key> for u8 {
    fn from(val: Key) -> Self {
        val as u8
    }
}
