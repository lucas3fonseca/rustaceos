use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TimePointSec {
    utc_seconds: u32,
}
