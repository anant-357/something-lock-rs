use smithay_client_toolkit::{
    output::OutputState,
    registry::{ProvidesRegistryState, RegistryState},
    registry_handlers,
};

use crate::app_data::AppData;

impl ProvidesRegistryState for AppData {
    fn registry(&mut self) -> &mut RegistryState {
        &mut self.registry_state
    }
    registry_handlers![OutputState,];
}
smithay_client_toolkit::delegate_registry!(AppData);
