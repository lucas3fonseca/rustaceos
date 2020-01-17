use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PermissionLevel {
    pub actor: u64,      // name
    pub permission: u64, // name
}
