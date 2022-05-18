pub struct DexiosFile {
    pub salt: [u8; SALT_LEN],
    pub nonce: [u8; 12],
    pub data: Vec<u8>,
}

pub const BLOCK_SIZE: usize = 1048576; // 1024*1024
pub const SALT_LEN: usize = 16; // bytes