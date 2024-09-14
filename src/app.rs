use crate::graphics::Graphics;
use crate::lock::LockState;
use crate::media::Media;
use smithay_client_toolkit::{
    compositor::CompositorState,
    output::OutputState,
    reexports::{
        calloop::{EventLoop as CEventLoop, LoopHandle},
        client::{
            globals::registry_queue_init, protocol::wl_keyboard::WlKeyboard, Connection,
            QueueHandle,
        },
    },
    registry::RegistryState,
    seat::SeatState,
    session_lock::SessionLockState,
};
use wayland_client::protocol::wl_compositor;
use xdg::BaseDirectories;

use crate::config::Config;

pub struct Wayland {
    pub conn: Connection,
    pub compositor: Option<wl_compositor::WlCompositor>,
    pub compositor_state: CompositorState,
    pub registry_state: RegistryState,
    pub output_state: OutputState,
    pub seat_state: SeatState,
    pub keyboard: Option<WlKeyboard>,
}

pub struct AppData {
    pub xdg: BaseDirectories,
    pub loop_handle: LoopHandle<'static, Self>,
    pub wayland: Wayland,
    pub graphics_context: Graphics,
    pub lock_data: LockState,
    pub media: Media,
    pub _config: Config,
    pub exit: bool,
}

impl AppData {
    pub fn connect(config: Config, base: BaseDirectories) {
        let conn = Connection::connect_to_env().unwrap();

        let (globals, mut event_queue) = registry_queue_init(&conn).unwrap();
        let qh: QueueHandle<AppData> = event_queue.handle();
        let event_loop: CEventLoop<AppData> =
            CEventLoop::try_new().expect("Failed to initialize the event loop!");
        let mut app_data = AppData {
            xdg: base,
            loop_handle: event_loop.handle(),
            wayland: Wayland {
                conn: conn.clone(),
                compositor: None,
                compositor_state: CompositorState::bind(&globals, &qh).unwrap(),
                registry_state: RegistryState::new(&globals),
                output_state: OutputState::new(&globals, &qh),
                seat_state: SeatState::new(&globals, &qh),
                keyboard: None,
            },
            graphics_context: Graphics::new(),
            lock_data: LockState::from_lock(
                SessionLockState::new(&globals, &qh)
                    .lock(&qh)
                    .expect("ext-session-lock not supported"),
            ),
            media: Media::from_config(&config),
            _config: config,
            exit: false,
        };
        tracing::trace!("Initiating lock");
        conn.roundtrip().unwrap();
        loop {
            event_queue.blocking_dispatch(&mut app_data).unwrap();

            if app_data.exit {
                break;
            }
        }
    }
}

smithay_client_toolkit::reexports::client::delegate_noop!(AppData: ignore smithay_client_toolkit::reexports::client::protocol::wl_buffer::WlBuffer);
