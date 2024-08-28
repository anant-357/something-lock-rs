use smithay_client_toolkit::{
    reexports::client::{protocol::wl_surface, Connection, QueueHandle},
    seat::keyboard::{KeyboardHandler, Keysym},
};

use crate::app::AppData;

impl KeyboardHandler for AppData {
    fn enter(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _keyboard: &smithay_client_toolkit::reexports::client::protocol::wl_keyboard::WlKeyboard,
        _surface: &wl_surface::WlSurface,
        _serial: u32,
        _raw: &[u32],
        _keysyms: &[smithay_client_toolkit::seat::keyboard::Keysym],
    ) {
    }

    fn leave(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _keyboard: &smithay_client_toolkit::reexports::client::protocol::wl_keyboard::WlKeyboard,
        _surface: &wl_surface::WlSurface,
        _serial: u32,
    ) {
    }

    fn press_key(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _keyboard: &smithay_client_toolkit::reexports::client::protocol::wl_keyboard::WlKeyboard,
        _serial: u32,
        event: smithay_client_toolkit::seat::keyboard::KeyEvent,
    ) {
        tracing::trace!(
            "Entering Key press: {event:?}, password_buffer: {}",
            self.lock_data.password_buffer
        );
        match event.keysym {
            Keysym::Return => {
                match self.lock_data.unlock_with_auth() {
                    Ok(_) => {
                        tracing::trace!("Authenticated, unlocked!");
                    }
                    Err(e) => tracing::warn!("{e}"),
                }
                self.exit = true;
                self.conn.roundtrip().unwrap();
            }
            Keysym::BackSpace => {
                tracing::trace!("Backspacing!");
                self.lock_data.password_buffer.pop();
                return;
            }
            _ => {
                let key_char = event.keysym.key_char();
                if key_char.is_some() {
                    self.lock_data.password_buffer.push(key_char.unwrap());
                }
            }
        }
        tracing::trace!(
            "Leaving Key press: {event:?}, password_buffer: {}",
            self.lock_data.password_buffer
        );
    }

    fn release_key(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _keyboard: &smithay_client_toolkit::reexports::client::protocol::wl_keyboard::WlKeyboard,
        _serial: u32,
        _event: smithay_client_toolkit::seat::keyboard::KeyEvent,
    ) {
    }

    fn update_modifiers(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _keyboard: &smithay_client_toolkit::reexports::client::protocol::wl_keyboard::WlKeyboard,
        _serial: u32,
        modifiers: smithay_client_toolkit::seat::keyboard::Modifiers,
        _layout: u32,
    ) {
        tracing::trace!("Keyboard Handler: Update Modifiers {modifiers:?}");
    }

    fn update_repeat_info(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _keyboard: &smithay_client_toolkit::reexports::client::protocol::wl_keyboard::WlKeyboard,
        info: smithay_client_toolkit::seat::keyboard::RepeatInfo,
    ) {
        tracing::trace!("Keyboard Handler: Update Repeat Information {info:?}");
    }

    fn update_keymap(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _keyboard: &smithay_client_toolkit::reexports::client::protocol::wl_keyboard::WlKeyboard,
        _keymap: smithay_client_toolkit::seat::keyboard::Keymap<'_>,
    ) {
    }
}

smithay_client_toolkit::delegate_keyboard!(AppData);
