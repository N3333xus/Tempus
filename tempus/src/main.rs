use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow};

const APP_ID: &str = "org.gtk_rs.HelloWorld2";

fn main() -> glib::ExitCode {
    
    let app = Application::builder().application_id(APP_ID).build(); // creates a new app

    app.connect_activate(build_ui); // Connect to "activate" signal of `app`

    app.run()
}

fn build_ui(app: &Application) {
    // Create a window and set the title
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .build();

    window.present();
}