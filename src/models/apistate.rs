use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use vigem_client::Client;

use super::virtual_target::VirtualTarget;

pub struct ApiState {
    pub client: Arc<Client>,
    pub controller_ids: RwLock<Vec<String>>,
    pub virtual_targets: RwLock<HashMap<String, VirtualTarget>>,
    pub binary_string_input_converter: Arc<HashMap<String, u16>>,
}
