use smithay_client_toolkit::{
    reexports::client::{protocol::wl_surface, Connection, QueueHandle},
    seat::keyboard::KeyboardHandler,
};

use crate::app_data::AppData;

impl KeyboardHandler for AppData {
    fn enter(
        &mut self,
        conn: &Connection,
        qh: &QueueHandle<Self>,
        keyboard: &smithay_client_toolkit::reexports::client::protocol::wl_keyboard::WlKeyboard,
        surface: &wl_surface::WlSurface,
        serial: u32,
        raw: &[u32],
        keysyms: &[smithay_client_toolkit::seat::keyboard::Keysym],
    ) {
    }

    fn leave(
        &mut self,
        conn: &Connection,
        qh: &QueueHandle<Self>,
        keyboard: &smithay_client_toolkit::reexports::client::protocol::wl_keyboard::WlKeyboard,
        surface: &wl_surface::WlSurface,
        serial: u32,
    ) {
    }

    fn press_key(
        &mut self,
        conn: &Connection,
        qh: &QueueHandle<Self>,
        keyboard: &smithay_client_toolkit::reexports::client::protocol::wl_keyboard::WlKeyboard,
        serial: u32,
        event: smithay_client_toolkit::seat::keyboard::KeyEvent,
    ) {
    }

    fn release_key(
        &mut self,
        conn: &Connection,
        qh: &QueueHandle<Self>,
        keyboard: &smithay_client_toolkit::reexports::client::protocol::wl_keyboard::WlKeyboard,
        serial: u32,
        event: smithay_client_toolkit::seat::keyboard::KeyEvent,
    ) {
    }

    fn update_modifiers(
        &mut self,
        conn: &Connection,
        qh: &QueueHandle<Self>,
        keyboard: &smithay_client_toolkit::reexports::client::protocol::wl_keyboard::WlKeyboard,
        serial: u32,
        modifiers: smithay_client_toolkit::seat::keyboard::Modifiers,
        layout: u32,
    ) {
    }

    fn update_repeat_info(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _keyboard: &smithay_client_toolkit::reexports::client::protocol::wl_keyboard::WlKeyboard,
        _info: smithay_client_toolkit::seat::keyboard::RepeatInfo,
    ) {
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
