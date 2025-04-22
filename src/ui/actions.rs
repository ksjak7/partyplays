use crate::shared_models::{
    error::Error, shared_state::SharedState, virtual_target::VirtualTarget,
};
use nanoid::nanoid;
use std::sync::Arc;
use vigem_client::{TargetId, XGamepad, Xbox360Wired};

pub fn create_controllers(
    state: Arc<SharedState>,
    number_of_controllers: usize,
) -> Result<(), Error> {
    clear_targets_and_controllers(state.clone())?;

    let mut virtual_targets = state.virtual_targets.write()?;

    let mut controller_ids: Vec<String> = Vec::with_capacity(number_of_controllers);
    for _ in 0..number_of_controllers {
        let controller_id = nanoid!(6);
        let new_target = Xbox360Wired::new(state.client.clone(), TargetId::XBOX360_WIRED);

        controller_ids.push(controller_id.clone());
        virtual_targets.insert(
            controller_id,
            VirtualTarget {
                controller: new_target,
                state: XGamepad::default(),
                ui_buttons_pressed: u16::default(),
            },
        );
    }

    for target in virtual_targets.values_mut() {
        if let Err(e) = target.controller.plugin() {
            virtual_targets.clear();
            return Err(Error::from(e));
        };
    }

    let mut writable_controller_ids = state.controller_ids.write()?;
    writable_controller_ids.append(&mut controller_ids);
    Ok(())
}

pub fn clear_targets_and_controllers(state: Arc<SharedState>) -> Result<(), Error> {
    let mut virtual_targets = state.virtual_targets.write()?;
    virtual_targets.clear();

    let mut controller_ids = state.controller_ids.write()?;
    controller_ids.clear();
    Ok(())
}
