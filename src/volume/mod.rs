pub mod state;

use state::VolumeState;

use anyhow::{anyhow, Result};
use gtk::prelude::*;
use gtk::{
    glib, Align, Application, Button, EventControllerScroll, EventControllerScrollFlags, Label,
};

const VOLUME_COMMAND: &str = "pw-volume";
const SCROLL_DELTA: f64 = -10.0;

pub fn new(_app: Application) -> Result<Button> {
    log::trace!("Initalizing Volume Widget");

    let label = Label::builder().name("volume").build();

    update_state(&label);

    let button = Button::builder()
        .child(&label)
        .valign(Align::Center)
        .halign(Align::Center)
        .hexpand(false)
        .css_classes(["icon"])
        .build();

    let controller = EventControllerScroll::builder()
        .flags(EventControllerScrollFlags::VERTICAL)
        .build();

    controller.connect_scroll(move |_controller, _dx, dy| {
        use std::cmp::Ordering;

        let delta_y = (SCROLL_DELTA * dy).round() as isize;

        let volume_delta = match delta_y.cmp(&0) {
            Ordering::Greater => format!("+{}%", delta_y),
            Ordering::Less => format!("{}%", delta_y),
            Ordering::Equal => "+0%".to_owned(),
        };

        if let Err(err) = run_command(&["change", volume_delta.as_str()]) {
            log::warn!("{VOLUME_COMMAND} failed to execute. err={err}");
        }

        glib::signal::Propagation::Stop
    });

    label.add_controller(controller);

    {
        let l = label.clone();
        button.connect_clicked(move |_button| {
            if let Err(err) = run_command(&["mute", "toggle"]) {
                log::warn!("{VOLUME_COMMAND} Failed to execute. err={err}");
                return;
            }
            update_state(&l);
        });
    }

    use core::time::Duration;
    glib::timeout_add_local(Duration::from_millis(5000), move || {
        update_state(&label);

        glib::ControlFlow::Continue
    });

    Ok(button)
}

pub fn update_state(label: &Label) {
    let state = VolumeState::get();

    label.set_text(state.to_str());
    label.set_css_classes(match state {
        VolumeState::Volume(_) => &["on"],
        VolumeState::Muted => &["muted"],
        VolumeState::Off => &["off"],
    });
}

use std::process::Command;
use std::rc::Rc;

pub fn run_command(args: &[&str]) -> Result<Rc<str>> {
    let out = Command::new(VOLUME_COMMAND).args(args).output()?;

    if out.status.success() {
        Ok(std::str::from_utf8(&out.stdout)?.into())
    } else if let Some(code) = out.status.code() {
        Err(anyhow!("status_code={code}"))
    } else {
        Err(anyhow!("program crashed"))
    }
}
