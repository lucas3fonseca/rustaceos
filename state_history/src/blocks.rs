use abieos::Checksum256;

#[derive(Debug)]
pub struct BlockPosition {
    pub block_num: u32,
    pub block_id: Checksum256,
}
