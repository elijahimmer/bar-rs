use anyhow::Result;
use gtk::{Align, Button, Label};

//const VOLUME_ICONS: [&str; 3] = ["󰕿", "󰖀", "󰕾"];
//const VOLUME_CLAMP: f64 = (VOLUME_ICONS.len() - 1) as f64;
//const VOLUME_MUTED: &str = "󰝟";
const VOLUME_OFF: &str = "󰸈";

pub fn element() -> Result<Button> {
    let label = Label::builder()
        .label(VOLUME_OFF)
        .css_classes(["muted"])
        .name("volume")
        .build();

    let button = Button::builder()
        .child(&label)
        .valign(Align::Center)
        .halign(Align::Center)
        .hexpand(false)
        .css_classes(["icon"])
        .build();

    Ok(button)
}
