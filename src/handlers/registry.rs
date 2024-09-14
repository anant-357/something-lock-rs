use smithay_client_toolkit::{
    output::OutputState,
    registry::{ProvidesRegistryState, RegistryState},
    registry_handlers,
};
use wayland_client::{
    delegate_noop, protocol::{wl_compositor, wl_registry}, Connection, Dispatch, QueueHandle
};

use crate::app::AppData;

impl ProvidesRegistryState for AppData {
    fn registry(&mut self) -> &mut RegistryState {
        &mut self.wayland.registry_state
    }
    registry_handlers![OutputState,];
}

impl Dispatch<wl_registry::WlRegistry, ()> for AppData {
    fn event(
        state: &mut Self,
        registry: &wl_registry::WlRegistry,
        event: wl_registry::Event,
        _: &(),
        _conn: &Connection,
        qh: &QueueHandle<Self>,
    ) {
        if let wl_registry::Event::Global {
            name,
            interface,
            version,
        } = event
        {
            match &interface[..] {
                "wl_compositor" => {
                    tracing::trace!("New Compositor: {}, {}, {}", name, interface, version);
                    let compositor =
                        registry.bind::<wl_compositor::WlCompositor, _, _>(name, 1, qh, ());
                    state.wayland.compositor = Some(compositor);
                }
                _ => {}
            }
        }
    }
}

delegate_noop!(AppData: ignore wl_compositor::WlCompositor);

smithay_client_toolkit::delegate_registry!(AppData);
