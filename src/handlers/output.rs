use smithay_client_toolkit::{
    output::{OutputHandler, OutputState},
    reexports::client::{protocol::wl_output, Connection, QueueHandle},
};

use crate::app::AppData;

impl OutputHandler for AppData {
    fn output_state(&mut self) -> &mut OutputState {
        &mut self.states.output_state
    }

    fn new_output(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        output: wl_output::WlOutput,
    ) {
        let output_info = self.states.output_state.info(&output).unwrap();
        tracing::trace!("New Output {:#?}", output_info);
    }

    fn update_output(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _output: wl_output::WlOutput,
    ) {
        tracing::trace!("Update Output");
    }

    fn output_destroyed(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _output: wl_output::WlOutput,
    ) {
        tracing::trace!("Destroyed Output");
    }
}

smithay_client_toolkit::delegate_output!(AppData);
