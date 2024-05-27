use std::process::Command;

use wayland_client::protocol::wl_compositor::WlCompositor;
use wayland_client::protocol::wl_registry::Event;
use wayland_client::protocol::wl_seat::WlSeat;
use wayland_client::protocol::wl_surface::WlSurface;
use wayland_client::{delegate_noop, Connection};
use wayland_client::{
    globals::GlobalListContents, protocol::wl_registry::WlRegistry, Dispatch, Proxy,
};
use wayland_protocols::ext::session_lock::v1::client::ext_session_lock_manager_v1::ExtSessionLockManagerV1;
use wayland_protocols::ext::session_lock::v1::client::ext_session_lock_surface_v1::ExtSessionLockSurfaceV1;
use wayland_protocols::ext::session_lock::v1::client::ext_session_lock_v1;
use wayland_protocols::ext::session_lock::v1::client::ext_session_lock_v1::ExtSessionLockV1;
use wayland_protocols::xdg::shell::client::xdg_surface::XdgSurface;
use wayland_protocols::xdg::shell::client::xdg_toplevel::XdgToplevel;
use wayland_protocols::xdg::shell::client::xdg_wm_base::XdgWmBase;

use crate::lockgtk::LockGTK;
use gtk4::prelude::*;

pub struct LockState {
    pub locked: bool,
    pub base_surface: Option<WlSurface>,
    pub session_lock_surfaces: Vec<ExtSessionLockSurfaceV1>,
    pub wm_base: Option<XdgWmBase>,
    pub xdg_surface: Option<(XdgSurface, XdgToplevel)>,
    pub gtk_lock: Option<LockGTK>,
}

impl Dispatch<WlRegistry, GlobalListContents> for LockState {
    fn event(
        _state: &mut Self,
        _proxy: &WlRegistry,
        _event: <WlRegistry as Proxy>::Event,
        _data: &GlobalListContents,
        _conn: &Connection,
        _qhandle: &wayland_client::QueueHandle<Self>,
    ) {
        tracing::debug!("Dispatched WlRegistry, GlobalListContents For LockState");
    }
}
impl Dispatch<WlRegistry, ()> for LockState {
    fn event(
        state: &mut Self,
        proxy: &WlRegistry,
        event: <WlRegistry as Proxy>::Event,
        _data: &(),
        _conn: &Connection,
        qhandle: &wayland_client::QueueHandle<Self>,
    ) {
        if let Event::Global {
            name, interface, ..
        } = event
        {
            match &interface[..] {
                "wl_compositor" => {
                    let compositor = proxy.bind::<WlCompositor, _, _>(name, 1, qhandle, ());
                    tracing::debug!(
                        "Created Compositor from WlRegistry Event of LockState compositor: {:#?}",
                        compositor
                    );
                    let surface = compositor.create_surface(qhandle, ());
                    state.base_surface = Some(surface);
                    tracing::debug!(
                        "Created Base surface from WlRegistry Event of LockState surface: {:#?}",
                        state.base_surface
                    );
                }
                "wl_seat" => {
                    proxy.bind::<WlSeat, _, _>(name, 1, qhandle, ());
                }
                "xdg_wm_base" => {
                    let wm_base = proxy.bind::<XdgWmBase, _, _>(name, 6, qhandle, ());
                    state.wm_base = Some(wm_base);

                    if state.base_surface.is_some() && state.xdg_surface.is_none() {}
                }
                _ => {
                    //tracing::debug!("Dispatched WlRegistry For LockState, Event: {:#?}, interface: {:#?}",name,interface)
                }
            }
        }
    }
}

impl Dispatch<ExtSessionLockManagerV1, ()> for LockState {
    fn event(
        _state: &mut Self,
        _proxy: &ExtSessionLockManagerV1,
        _event: <ExtSessionLockManagerV1 as Proxy>::Event,
        _data: &(),
        _: &Connection,
        _: &wayland_client::QueueHandle<Self>,
    ) {
        tracing::debug!("Dispatched ExtSessionLockManagerV1 For LockState");
    }
}

impl Dispatch<ExtSessionLockV1, ()> for LockState {
    fn event(
        state: &mut Self,
        _proxy: &ExtSessionLockV1,
        event: <ExtSessionLockV1 as Proxy>::Event,
        _data: &(),
        _: &Connection,
        _: &wayland_client::QueueHandle<Self>,
    ) {
        tracing::debug!("Dispatched ExtSessionLockV1 For LockState");
        match event {
            ext_session_lock_v1::Event::Locked => {
                //let lock = LockGTK::create();
                Command::new("swaylock");
                tracing::debug!("Event Locked fired");
            }
            ext_session_lock_v1::Event::Finished => {
                tracing::debug!(
                    "Event Finished fired: Either session_lock is already there or it is being closed by client"
                );
            }
            _ => {}
        }
    }
}

delegate_noop!(LockState: ignore ExtSessionLockSurfaceV1);
delegate_noop!(LockState: ignore WlCompositor);
delegate_noop!(LockState: ignore WlSurface);
delegate_noop!(LockState: ignore WlSeat);
delegate_noop!(LockState: ignore XdgWmBase);
