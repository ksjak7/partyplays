use std::sync::Arc;
use vigem_client::{Client, XGamepad, Xbox360Wired};

pub struct VirtualTarget {
    pub controller: Xbox360Wired<Arc<Client>>,
    pub state: XGamepad,
}
