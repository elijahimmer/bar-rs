use super::run_command::run_command;
use super::VolumeState;
use super::VOLUME_COMMAND;
use gtk::prelude::*;
use gtk::{glib, EventControllerScroll, EventControllerScrollFlags, Label};

#[derive(Clone)]
pub struct VolumeController {
    pub label: Label,
}

impl VolumeController {
    pub fn new() -> Self {
        let state = VolumeState::get();
        let label = Label::builder()
            .label(state.to_str())
            .css_classes(["muted"])
            .name("volume")
            .build();

        let controller = EventControllerScroll::builder()
            .flags(EventControllerScrollFlags::VERTICAL)
            .build();

        controller.connect_scroll(move |_controller, _dx, dy| {
            use std::cmp::Ordering;
            let delta_y = dy.round() as isize;
            let volume_delta = match delta_y.cmp(&0) {
                Ordering::Greater => format!("+{}", delta_y),
                Ordering::Less => format!("{}", delta_y),
                Ordering::Equal => "+0".to_owned(),
            };

            if let Err(err) = run_command(&["volume", volume_delta.as_str()]) {
                log::warn!("{VOLUME_COMMAND} failed to execute. err={err}");
            }

            glib::signal::Propagation::Stop
        });

        Self { label }
    }

    pub fn mute(&self) {
        if let Err(err) = run_command(&["mute", "toggle"]) {
            log::warn!("{VOLUME_COMMAND} Failed to execute. err={err}");
            return;
        }
        self.update_state();
    }

    pub fn update_state(&self) {
        let state = VolumeState::get();

        self.label.set_text(state.to_str());
        self.label.set_css_classes(match state {
            VolumeState::Volume(_) => &["on"],
            VolumeState::Muted => &["muted"],
            VolumeState::Off => &["off"],
        });
    }
}

impl Default for VolumeController {
    fn default() -> Self {
        Self::new()
    }
}
