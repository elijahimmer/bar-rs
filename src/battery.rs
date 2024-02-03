use crate::utils::{read_f64, read_trim};

use anyhow::Result;
use const_format::concatcp;
use gtk::prelude::*;
use gtk::{glib, Align, Application, Button, Fixed, Label};
use std::time::Duration;

const BATTERY_ICONS: [&str; 10] = ["󰂃", "󰁻", "󰁼", "󰁽", "󰁾", "󰁿", "󰂀", "󰂁", "󰂂", "󰁹"];
const BATTERY_CLAMP: f64 = (BATTERY_ICONS.len() - 1) as f64;
// TODO: I should make this not hard coded and read all of them.
const BATTERY_FOLDER: &str = "/sys/class/power_supply/BAT0";
const ENERGY_FULL_FILE: &str = concatcp!(BATTERY_FOLDER, "/energy_full");
const BATTERY_STATUS_FILE: &str = concatcp!(BATTERY_FOLDER, "/status");
const ENERGY_NOW_FILE: &str = concatcp!(BATTERY_FOLDER, "/energy_now");
const MAX_TRIES: usize = 10;

pub struct Icons {
    container: Fixed,
    charging: Label,
    battery: Label,
}

impl Icons {
    pub fn new() -> Icons {
        let charging = Label::builder()
            .name("battery_charging")
            .label("󱐋")
            .visible(false)
            .build();
        let battery = Label::new(Some(BATTERY_ICONS[0]));
        let container = Fixed::builder().name("battery").hexpand(false).build();

        container.put(&battery, 0.0, 0.0);
        container.put(&charging, 5.0, 0.0);

        Icons {
            container,
            charging,
            battery,
        }
    }
}

use std::default::Default;
impl Default for Icons {
    fn default() -> Self {
        Self::new()
    }
}

pub fn new(_app: Application) -> Result<Button> {
    log::trace!("Initalizing Battery Widgets");

    let full = read_f64(ENERGY_FULL_FILE)?;
    let icons = Icons::new();

    let button = Button::builder()
        .child(&icons.container)
        .valign(Align::Center)
        .halign(Align::Center)
        .css_classes(["icon"])
        .build();

    let mut tries = 0;

    glib::timeout_add_local(Duration::from_millis(250), move || {
        let (energy, status) = match get_battery_info() {
            Ok(res) => res,
            Err(err) => {
                log::warn!("{err}");

                tries += 1;

                return if tries > MAX_TRIES {
                    log::warn!(
                        "Widget Disabled: Failed to query battery too many successive times."
                    );
                    glib::ControlFlow::Break
                } else {
                    glib::ControlFlow::Continue
                };
            }
        };

        // reset tries count if it gets passed it all
        tries = 0;

        let charge_percent = energy / full;

        let i = (BATTERY_CLAMP * charge_percent)
            .round()
            .clamp(0.0, BATTERY_CLAMP) as usize;

        let status_list = if status == "Discharging" && charge_percent < 0.25 {
            if charge_percent < 0.1 {
                ["Critical"]
            } else {
                ["Warn"]
            }
        } else {
            [status.as_str()]
        };

        icons.battery.set_label(BATTERY_ICONS[i]);
        icons.battery.set_css_classes(&status_list);

        icons.charging.set_visible(status == "Charging");

        glib::ControlFlow::Continue
    });

    Ok(button)
}

fn get_battery_info() -> Result<(f64, String)> {
    let status = {
        let s = read_trim(BATTERY_STATUS_FILE)?;

        if s == "Not charging" {
            "Full".to_owned()
        } else {
            s
        }
    };

    let energy = read_f64(ENERGY_NOW_FILE)?;

    Ok((energy, status))
}
