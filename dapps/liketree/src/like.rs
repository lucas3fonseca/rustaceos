use eosio_cdt::eos;

#[derive(eos::Serialize, eos::Deserialize)]
pub struct Like {
    id: u64,
    like: bool,
    likes: u32,
    parent_like_id: u64,
    relike_id: u64,
    content: eos::Checksum256,
}
