use std::time::Duration;

use smithay_client_toolkit::{
    compositor::{CompositorHandler, CompositorState},
    output::{OutputHandler, OutputState},
    reexports::{
        calloop::{timer::Timer, EventLoop, LoopHandle},
        calloop_wayland_source::WaylandSource,
        client::{
            globals::registry_queue_init,
            protocol::{wl_output, wl_surface},
            Connection, QueueHandle,
        },
    },
    registry::{ProvidesRegistryState, RegistryState},
    registry_handlers,
    session_lock::{
        SessionLock, SessionLockHandler, SessionLockState, SessionLockSurface,
        SessionLockSurfaceConfigure,
    },
};

use crate::lock_data::LockData;

pub struct AppData {
    loop_handle: LoopHandle<'static, Self>,
    conn: Connection,
    compositor_state: CompositorState,
    output_state: OutputState,
    registry_state: RegistryState,
    lock_data: LockData,
    exit: bool,
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
            lock_data: LockData::from_state(SessionLockState::new(&globals, &qh)),
            exit: false,
        };

        app_data.lock_data.lock(&qh);

        WaylandSource::new(conn.clone(), event_queue)
            .insert(event_loop.handle())
            .unwrap();

        loop {
            event_loop
                .dispatch(Duration::from_millis(500), &mut app_data)
                .unwrap();

            if app_data.exit {
                break;
            }
        }
    }
}

impl SessionLockHandler for AppData {
    fn locked(&mut self, _conn: &Connection, qh: &QueueHandle<Self>, session_lock: SessionLock) {
        println!("Locked");

        for output in self.output_state.outputs() {
            let surface = self.compositor_state.create_surface(&qh);
            let lock_surface = session_lock.create_lock_surface(surface, &output, qh);
            self.loop_handle.insert_idle(|app_data| {
                app_data.lock_data.add_surface(lock_surface);
            });
        }

        self.loop_handle
            .insert_source(
                Timer::from_duration(Duration::from_secs(5)),
                |_, _, app_data| {
                    app_data.lock_data.unlock();
                    app_data.conn.roundtrip().unwrap();
                    // Then we can exit
                    app_data.exit = true;
                    smithay_client_toolkit::reexports::calloop::timer::TimeoutAction::Drop
                },
            )
            .unwrap();
    }

    fn finished(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _session_lock: SessionLock,
    ) {
        println!("Finished");
        self.exit = true;
    }

    fn configure(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _session_lock_surface: SessionLockSurface,
        configure: SessionLockSurfaceConfigure,
        _serial: u32,
    ) {
        let (_width, _height) = configure.new_size;
    }
}

impl CompositorHandler for AppData {
    fn scale_factor_changed(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _surface: &wl_surface::WlSurface,
        _new_factor: i32,
    ) {
    }

    fn transform_changed(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _surface: &wl_surface::WlSurface,
        _new_transform: wl_output::Transform,
    ) {
    }

    fn frame(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _surface: &wl_surface::WlSurface,
        _time: u32,
    ) {
    }

    fn surface_enter(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _surface: &wl_surface::WlSurface,
        _output: &wl_output::WlOutput,
    ) {
        // Not needed for this example.
    }

    fn surface_leave(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _surface: &wl_surface::WlSurface,
        _output: &wl_output::WlOutput,
    ) {
        // Not needed for this example.
    }
}

impl OutputHandler for AppData {
    fn output_state(&mut self) -> &mut OutputState {
        &mut self.output_state
    }

    fn new_output(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _output: wl_output::WlOutput,
    ) {
    }

    fn update_output(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _output: wl_output::WlOutput,
    ) {
    }

    fn output_destroyed(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _output: wl_output::WlOutput,
    ) {
    }
}

impl ProvidesRegistryState for AppData {
    fn registry(&mut self) -> &mut RegistryState {
        &mut self.registry_state
    }
    registry_handlers![OutputState,];
}

smithay_client_toolkit::delegate_compositor!(AppData);
smithay_client_toolkit::delegate_output!(AppData);
smithay_client_toolkit::delegate_session_lock!(AppData);
smithay_client_toolkit::delegate_registry!(AppData);
smithay_client_toolkit::reexports::client::delegate_noop!(AppData: ignore smithay_client_toolkit::reexports::client::protocol::wl_buffer::WlBuffer);
