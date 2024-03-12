use wayland_client::{
    globals::{registry_queue_init, GlobalList},
    Connection,
};
use wayland_protocols::{
    ext::session_lock::v1::client::{
        ext_session_lock_manager_v1::ExtSessionLockManagerV1,
        ext_session_lock_surface_v1::ExtSessionLockSurfaceV1,
    },
    xdg::xdg_output::zv1::client::{
        zxdg_output_manager_v1::ZxdgOutputManagerV1, zxdg_output_v1::ZxdgOutputV1,
    },
};

use crate::{
    lock_state::LockState,
    output::{OutputInfo, OutputState},
};

pub struct LockConnection {
    pub conn: Connection,
    pub globals: GlobalList,
    outputs: Vec<OutputInfo>,
}

impl LockConnection {
    pub fn connect() -> Self {
        let conn = Connection::connect_to_env().unwrap();
        tracing::debug!("Connected to Environment!");
        Self::from_connection(conn)
    }

    fn from_connection(conn: Connection) -> Self {
        let (globals, _) = registry_queue_init::<LockState>(&conn).unwrap();
        let mut state = Self {
            conn,
            globals,
            outputs: Vec::new(),
        };
        state.refresh_outputs();
        state
    }

    pub fn _get_outputs(&self) -> &Vec<OutputInfo> {
        &self.outputs
    }

    pub fn refresh_outputs(&mut self) {
        let mut state = OutputState {
            outputs: Vec::new(),
        };
        let mut event_queue = self.conn.new_event_queue::<OutputState>();
        let qh = event_queue.handle();
        let output_manager = match self
            .globals
            .bind::<ZxdgOutputManagerV1, _, _>(&qh, 3..=3, ())
        {
            Ok(x) => {
                tracing::debug!("created output manager");
                x
            }
            Err(e) => panic!("{:#?}", e),
        };
        let display = self.conn.display();
        let _ = display.get_registry(&qh, ());
        event_queue.roundtrip(&mut state).unwrap();
        event_queue.roundtrip(&mut state).unwrap();

        let xdg_outputs: Vec<ZxdgOutputV1> = state
            .outputs
            .iter()
            .map(|output| output_manager.get_xdg_output(&output.output, &qh, ()))
            .collect();

        event_queue.roundtrip(&mut state).unwrap();

        for xdg_output in xdg_outputs {
            xdg_output.destroy();
        }

        if state.outputs.is_empty() {
            tracing::error!("Compositor did not advertise any wl_output devices!");
        }

        tracing::debug!("outputs detected: {:#?}", state.outputs);
        self.outputs = state.outputs;
    }

    pub fn lock(&self) {
        let mut state = LockState {
            locked: false,
            base_surface: None,
            xdg_surface: None,
            wm_base: None,
            session_lock_surfaces: Vec::new(),
        };
        let mut event_queue = self.conn.new_event_queue::<LockState>();
        let qh = event_queue.handle();
        let session_lock_manager =
            match self
                .globals
                .bind::<ExtSessionLockManagerV1, _, _>(&qh, 1..=1, ())
            {
                Ok(x) => {
                    tracing::debug!("created session lock manager");
                    x
                }
                Err(e) => panic!("{:#?}", e),
            };
        let session_lock = session_lock_manager.lock(&qh, ());
        let display = self.conn.display();
        let _ = display.get_registry(&qh, ());
        event_queue.roundtrip(&mut state).unwrap();
        let base_surface = state.base_surface.unwrap();
        let _lock_surfaces: Vec<ExtSessionLockSurfaceV1> = self
            .outputs
            .iter()
            .map(|output| session_lock.get_lock_surface(&base_surface, &output.output, &qh, ()))
            .collect();
    }

    pub fn _unlock(&self) {}
}
