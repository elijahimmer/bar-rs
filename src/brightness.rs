use crate::utils::read_f64;

use anyhow::Result;
use const_format::concatcp;
use gtk::prelude::*;
use gtk::{
    glib, Align, Application, Button, EventControllerScroll, EventControllerScrollFlags, Label,
};
use std::fs;
use std::time::Duration;

const BRIGHTNESS_ICONS: [&str; 7] = ["󰃚 ", "󰃛", "󰃜", "󰃝", "󰃞", "󰃟", "󰃠"];
const BRIGHTNESS_CLAMP: f64 = (BRIGHTNESS_ICONS.len() - 1) as f64;
const BACKLIGHT_FOLDER: &str = "/sys/class/backlight/intel_backlight";
const MAX_BRIGHTNESS_FILE: &str = concatcp!(BACKLIGHT_FOLDER, "/max_brightness");
const BRIGHTNESS_FILE: &str = concatcp!(BACKLIGHT_FOLDER, "/brightness");

pub fn new(_app: Application) -> Result<Button> {
    log::trace!("Initalizing Brightness Widget");
    let full = read_f64(MAX_BRIGHTNESS_FILE)?;

    let label = Label::builder().halign(Align::Start).build();

    let controller = EventControllerScroll::builder()
        .flags(EventControllerScrollFlags::VERTICAL)
        .build();

    let button = Button::builder()
        .child(&label)
        .valign(Align::Center)
        .halign(Align::Center)
        .hexpand(false)
        .name("brightness")
        .css_classes(["icon"])
        .build();

    let scroll_delta = full / 100.0;

    controller.connect_scroll(move |_controller, _dx, dy| {
        let current_brightness = match read_f64(BRIGHTNESS_FILE) {
            Ok(float) => float,
            Err(err) => {
                log::warn!("Couldn't read Backlight's Current Brightness. error={err}");

                return glib::signal::Propagation::Stop;
            }
        };

        let brightness = format!(
            "{}",
            (current_brightness + scroll_delta * dy).clamp(0.0, full) as usize
        );

        if let Err(err) = fs::write(BRIGHTNESS_FILE, &brightness) {
            log::warn!("Couldn't set Backlight Brightness. brightness={brightness}, error={err}");
        };

        glib::signal::Propagation::Stop
    });

    button.add_controller(controller);

    glib::timeout_add_local(Duration::from_millis(250), move || {
        let brightness = match read_f64(BRIGHTNESS_FILE) {
            Ok(float) => float,
            Err(err) => {
                log::warn!("Failed to read Brightness File. error={err}");

                return glib::ControlFlow::Break;
            }
        };

        let i = (BRIGHTNESS_CLAMP * brightness / full)
            .round()
            .clamp(0.0, BRIGHTNESS_CLAMP) as usize;

        label.set_label(BRIGHTNESS_ICONS[i]);

        glib::ControlFlow::Continue
    });

    Ok(button)
}
