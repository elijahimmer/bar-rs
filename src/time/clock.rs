use anyhow::Result;
use gtk::prelude::*;
use gtk::{glib, Box, Label};

pub struct Clock {
    pub container: Box,
    pub hours: Label,
    pub spacer1: Label,
    pub minutes: Label,
    pub spacer2: Label,
    pub seconds: Label,
}

impl Clock {
    pub fn new() -> Clock {
        let hours = Label::new(Some("00"));
        let minutes = Label::new(Some("00"));
        let seconds = Label::new(Some("00"));
        let spacer1 = Label::builder().css_classes(["clock-spacer"]).label("").build();
        let spacer2 = Label::builder().css_classes(["clock-spacer"]).label("").build();

        let container = Box::new(gtk::Orientation::Horizontal, 0);

        container.append(&hours);
        container.append(&spacer1);
        container.append(&minutes);
        container.append(&spacer2);
        container.append(&seconds);

        Clock {
            container,
            hours,
            spacer1,
            minutes,
            spacer2,
            seconds,
        }
    }
    pub fn update(&self) -> Result<()> {
        let now = glib::DateTime::now_local()?;

        self.hours.set_label(format!("{:02}", now.hour()).as_str());
        self.minutes
            .set_label(format!("{:02}", now.minute()).as_str());
        self.seconds
            .set_label(format!("{:02}", now.second()).as_str());

        Ok(())
    }
}

use std::default::Default;
impl Default for Clock {
    fn default() -> Self {
        Self::new()
    }
}
