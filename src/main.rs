use std::path::PathBuf;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{mpsc, RwLock};
use std::{fs, thread};

use gtk::glib::{self, Propagation};
use gtk::prelude::*;
use gtk::{
    Application, ApplicationWindow, CssProvider, Justification, Label,
    Orientation, Scale,
};

mod audio;
mod config;
mod consts;

use audio::Metronome;
use config::*;
use consts::*;

static METRONOME: RwLock<Metronome> = RwLock::new(Metronome::new());
static CONFIG: RwLock<Option<Config>> = RwLock::new(None);

fn main() -> glib::ExitCode {
    // make the user data folder
    let data_dir = user_data_dir();
    if !data_dir.exists() && fs::create_dir_all(&data_dir).is_err() {
        eprintln!("warning: could not create data directory.");
    }
    let config = Config::load_or_create();
    CONFIG.write().unwrap().replace(config);

    let (tx, rx): (Sender<bool>, Receiver<bool>) = mpsc::channel();

    // start in the "off" state
    tx.send(false).unwrap();
    thread::spawn(|| Metronome::start(&METRONOME, rx));

    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(move |app| build_ui(app, tx.clone()));
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

fn build_ui(app: &Application, tx: Sender<bool>) {
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

    let slider_box = build_slider_box(tx);

    main_box.append(&slider_box);

    // Present window
    window.set_child(Some(&main_box));

    window.connect_close_request(|_| {
        CONFIG.read().unwrap().as_ref().unwrap().save();
        Propagation::Proceed
    });

    window.present();
}

fn build_slider_box(stop_button_tx: Sender<bool>) -> gtk::Box {
    let last_bpm = CONFIG.read().unwrap().as_ref().unwrap().bpm;

    let res = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .build();

    let bpm_label = Label::builder()
        .label(&fmt_bpm(last_bpm))
        .name("bpm-label")
        .hexpand(true)
        .justify(Justification::Center)
        .build();

    let scale = Scale::builder()
        .hexpand(true)
        .round_digits(0)
        .show_fill_level(true)
        .build();
    scale.set_range(MIN_BPM, MAX_BPM);
    scale.set_value(last_bpm);

    let start_btn = gtk::Button::builder().label("Start").hexpand(true).build();

    res.append(&bpm_label);
    res.append(&scale);
    res.append(&start_btn);

    scale.connect_value_changed(move |scale| {
        let new_bpm = scale.value();
        CONFIG.write().unwrap().as_mut().unwrap().bpm = new_bpm;
        METRONOME.write().unwrap().bpm = new_bpm;
        bpm_label.set_text(&fmt_bpm(new_bpm));
    });

    res
}

fn fmt_bpm(bpm: f64) -> String {
    format!("{} BPM", bpm)
}
