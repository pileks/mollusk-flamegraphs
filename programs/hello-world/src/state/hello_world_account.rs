use bytemuck::{Pod, Zeroable};
use shank::ShankAccount;

#[repr(C)]
#[derive(Pod, Zeroable, Clone, Copy, ShankAccount)]
pub struct HelloWorldAccount {
    #[idl_type("Key")]
    pub key: u8,
    pub padding: [u8; 7],
    pub a: u64,
    pub b: u64,
}

impl HelloWorldAccount {
    pub const BASE_LEN: usize = size_of::<Self>();
}
