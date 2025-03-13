use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use vigem_client::{Client, Xbox360Wired};

pub struct AppState {
    pub client: Arc<Client>,
    pub controller_ids: Mutex<Vec<String>>,
    pub virtual_targets: Mutex<HashMap<String, Xbox360Wired<Arc<Client>>>>,
}
