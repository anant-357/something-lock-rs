use smithay_client_toolkit::{
    reexports::client::{Connection, QueueHandle},
    seat::{Capability, SeatHandler, SeatState},
};

use crate::app::AppData;

impl SeatHandler for AppData {
    fn new_seat(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _seat: smithay_client_toolkit::reexports::client::protocol::wl_seat::WlSeat,
    ) {
        tracing::debug!("New Seat created!");
    }

    fn seat_state(&mut self) -> &mut SeatState {
        &mut self.wayland.seat_state
    }

    fn new_capability(
        &mut self,
        _conn: &Connection,
        qh: &QueueHandle<Self>,
        seat: smithay_client_toolkit::reexports::client::protocol::wl_seat::WlSeat,
        capability: smithay_client_toolkit::seat::Capability,
    ) {
        if capability == Capability::Keyboard && self.wayland.keyboard.is_none() {
            tracing::trace!("Adding keyboard!");
            self.wayland.keyboard = Some(
                self.wayland.seat_state
                    .get_keyboard(qh, &seat, None)
                    .expect("Failed to create keyboard!"),
            )
        }
    }

    fn remove_capability(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _seat: smithay_client_toolkit::reexports::client::protocol::wl_seat::WlSeat,
        capability: smithay_client_toolkit::seat::Capability,
    ) {
        if capability == Capability::Keyboard && self.wayland.keyboard.is_some() {
            self.wayland.keyboard.take().unwrap().release();
        }
    }

    fn remove_seat(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _seat: smithay_client_toolkit::reexports::client::protocol::wl_seat::WlSeat,
    ) {
    }
}

smithay_client_toolkit::delegate_seat!(AppData);
