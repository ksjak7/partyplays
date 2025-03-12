use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct CreateControllersRequest {
  pub number_of_controllers: u8,
}

#[derive(Serialize, Deserialize)]
pub struct HandleActionRequest {
  pub controller_id: String,
  pub action_id: String,
}