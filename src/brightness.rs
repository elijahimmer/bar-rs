use crate::util::read_f64;

use const_format::concatcp;
use gtk::prelude::*;
use gtk::{glib, Align, Button, EventControllerScroll, EventControllerScrollFlags, Label};
use std::fs;
use std::time::Duration;

const BRIGHTNESS_ICONS: [&str; 7] = ["󰃚", "󰃛", "󰃜", "󰃝", "󰃞", "󰃟", "󰃠"];
const BRIGHTNESS_CLAMP: f64 = (BRIGHTNESS_ICONS.len() - 1) as f64;
const BACKLIGHT_FOLDER: &str = "/sys/class/backlight/intel_backlight";
const MAX_BRIGHTNESS_FILE: &str = concatcp!(BACKLIGHT_FOLDER, "/max_brightness");
const BRIGHTNESS_FILE: &str = concatcp!(BACKLIGHT_FOLDER, "/brightness");

pub fn element() -> Option<Button> {
    let label = Label::builder().name("brightness").build();

    let controller = EventControllerScroll::builder()
        .flags(EventControllerScrollFlags::VERTICAL)
        .build();

    let button = Button::builder()
        .child(&label)
        .valign(Align::Center)
        .halign(Align::Center)
        .hexpand(false)
        .build();

    let full = match read_f64(MAX_BRIGHTNESS_FILE) {
        Ok(f) => f,
        Err(e) => {
            log::warn!(
                "Couldn't read Backlight's Current Brightness: {}",
                e.to_string()
            );

            return None;
        }
    };

    let scroll_delta = full / 25.0;

    controller.connect_scroll(move |_controller, _dx, dy| {
        let current_brightness = match read_f64(BRIGHTNESS_FILE) {
            Ok(f) => f,
            Err(e) => {
                log::warn!("Couldn't read Backlight's Current Brightness: {}", e);

                return glib::signal::Propagation::Stop;
            }
        };

        match fs::write(
            BRIGHTNESS_FILE,
            format!("{}", current_brightness + scroll_delta * dy),
        ) {
            Ok(f) => f,
            Err(e) => {
                log::warn!("Couldn't set Backlight Brightness: {}", e.to_string());

                return glib::signal::Propagation::Stop;
            }
        };

        glib::signal::Propagation::Stop
    });

    button.add_controller(controller);

    glib::timeout_add_local(Duration::from_millis(250), move || {
        let brightness = match read_f64(BRIGHTNESS_FILE) {
            Ok(f) => f,
            Err(e) => {
                log::warn!("Failed to read Brightness File: {}", e);

                return glib::ControlFlow::Break;
            }
        };

        let i = (BRIGHTNESS_CLAMP * brightness / full)
            .round()
            .clamp(0.0, BRIGHTNESS_CLAMP) as usize;

        label.set_label(BRIGHTNESS_ICONS[i]);

        glib::ControlFlow::Continue
    });

    Some(button)
}
