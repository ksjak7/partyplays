use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use vigem_client::{Client, XGamepad, Xbox360Wired};

pub struct AppState {
    pub client: Arc<Client>,
    pub controller_ids: Mutex<Vec<String>>,
    pub virtual_targets: Mutex<HashMap<String, Xbox360Wired<Arc<Client>>>>,
    pub binary_string_input_converter: Arc<HashMap<String, u16>>,
    pub gamepad_off: Arc<XGamepad>,
}
