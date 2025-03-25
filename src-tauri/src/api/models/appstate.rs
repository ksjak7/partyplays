use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use vigem_client::Client;

use super::virtual_target::VirtualTarget;

pub struct AppState {
    pub client: Arc<Client>,
    pub controller_ids: Mutex<Vec<String>>,
    pub virtual_targets: Mutex<HashMap<String, VirtualTarget>>,
    pub binary_string_input_converter: Arc<HashMap<String, u16>>,
}
