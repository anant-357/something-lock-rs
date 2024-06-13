use std::time::Duration;

use smithay_client_toolkit::{
    compositor::CompositorState,
    output::OutputState,
    reexports::{
        calloop::{EventLoop, LoopHandle},
        calloop_wayland_source::WaylandSource,
        client::{
            globals::registry_queue_init, protocol::wl_keyboard::WlKeyboard, Connection,
            QueueHandle,
        },
    },
    registry::RegistryState,
    seat::SeatState,
    session_lock::SessionLockState,
};

use crate::lock_data::LockData;

pub struct AppData {
    pub loop_handle: LoopHandle<'static, Self>,
    pub conn: Connection,
    pub compositor_state: CompositorState,
    pub output_state: OutputState,
    pub registry_state: RegistryState,
    pub seat_state: SeatState,
    pub keyboard: Option<WlKeyboard>,
    pub lock_data: LockData,
    pub exit: bool,
}

impl AppData {
    pub fn connect() {
        let conn = Connection::connect_to_env().unwrap();

        let (globals, event_queue) = registry_queue_init(&conn).unwrap();
        let qh: QueueHandle<AppData> = event_queue.handle();
        let mut event_loop: EventLoop<AppData> =
            EventLoop::try_new().expect("Failed to initialize the event loop!");

        let mut app_data = AppData {
            loop_handle: event_loop.handle(),
            conn: conn.clone(),
            compositor_state: CompositorState::bind(&globals, &qh).unwrap(),
            output_state: OutputState::new(&globals, &qh),
            registry_state: RegistryState::new(&globals),
            seat_state: SeatState::new(&globals, &qh),
            keyboard: None,
            lock_data: LockData::from_state(SessionLockState::new(&globals, &qh)),
            exit: false,
        };

        app_data.lock_data.lock(&qh);

        WaylandSource::new(conn.clone(), event_queue)
            .insert(event_loop.handle())
            .unwrap();

        loop {
            event_loop
                .dispatch(Duration::from_millis(5), &mut app_data)
                .unwrap();

            if app_data.exit {
                break;
            }
        }
    }
}

smithay_client_toolkit::reexports::client::delegate_noop!(AppData: ignore smithay_client_toolkit::reexports::client::protocol::wl_buffer::WlBuffer);
