use anyhow::{anyhow, Result};
use gtk::prelude::*;
use gtk::{glib, Align, Application, Button, Label};
use std::process::Command;
use std::rc::Rc;

const VOLUME_ICONS: [&str; 3] = ["󰕿", "󰖀", "󰕾"];
const VOLUME_CLAMP: f64 = (VOLUME_ICONS.len() - 1) as f64;
const VOLUME_MUTED: &str = "󰝟";
const VOLUME_OFF: &str = "󰸈";
const VOLUME_COMMAND: &str = "pw-volume";

pub fn new(_app: Application) -> Result<Button> {
    log::trace!("Initalizing Volume Widget");

    let volume = VolumeController::new();

    let button = Button::builder()
        .child(&volume.label)
        .valign(Align::Center)
        .halign(Align::Center)
        .hexpand(false)
        .css_classes(["icon"])
        .build();

    {
        let v = volume.clone();
        button.connect_clicked(move |_button| v.mute());
    }

    use core::time::Duration;
    glib::timeout_add_local(Duration::from_millis(250), move || {
        volume.update_state();

        glib::ControlFlow::Continue
    });

    Ok(button)
}

pub fn run_command(args: &[&str]) -> Result<Rc<str>> {
    let out = Command::new(VOLUME_COMMAND).args(args).output()?;

    if out.status.success() {
        Ok(std::str::from_utf8(&out.stdout)?.into())
    } else {
        Err(anyhow!(
            "status_code={}",
            out.status.code().unwrap_or_default()
        ))
    }
}

#[derive(Clone)]
enum VolumeState {
    Volume(isize),
    Muted,
    Off,
}

impl VolumeState {
    fn to_str(&self) -> &'static str {
        match self {
            Self::Volume(percent) => {
                VOLUME_ICONS[((*percent as f64) / (VOLUME_ICONS.len() as f64))
                    .clamp(0.0, VOLUME_CLAMP) as usize]
            }
            Self::Muted => VOLUME_MUTED,
            Self::Off => VOLUME_OFF,
        }
    }

    fn get() -> Self {
        match run_command(&["status"]) {
            Ok(str) => {
                // This is a simple- but breakable- way to avoid adding serde to
                // the program.
                const MATCH_STR: &str = "\"percentage\":";

                match str.find(MATCH_STR) {
                    Some(index) => {
                        let idx = index + MATCH_STR.len();
                        match str[idx..].find(',') {
                            Some(jdx) => match str[idx..idx+jdx].parse() {
                                Ok(i) => Self::Volume(i),
                                Err(err) => {
                                    log::warn!("Failed to parse percent from {VOLUME_COMMAND}. error={err}");
                                    Self::Volume(100)
                                }
                            },
                            None => {
                                log::trace!("Failed to find percent's value from {VOLUME_COMMAND}. idx={idx}, str={str}");
                                Self::Muted
                            }
                        }
                    }
                    None => Self::Muted,
                }
            }
            Err(err) => {
                log::warn!("{VOLUME_COMMAND} Failed to execute. err={err}");
                Self::Off
            }
        }
    }
}

#[derive(Clone)]
pub struct VolumeController {
    label: Label,
    controlled: Controller,
}

impl VolumeController {
    fn new() -> Self {
        let state = VolumeState::get();
        let label = Label::builder()
            .label(state.to_str())
            .css_classes(["muted"])
            .name("volume")
            .build();

        let controller = EventControllerScroll::builder()
                .flags(EventControllerScrollFlags::VERTICAL)
                .build();

        let scroll_delta = full / 100.0;

        controller.connect_scroll(move |_controller, _dx, dy| {
        let current_brightness = match read_f64(BRIGHTNESS_FILE) {
            Ok(float) => float,
            Err(err) => {
                log::warn!("Couldn't read Backlight's Current Brightness. error={err}");

                return glib::signal::Propagation::Stop;
            }
        };

        let brightness = format!(
            "{}",
            (current_brightness + scroll_delta * dy).clamp(0.0, full) as usize
        );

        if let Err(err) = fs::write(BRIGHTNESS_FILE, &brightness) {
            log::warn!("Couldn't set Backlight Brightness. brightness={brightness}, error={err}");
        };

        glib::signal::Propagation::Stop
    });



        Self { label }
    }

    fn mute(&self) {
        if let Err(err) = run_command(&["mute", "toggle"]) {
            log::warn!("{VOLUME_COMMAND} Failed to execute. err={err}");
            return;
        }
        self.update_state();
    }

    fn update_state(&self) {
        let state = VolumeState::get();

        self.label.set_text(state.to_str());
        self.label.set_css_classes(&[]);
    }
}


