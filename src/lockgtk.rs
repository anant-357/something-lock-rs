use glib::clone;
use gtk4::{prelude::*, Box, Button};
use gtk4::{Application, ApplicationWindow};
pub struct LockGTK {
    pub app: Application,
    _config_path: Option<String>,
}

impl LockGTK {
    pub fn create() -> Self {
        let application = Application::builder()
            .application_id("com.wayland.session_lock")
            .build();
        application.connect_activate(build_ui);
        application.run();

        LockGTK {
            app: application,
            _config_path: None,
        }
    }
}

fn build_ui(application: &Application) {
    let gtk_box = Box::builder()
        .orientation(gtk4::Orientation::Vertical)
        .valign(gtk4::Align::Center)
        .build();
    let button = Button::with_label("Quit");
    button.connect_activate(clone!(@weak application => move |_|{
        tracing::debug!("Clicked Button!");
        application.quit();
    }));
    gtk_box.append(&button);
    let window = ApplicationWindow::builder()
        .title("Lock Screen")
        .default_height(200)
        .default_width(200)
        .child(&gtk_box)
        .application(application)
        .build();

    window.present();
}
