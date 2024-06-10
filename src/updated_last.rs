use anyhow::{anyhow, Result};
use gtk::{glib, glib::DateTime, Align, Application, Label};

pub fn new(_app: Application) -> Result<Label> {
    log::trace!("Initalizing Updated Last Widget");

    let label = Label::builder()
        .valign(Align::Center)
        .halign(Align::Center)
        .build();

    let mut updated_last = DateTime::from_unix_utc(0)?;

    let mut was_provided = false;
    for arg in std::env::args() {
        if was_provided {
            updated_last = DateTime::from_unix_utc(arg.parse()?)?;
            break;
        }
        was_provided = arg == "--updated-last" || arg == "-U";
    }

    if !was_provided {
        return Err(anyhow!("Last Update time was not provided."));
    }

    let now = match DateTime::now_utc() {
        Ok(now) => now,
        Err(err) => return Err(anyhow!("failed to get time. error={err}")),
    };

    let text = label_from_time(now.difference(&updated_last));

    label.set_label(&text);

    let l2 = label.clone();

    glib::timeout_add_seconds_local(30, move || {
        let now = match DateTime::now_utc() {
            Ok(now) => now,
            Err(err) => {
                log::warn!("failed to get date time. error={err}");
                return glib::ControlFlow::Continue;
            }
        };

        let text = label_from_time(now.difference(&updated_last));

        l2.set_label(&text);

        glib::ControlFlow::Continue
    });

    Ok(label)
}

pub fn label_from_time(delta_time: glib::TimeSpan) -> String {
    if delta_time.as_seconds() < 0 {
        return format!("The Future?");
    }

    let days = delta_time.as_days();
    match days.cmp(&1) {
        core::cmp::Ordering::Equal => return format!("1 Day Ago"),
        core::cmp::Ordering::Greater => return format!("{days} Days Ago"),
        core::cmp::Ordering::Less => {}
    }

    let hours = delta_time.as_hours();
    match hours.cmp(&1) {
        core::cmp::Ordering::Equal => return format!("1 Hour Ago"),
        core::cmp::Ordering::Greater => return format!("{hours} Hours Ago"),
        core::cmp::Ordering::Less => {}
    }

    let minutes = delta_time.as_minutes();
    match minutes.cmp(&1) {
        core::cmp::Ordering::Equal => return format!("1 Minute Ago"),
        core::cmp::Ordering::Greater => return format!("{minutes} Minutes Ago"),
        core::cmp::Ordering::Less => {}
    }

    return format!("Now");
}
