use anyhow::{anyhow, Result};
use gtk::prelude::*;
use gtk::{glib, Align, Application, Box, Button, Label, LevelBar, LevelBarMode, Orientation};
use std::time::Duration;
use sysinfo::{MemoryRefreshKind, System};

const MEMORY_ICON: &str = "RAM";
const MINIMUM_MEMORY_USAGE: f64 = 75.0;

pub fn new(_app: Application) -> Result<Button> {
    log::trace!("Initalizing RAM Widget");
    if !sysinfo::IS_SUPPORTED_SYSTEM {
        return Err(anyhow!("sysinfo does not support this system/os!"));
    };

    let refresh_kind = MemoryRefreshKind::new().with_ram();
    let mut ram_tracker = System::new();

    let base_box = Box::builder().css_classes(["metric"]).name("ram").build();

    let label = Label::new(Some(MEMORY_ICON));

    let bar = LevelBar::builder()
        .css_classes(["metric-bar"])
        .name("ram-bar")
        .max_value(100.0)
        .min_value(0.0)
        .width_request(10)
        .height_request(20)
        .mode(LevelBarMode::Continuous)
        .orientation(Orientation::Vertical)
        .inverted(true)
        .build();

    base_box.append(&label);
    base_box.append(&bar);

    let button = Button::builder()
        .child(&base_box)
        .valign(Align::Center)
        .halign(Align::Center)
        .visible(false)
        .build();

    let b2 = button.clone();

    glib::timeout_add_local(Duration::from_secs(1), move || {
        ram_tracker.refresh_memory_specifics(refresh_kind);

        let ram_used = ram_tracker.used_memory() as f64;
        let ram_total = ram_tracker.total_memory() as f64;

        // just in case something weird happends, we clamp it.
        let ram_fraction = (100.0 * ram_used / ram_total).clamp(0.0, 100.0);

        if ram_fraction < MINIMUM_MEMORY_USAGE {
            b2.set_visible(false);
        } else {
            bar.set_value(ram_fraction);
            b2.set_visible(true);
        }

        glib::ControlFlow::Continue
    });

    Ok(button)
}
