use std::io::{BufReader, Cursor};
use std::path::PathBuf;
use std::sync::RwLock;
use std::{fs, thread};

use audio::play_metronome;
use gtk::prelude::*;
use gtk::{
    glib, Application, ApplicationWindow, CssProvider, Justification, Label,
    Orientation, Scale,
};

mod audio;
mod config;
mod consts;

use config::*;
use consts::*;

static CONFIG: RwLock<Option<Config>> = RwLock::new(None);

fn main() -> glib::ExitCode {
    // make the user data folder
    let data_dir = user_data_dir();
    if !data_dir.exists() && fs::create_dir_all(&data_dir).is_err() {
        eprintln!("warning: could not create data directory.");
    }
    let config = Config::load_or_create();
    CONFIG.write().unwrap().replace(config);

    // start the app
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.connect_startup(|_| load_css());
    app.run()
}

fn load_css() {
    let css = CssProvider::new();
    css.load_from_string(include_str!("style.css"));

    gtk::style_context_add_provider_for_display(
        &gtk::gdk::Display::default().expect("Could not connect to a display."),
        &css,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

fn build_ui(app: &Application) {
    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Metronome")
        .build();

    let main_box = gtk::Box::builder()
        .spacing(10)
        .margin_top(10)
        .margin_bottom(10)
        .margin_start(10)
        .margin_end(10)
        .width_request(600)
        .orientation(Orientation::Vertical)
        .build();

    let slider_box = build_slider_box();

    main_box.append(&slider_box);

    // Present window
    window.set_child(Some(&main_box));
    window.present();

    // let audio = include_bytes!("sounds/fl-metronome-hat.wav");
    // let audio = BufReader::new(Cursor::new(audio));
    // let player = thread::spawn(|| play_metronome(audio));
    // player.join().unwrap();
}

fn build_slider_box() -> gtk::Box {
    let res = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .build();

    let bpm_label = Label::builder()
        .label("XXX BPM")
        .name("bpm-label")
        .hexpand(true)
        .justify(Justification::Center)
        .build();
    let slider = Scale::builder().hexpand(true).build();

    res.append(&bpm_label);
    res.append(&slider);
    res
}
