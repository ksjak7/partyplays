use std::thread;

mod api;
mod application;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn start() {
  thread::spawn(api::init::run);
  application::init::run();
}