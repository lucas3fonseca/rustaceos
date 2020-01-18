use serde::{Serialize, Deserialize};

use super::name::Name;

#[derive(Serialize, Deserialize, Debug)]
pub struct PermissionLevel {
    pub actor: Name,
    pub permission: Name,
}
