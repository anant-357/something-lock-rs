use crate::graphics::{GContext, GSurfaceWrapper};
use crate::lock_data::LockData;
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
use xdg::BaseDirectories;

use crate::conf::Config;

pub struct States {
    pub compositor_state: CompositorState,
    pub registry_state: RegistryState,
    pub output_state: OutputState,
    pub seat_state: SeatState,
}

pub struct AppData {
    pub xdg: BaseDirectories,
    pub conn: Connection,
    pub loop_handle: LoopHandle<'static, Self>,
    pub states: States,
    pub graphics_context: Option<GContext>,
    pub keyboard: Option<WlKeyboard>,
    pub lock_data: LockData,
    pub gsurfaces: Vec<GSurfaceWrapper>,
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
            conn: conn.clone(),
            loop_handle: event_loop.handle(),
            states: States {
                compositor_state: CompositorState::bind(&globals, &qh).unwrap(),
                registry_state: RegistryState::new(&globals),
                output_state: OutputState::new(&globals, &qh),
                seat_state: SeatState::new(&globals, &qh),
            },
            graphics_context: None,
            keyboard: None,
            lock_data: LockData::from_state(SessionLockState::new(&globals, &qh)),
            gsurfaces: Vec::new(),
            media: Media::from_config(&config),
            _config: config,
            exit: false,
        };

        app_data.lock_data.lock(&qh);
        loop {
            event_queue.blocking_dispatch(&mut app_data).unwrap();

            if app_data.exit {
                break;
            }
        }
    }
}

smithay_client_toolkit::reexports::client::delegate_noop!(AppData: ignore smithay_client_toolkit::reexports::client::protocol::wl_buffer::WlBuffer);
