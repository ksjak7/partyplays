use super::virtual_target::VirtualTarget;
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};
use vigem_client::{Client, XButtons};
pub struct SharedState {
    pub client: Arc<Client>,
    pub controller_ids: RwLock<Vec<String>>,
    pub virtual_targets: RwLock<HashMap<String, VirtualTarget>>,
    pub binary_string_input_converter: Arc<HashMap<String, u16>>,
}

impl SharedState {
    pub fn new_arc() -> Arc<Self> {
        let client = Client::connect().unwrap();

        let binary_string_input_converter: HashMap<String, u16> = HashMap::from([
            (String::from("a"), XButtons::A),
            (String::from("b"), XButtons::B),
            (String::from("x"), XButtons::X),
            (String::from("y"), XButtons::Y),
            (String::from("dpad_left"), XButtons::LEFT),
            (String::from("dpad_up"), XButtons::UP),
            (String::from("dpad_down"), XButtons::DOWN),
            (String::from("dpad_right"), XButtons::RIGHT),
            (String::from("back"), XButtons::BACK),
            (String::from("start"), XButtons::START),
            (String::from("lb"), XButtons::LB),
            (String::from("rb"), XButtons::RB),
            (String::from("ls"), XButtons::LTHUMB),
            (String::from("rs"), XButtons::RTHUMB),
        ]);

        Arc::new(SharedState {
            client: Arc::new(client),
            controller_ids: RwLock::new(Vec::new()),
            virtual_targets: RwLock::new(HashMap::new()),
            binary_string_input_converter: Arc::new(binary_string_input_converter),
        })
    }
}
