#[derive(Clone, Default)]
pub struct ChainStatus {
    pub block_num: u32,
    pub connection: ConnectionStatus,
}

#[derive(Clone, PartialEq, Eq)]
pub enum ConnectionStatus {
    Online,
    Offline,
}

impl Default for ConnectionStatus {
    fn default() -> Self {
        ConnectionStatus::Offline
    }
}
