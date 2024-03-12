use wayland_client::{
    delegate_noop,
    protocol::{
        wl_output::{self, WlOutput},
        wl_registry::{self, WlRegistry},
    },
    Dispatch,
};
use wayland_protocols::xdg::xdg_output::zv1::client::{
    zxdg_output_manager_v1::ZxdgOutputManagerV1, zxdg_output_v1::ZxdgOutputV1,
};

#[derive(Debug, Clone)]
pub struct OutputInfo {
    pub output: WlOutput,
    pub name: String,
    pub description: String,
}

pub struct OutputState {
    pub outputs: Vec<OutputInfo>,
}

impl Dispatch<WlRegistry, ()> for OutputState {
    fn event(
        state: &mut Self,
        proxy: &WlRegistry,
        event: <WlRegistry as wayland_client::Proxy>::Event,
        _data: &(),
        _conn: &wayland_client::Connection,
        qhandle: &wayland_client::QueueHandle<Self>,
    ) {
        if let wl_registry::Event::Global {
            name,
            interface,
            version,
        } = event
        {
            if interface == "wl_output" {
                if version >= 4 {
                    let output = proxy.bind::<WlOutput, _, _>(name, 4, qhandle, ());
                    state.outputs.push(OutputInfo {
                        output,
                        name: String::new(),
                        description: String::new(),
                    });
                } else {
                    tracing::error!("Ignoring a wl_output with version < 4.");
                }
            }
            //tracing::debug!("WlRegistry global event for interface: {}", interface);
        }
    }
}

impl Dispatch<WlOutput, ()> for OutputState {
    fn event(
        state: &mut Self,
        proxy: &WlOutput,
        event: <WlOutput as wayland_client::Proxy>::Event,
        _data: &(),
        _conn: &wayland_client::Connection,
        _qhandle: &wayland_client::QueueHandle<Self>,
    ) {
        let output: &mut OutputInfo = state
            .outputs
            .iter_mut()
            .find(|x| x.output == *proxy)
            .unwrap();

        match event {
            wl_output::Event::Name { name } => output.name = name,
            wl_output::Event::Description { description } => output.description = description,
            _ => {}
        }
    }
}

delegate_noop!(OutputState: ignore ZxdgOutputManagerV1);
delegate_noop!(OutputState: ignore ZxdgOutputV1);
