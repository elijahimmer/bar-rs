pub mod controller;
pub mod run_command;
pub mod state;
use controller::VolumeController;
use state::VolumeState;

use anyhow::Result;
use gtk::prelude::*;
use gtk::{glib, Align, Application, Button};

const VOLUME_COMMAND: &str = "pw-volume";

pub fn new(_app: Application) -> Result<Button> {
    log::trace!("Initalizing Volume Widget");

    let volume = VolumeController::new();

    let button = Button::builder()
        .child(&volume.label)
        .valign(Align::Center)
        .halign(Align::Center)
        .hexpand(false)
        .css_classes(["icon"])
        .build();

    {
        let v = volume.clone();
        button.connect_clicked(move |_button| v.mute());
    }

    use core::time::Duration;
    glib::timeout_add_local(Duration::from_millis(5000), move || {
        volume.update_state();

        glib::ControlFlow::Continue
    });

    Ok(button)
}
