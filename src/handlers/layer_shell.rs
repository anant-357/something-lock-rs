use smithay_client_toolkit::shell::wlr_layer::LayerShellHandler;

use crate::app_data::AppData;

impl LayerShellHandler for AppData {
    fn closed(
        &mut self,
        _conn: &smithay_client_toolkit::reexports::client::Connection,
        _qh: &smithay_client_toolkit::reexports::client::QueueHandle<Self>,
        _layer: &smithay_client_toolkit::shell::wlr_layer::LayerSurface,
    ) {
    }

    fn configure(
        &mut self,
        _conn: &smithay_client_toolkit::reexports::client::Connection,
        _qh: &smithay_client_toolkit::reexports::client::QueueHandle<Self>,
        _layer: &smithay_client_toolkit::shell::wlr_layer::LayerSurface,
        _configure: smithay_client_toolkit::shell::wlr_layer::LayerSurfaceConfigure,
        _serial: u32,
    ) {
    }
}

smithay_client_toolkit::delegate_layer!(AppData);
