use std::time::Duration;

use gtk::prelude::*;
use gtk::{glib, Align, Button, Fixed, Label, Overflow};

static BUTTON_ICONS: [&str; 10] = ["󰂃", "󰁻", "󰁼", "󰁽", "󰁾", "󰁿", "󰂀", "󰂁", "󰂂", "󰁹"];
static BATTERY_FOLDER: &str = "/sys/class/power_supply/BAT0";

pub fn button() -> Button {
    let label = Label::builder()
        .label(BUTTON_ICONS[0])
        .css_classes(["battery_label"])
        .build();

    let charging_label = Label::builder()
        .label("󱐋")
        .visible(false)
        .css_classes(["battery_charging_label"])
        .build();

    let fixed = Fixed::builder().overflow(Overflow::Visible).build();

    fixed.put(&label, 0.0, 0.0);
    fixed.put(&charging_label, 0.0, 0.0);

    let button = Button::builder()
        .child(&fixed)
        .valign(Align::Center)
        .halign(Align::Center)
        .hexpand(false)
        .css_classes(["battery"])
        .build();

    let full: f64 = std::fs::read_to_string(format!("{BATTERY_FOLDER}/energy_full"))
        .expect("Battery not found")
        .trim()
        .parse::<usize>()
        .expect("Battery full energy not parsable") as f64;

    glib::timeout_add_local(Duration::from_millis(250), move || {
        let s =
            std::fs::read_to_string(format!("{BATTERY_FOLDER}/status")).expect("Battery not found");

        let status = s.trim();

        let energy: f64 = std::fs::read_to_string(format!("{BATTERY_FOLDER}/energy_now"))
            .expect("Battery not found")
            .trim()
            .parse::<usize>()
            .expect("Battery energy not parsable") as f64;

        let i = ((10.0 * energy / full - 1.0).round().clamp(0.0, 9.0)) as usize;

        label.set_label(BUTTON_ICONS[i]);
        label.set_css_classes(&[&status]);

        charging_label.set_visible(status == "Charging");

        glib::ControlFlow::Continue
    });

    button
}
