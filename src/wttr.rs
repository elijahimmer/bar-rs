/*
 * I started to make a weather module, then I thought about it,
 *  I don't think I actually need this.
 *  I will keep it here incase I ever change my mind, but for now,
 *  It will not be apart of the project.
 */

use gtk::{glib, Align, Button, Label};
use std::process::Command;
use std::time::Duration;

const WTTR_URL: &str = "https://wttr.in/?m0QA";

pub fn element() -> Button {
    let wttr_label = Label::new(None);

    let wttr_button = Button::builder()
        .child(&wttr_label)
        .valign(Align::Center)
        .halign(Align::Center)
        .hexpand(false)
        .name("clock")
        .build();

    glib::timeout_add_local(Duration::from_secs(60), move || {
        let res = Command::new("curl").arg(WTTR_URL).output();

        match res {
            Ok(out) => {
                if out.status.success() {
                    log::trace!("curl executed successfully");
                    log::trace!("")
                } else {
                    log::warn!("Curl Failed: exit code {}", out.status.code().unwrap_or(-1));
                }
            }
            Err(e) => {
                log::warn!("Failed to execute curl: {e}");
            }
        }

        glib::ControlFlow::Continue
    });

    wttr_button
}
