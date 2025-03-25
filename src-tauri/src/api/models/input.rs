use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct StickInput {
    pub x: i16,
    pub y: i16,
}

#[derive(Serialize, Deserialize)]
pub struct TriggerInput {
    pub left: u8,
    pub right: u8,
}
