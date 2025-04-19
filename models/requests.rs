use serde::{Deserialize, Serialize};

use super::input::{StickInput, TriggerInput};

#[derive(Serialize, Deserialize)]
pub struct HandleActionRequest {
    pub controller_id: String,
    pub action_ids: Vec<String>,
    pub left_stick: StickInput,
    pub right_stick: StickInput,
    pub triggers: TriggerInput,
}
