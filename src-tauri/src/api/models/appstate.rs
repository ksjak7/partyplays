use std::sync::Mutex;

use vigem_client::{Client, Xbox360Wired};

pub struct AppState {
    pub controller_ids: Mutex<Vec<String>>,
    pub virtual_targets: Mutex<Vec<Xbox360Wired<Client>>>,
}
