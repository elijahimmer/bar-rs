use crate::util::{read_f64, read_trim};

use const_format::concatcp;
use gtk::prelude::*;
use gtk::{glib, Align, Button, Fixed, Label, Overflow};
use std::time::Duration;

const BATTERY_ICONS: [&str; 10] = ["󰂃", "󰁻", "󰁼", "󰁽", "󰁾", "󰁿", "󰂀", "󰂁", "󰂂", "󰁹"];
const BATTERY_CLAMP: f64 = (BATTERY_ICONS.len() - 1) as f64;
const BATTERY_FOLDER: &str = "/sys/class/power_supply/BAT0";
const ENERGY_FULL_FILE: &str = concatcp!(BATTERY_FOLDER, "/energy_full");
const BATTERY_STATUS_FILE: &str = concatcp!(BATTERY_FOLDER, "/status");
const ENERGY_NOW_FILE: &str = concatcp!(BATTERY_FOLDER, "/energy_now");

pub fn element() -> Option<Button> {
    let label = Label::builder()
        .label(BATTERY_ICONS[0])
        .name("battery")
        .build();

    let charging_label = Label::builder()
        .label("󱐋")
        .visible(false)
        .name("battery_charging")
        .build();

    let fixed = Fixed::builder()
        .overflow(Overflow::Visible)
        .hexpand(false)
        .width_request(25)
        .build();

    fixed.put(&label, 0.0, 0.0);
    fixed.put(&charging_label, 5.0, 0.0);

    let button = Button::builder()
        .child(&fixed)
        .valign(Align::Center)
        .halign(Align::Center)
        .hexpand(false)
        .build();

    let full = match read_f64(ENERGY_FULL_FILE) {
        Ok(f) => f,
        Err(e) => {
            log::warn!("Couldn't read battery's energy_full: {}", e);

            return None;
        }
    };

    glib::timeout_add_local(Duration::from_millis(250), move || {
        let status = {
            let s = match read_trim(BATTERY_STATUS_FILE) {
                Ok(s) => s,
                Err(e) => {
                    log::warn!("Couldn't read battery status: {}", e);

                    return glib::ControlFlow::Break;
                }
            };

            if s == "Not charging" {
                "Full".to_owned()
            } else {
                s
            }
        };

        let energy = match read_f64(ENERGY_NOW_FILE) {
            Ok(s) => s,
            Err(e) => {
                log::warn!("Couldn't read battery's energy_now: {}", e);

                return glib::ControlFlow::Break;
            }
        };

        let i = (BATTERY_CLAMP * energy / full)
            .round()
            .clamp(0.0, BATTERY_CLAMP) as usize;

        label.set_label(BATTERY_ICONS[i]);
        label.set_css_classes(&[&status]);

        charging_label.set_visible(status == "Charging");

        glib::ControlFlow::Continue
    });

    Some(button)
}
