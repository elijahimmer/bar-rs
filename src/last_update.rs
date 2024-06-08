use anyhow::Result;
use gtk::{Align, Application, Button, Label};


pub fn new(_app: Application) -> Result<Button> {
    log::trace!("Initalizing Last Update Widget");

    use gtk::glib;
    use std::fs;
    use std::time::SystemTime;

    let contents = match fs::read_to_string(UPDATE_PATH) {
        Ok(date) => date,
        Err(err) => {
            log::warn!("Couldn't read file. error={err}");
            "".to_string()
        }
    };

    let label = Label::new(Some(contents.trim()));

    let button = Button::builder()
        .child(&label)
        .valign(Align::Center)
        .halign(Align::Center)
        .build();

    const UPDATE_PATH: &str = "/etc/bar-rs/updated_last";

    let mut modified_last = SystemTime::now();

    glib::timeout_add_seconds_local(15, move || {
        match fs::metadata(UPDATE_PATH) {
            Ok(modified) => match modified.modified() {
                Ok(modified) => {
                    if modified > modified_last {
                        modified_last = modified;
                        log::info!("Updating Last Update Time");

                        match fs::read_to_string(UPDATE_PATH) {
                            Ok(date) => label.set_text(date.trim()),
                            Err(err) => log::warn!("Couldn't read file. error={err}"),
                        }
                    }
                }
                Err(err) => log::warn!("Couldn't get Update File Access Time file={UPDATE_PATH}, error={err}"),
            }
            Err(err) => log::warn!("Couldn't get Update File Metadata file={UPDATE_PATH}, error={err}"),
        };

        glib::ControlFlow::Continue
    });


    Ok(button)
}
