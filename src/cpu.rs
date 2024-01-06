use anyhow::{anyhow, Result};
use gtk::prelude::*;
use gtk::{glib, Align, Application, Box, Button, Label, LevelBar, LevelBarMode, Orientation};
use std::time::Duration;
use sysinfo::{CpuRefreshKind, System};

const CPU_ICON: &str = "CPU";
const MINIMUM_CPU_USAGE: f32 = 75.0;

pub fn element(_app: Application) -> Result<Button> {
    if !sysinfo::IS_SUPPORTED_SYSTEM {
        return Err(anyhow!(
            "Widget Disabled: sysinfo does not support this system/os!"
        ));
    };

    let refresh_kind = CpuRefreshKind::new().with_cpu_usage();
    let mut cpu_tracker = System::new();

    let base_box = Box::builder().css_classes(["metric"]).name("cpu").build();

    let label = Label::new(Some(CPU_ICON));

    let bar = LevelBar::builder()
        .name("cpu-bar")
        .css_classes(["metric-bar"])
        .max_value(100.0)
        .min_value(0.0)
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
        cpu_tracker.refresh_cpu_specifics(refresh_kind);

        let cpu_used = cpu_tracker.global_cpu_info().cpu_usage().clamp(0.0, 100.0);

        if cpu_used < MINIMUM_CPU_USAGE {
            b2.set_visible(false);
        } else {
            bar.set_value(cpu_used.into());
            b2.set_visible(true);
        }

        glib::ControlFlow::Continue
    });

    Ok(button)
}
