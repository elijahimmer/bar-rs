pub mod utils;
pub use utils::*;

use anyhow::{anyhow, Result};
use gtk::prelude::*;
use gtk::{glib, Box, Button, Label};
use std::env;
use std::io::{ErrorKind::WouldBlock, Read};
use std::os::unix::net::UnixStream;
use std::time::Duration;

pub type Workspace = (i32, Button);

const HYPR_STREAM_BUFFER_SIZE: usize = 1024;
const ACTIVE_WORKSPACE_CLASSES: [&str; 1] = ["active-workspace"];

// I don't know why someone would compile this on non-unix, but
//      this could help them I guess...
#[cfg(not(unix))]
pub fn element() -> Reselt<Box> {
    compile_error!("Hyprland Widget only works on windows! Disable the Hyprland Feature!");
}

#[cfg(unix)]
pub fn element() -> Result<Box> {
    let his = env::var_os("HYPRLAND_INSTANCE_SIGNATURE")
        .ok_or(anyhow!(
            "Failed to get HYPRLAND_INSTANCE_SIGNATURE environment variable"
        ))?
        .into_string()
        .unwrap();

    let stream_path = format!("/tmp/hypr/{his}/.socket2.sock");
    let mut hypr_stream = UnixStream::connect(stream_path)?;

    hypr_stream.set_nonblocking(true)?;

    let main = Box::builder().build();

    let work_box = Box::builder().homogeneous(true).name("workspaces").build();
    let submap_label = Label::builder().name("submap").build();

    main.append(&work_box);
    main.append(&submap_label);

    let mut workspaces = match utils::jumpstart_workspaces() {
        Ok(v) => v,
        Err(e) => {
            log::warn!("Failed to populate workspaces from hyprland: {e}");

            vec![utils::create_workspace(1)]
        }
    };

    let mut active_workspace: Workspace = {
        let i = match utils::jumpstart_active_workspace() {
            Ok(a) => a,
            Err(e) => {
                log::warn!("Failed to get active workspace from hyprctl: {e}");

                1
            }
        };

        let j = workspaces.binary_search_by_key(&i, |w| w.0).unwrap_or(0);

        workspaces[j].clone()
    };

    for (i, w) in workspaces.iter() {
        if *i == active_workspace.0 {
            w.set_css_classes(&ACTIVE_WORKSPACE_CLASSES);
        }
        work_box.append(w);
    }

    let mut buffer = [0; HYPR_STREAM_BUFFER_SIZE];
    let mut last_index = 0;

    let mut error_counter = 0;

    let m2 = main.clone();

    glib::timeout_add_local(Duration::from_millis(100), move || {
        let size = match hypr_stream.read(&mut buffer[last_index..]) {
            Ok(s) => s,
            Err(e) => {
                if e.kind() != WouldBlock {
                    log::warn!("Failed to read from Hyprstream: {e}");
                    error_counter += 1;

                    if error_counter > 10 {
                        log::error!(
                            "Failed to read from Hyprstream too many times. Killing widget"
                        );

                        m2.set_visible(false);

                        return glib::ControlFlow::Break;
                    }
                }
                return glib::ControlFlow::Continue;
            }
        };

        let message = std::str::from_utf8(&buffer[..size]).unwrap();

        let mut copy_over = false;

        while let Some(i) = message[last_index..].find('\n') {
            let m = &message[last_index..last_index + i];

            if m.contains('\n') {
                log::warn!("Failed to capture newline correctly. \"{m}\"");
                last_index = 0;
                break;
            }

            let message: &str;
            let args: &str;

            let j: usize = m.find(">>").unwrap();

            message = &m[..j];
            args = &m[(j + 2)..];

            match parse_message(message, args).unwrap() {
                Event::Workspace(i) => {
                    //log::debug!("Switching Workspace: i={i}");
                    active_workspace.1.set_css_classes(&[]);
                    match workspaces.binary_search_by_key(&i, |w| w.0) {
                        Ok(j) => {
                            let wk = &workspaces[j];

                            wk.1.set_css_classes(&ACTIVE_WORKSPACE_CLASSES);

                            active_workspace = wk.clone();
                        }
                        Err(j) => {
                            // if not found in array, add it.
                            active_workspace = create_workspace(i);
                            active_workspace
                                .1
                                .set_css_classes(&ACTIVE_WORKSPACE_CLASSES);
                            workspaces.insert(j, active_workspace.clone());
                        }
                    };
                }
                Event::CreateWorkspace(i) => match workspaces.binary_search_by_key(&i, |w| w.0) {
                    Ok(_j) => log::warn!(
                        "Internal Error: Tried to created already existing workspace! {i}"
                    ),
                    Err(j) => {
                        //log::debug!("Creating Workspace: i={i} j={j}");
                        let nwk = create_workspace(i);

                        if nwk.0 == active_workspace.0 {
                            nwk.1.set_css_classes(&ACTIVE_WORKSPACE_CLASSES);
                        }

                        work_box.insert_child_after(&nwk.1, Some(&workspaces[j - 1].1));
                        workspaces.insert(j, nwk);
                    }
                },
                Event::DestroyWorkspace(i) => match workspaces.binary_search_by_key(&i, |w| w.0) {
                    Ok(j) => {
                        //log::debug!("Destroying Workspace: i={i}, j={j}");
                        let (_, wk) = workspaces.remove(j);
                        work_box.remove(&wk);
                    }
                    Err(_j) => log::warn!("Internal Error: Destroyed non-existant workspace: {i}"),
                },
                Event::Submap(map) => submap_label.set_label(map.as_str()),
                Event::None => {}
            };

            last_index += i + 1;

            copy_over = true;
        }

        if copy_over {
            buffer.copy_within(last_index.., 0);
            last_index = 0;
        }

        glib::ControlFlow::Continue
    });

    Ok(main)
}
