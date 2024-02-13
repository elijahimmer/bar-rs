use crate::utils::read_f64;

use anyhow::Result;
use const_format::concatcp;
use gtk::prelude::*;
use gtk::{
    glib, Align, Application, Button, EventControllerScroll, EventControllerScrollFlags, Fixed,
    Label, LevelBar, LevelBarMode, Orientation,
};
use std::fs;
use std::time::Duration;

const BRIGHTNESS_ICON: &str = "󰃞";
const BACKLIGHT_FOLDER: &str = "/sys/class/backlight/intel_backlight";
const MAX_BRIGHTNESS_FILE: &str = concatcp!(BACKLIGHT_FOLDER, "/max_brightness");
const BRIGHTNESS_FILE: &str = concatcp!(BACKLIGHT_FOLDER, "/brightness");

use std::default::Default;
#[derive(Clone, Default)]
pub struct Icons {
    outer: Button,
    //container: Fixed,
    //bright_outline: Label,
    bright_bar: LevelBar,
}

impl Icons {
    pub fn new() -> Icons {
        let bright_outline = Label::new(Some(BRIGHTNESS_ICON));

        let bright_bar = LevelBar::builder()
            .max_value(1.0)
            .min_value(0.0)
            .mode(LevelBarMode::Continuous)
            .orientation(Orientation::Vertical)
            .inverted(true)
            .height_request(11)
            .width_request(11)
            .build();

        let container = Fixed::builder().width_request(19).hexpand(false).build();

        container.put(&bright_bar, 4.5, 7.5);
        container.put(&bright_outline, 0.0, 0.0);

        let outer = Button::builder()
            .child(&container)
            .valign(Align::Center)
            .halign(Align::Center)
            .hexpand(false)
            .name("brightness")
            .css_classes(["icon"])
            .build();

        Icons {
            outer,
            //container,
            //bright_outline,
            bright_bar,
        }
    }
}

pub fn new(_app: Application) -> Result<Button> {
    log::trace!("Initalizing Brightness Widget");

    let full = read_f64(MAX_BRIGHTNESS_FILE)?;
    let icons = Icons::new();

    let controller = EventControllerScroll::builder()
        .flags(EventControllerScrollFlags::VERTICAL)
        .build();

    let scroll_delta = full / 100.0;

    {
        let i = icons.clone();
        controller.connect_scroll(move |_controller, _dx, dy| {
            let current_brightness = match read_f64(BRIGHTNESS_FILE) {
                Ok(float) => float,
                Err(err) => {
                    log::warn!("Couldn't read Backlight's Current Brightness. error={err}");

                    return glib::signal::Propagation::Stop;
                }
            };

        let brightness =
            ((current_brightness + scroll_delta * dy).clamp(0.0, full) as usize).to_string();

            if let Err(err) = fs::write(BRIGHTNESS_FILE, &brightness) {
                log::warn!(
                    "Couldn't set Backlight Brightness. brightness={brightness}, error={err}"
                );
            };

            i.bright_bar.set_value(current_brightness / full);

            glib::signal::Propagation::Stop
        });
    }

    icons.outer.add_controller(controller);

    {
        let i = icons.clone();
        glib::timeout_add_local(Duration::from_secs(1), move || {
            let current_brightness = match read_f64(BRIGHTNESS_FILE) {
                Ok(float) => float,
                Err(err) => {
                    log::warn!("Failed to read Brightness File. error={err}");

                    return glib::ControlFlow::Break;
                }
            };

            // I shouldn't need to check if the brightness from the driver is higher than the full
            // brightness... right?
            let brightness = current_brightness.clamp(0.0, full) / full;

            i.bright_bar.set_value(brightness);

            glib::ControlFlow::Continue
        });
    }

    Ok(icons.outer)
}
