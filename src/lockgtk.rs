use gtk4::{Application, Window};

pub struct LockGTK {
    app: Application,
    focused_window: Window,
    config_path: String,
}

impl LockGTK {
    pub fn create() -> Self {}

    pub fn destroy(&self) {}
}
