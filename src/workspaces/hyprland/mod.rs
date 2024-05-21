pub mod utils;
pub use utils::*;

use anyhow::{anyhow, Result};
use gtk::prelude::*;
use gtk::{glib, Box, Button, Label};
use std::io::{self, Read};
use std::os::unix::net::UnixStream;
use std::time::Duration;

pub type Workspace = (i32, Button);

const HYPR_STREAM_BUFFER_SIZE: usize = 1024;
const ACTIVE_WORKSPACE_CLASSES: [&str; 1] = ["active-workspace"];

// I don't know why someone would compile this on non-unix, but
//      this could help them I guess...
#[cfg(not(unix))]
pub fn element() -> Result<Box> {
    compile_error!("Hyprland Widget only works on unix! Disable the Hyprland Feature!");
}

#[cfg(unix)]
pub fn element() -> Result<Box> {
    if HIS.is_empty() || XDG_RUNTIME_DIR.is_empty() {
        return Err(anyhow!("Failed to create Hyprland Widget."));
    }

    log::trace!("Initializing Hyprland Widget");
    let mut hypr_listen_stream = UnixStream::connect(HYPR_DIR.clone() + ".socket2.sock")?;

    hypr_listen_stream.set_nonblocking(true)?;

    let main = Box::builder().build();

    let work_box = Box::builder().homogeneous(true).name("workspaces").build();
    let submap_label = Label::builder().name("submap").build();

    main.append(&work_box);
    main.append(&submap_label);

    let mut workspaces = match jumpstart_workspaces() {
        Ok(vec) => vec,
        Err(err) => {
            log::warn!("Failed to populate workspaces from hyprctl. error={err}");
            vec![create_workspace(1)]
        }
    };

    let mut active_workspace: Workspace = {
        let idx = match jumpstart_active_workspace() {
            Ok(idx) => idx,
            Err(err) => {
                log::warn!("Failed to get active workspace from hyprctl. error={err}");
                1
            }
        };

        let jdx = workspaces.binary_search_by_key(&idx, |w| w.0).unwrap_or(0);

        workspaces[jdx].clone()
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
        let size = match hypr_listen_stream.read(&mut buffer[last_index..]) {
            Ok(size) => size,
            Err(err) => {
                if err.kind() != io::ErrorKind::WouldBlock {
                    log::warn!("Failed to read from Hyprland IPC. error={err}");
                    error_counter += 1;

                    if error_counter > 10 {
                        log::error!(
                            "Failed to read from Hyprland IPC too many times. Killing widget"
                        );

                        m2.set_visible(false);

                        return glib::ControlFlow::Break;
                    }
                }
                return glib::ControlFlow::Continue;
            }
        };

        let message = String::from_utf8_lossy(&buffer[..size]);

        let mut copy_over = false;

        while let Some(idx) = message[last_index..].find('\n') {
            let m = &message[last_index..(last_index + idx)];
            last_index += idx + 1;

            if m.contains('\n') {
                log::warn!(
                    "Failed to capture newline correctly. (maybe invalid message) message=\"{m}\""
                );
                break;
            }

            let sep_idx: usize = match m.find(">>") {
                Some(idx) => idx,
                None => {
                    log::warn!(
                        "Failed to read hyprland message. (maybe invalid message) message=\"{m}\""
                    );
                    continue;
                }
            };

            let message = &m[..sep_idx];
            let args = &m[(sep_idx + 2)..];

            let parsed = match parse_message(message, args) {
                Ok(parsed) => parsed,
                Err(err) => {
                    log::warn!(
                        "Failed to parse hyprland message args. (maybe invalid message) error={err}, message=\"{message}\", args=\"{args}\""
                    );
                    continue;
                }
            };

            match parsed {
                Event::Workspace(wk_id) => {
                    //log::debug!("Switching Workspace: wk_id={wk_id}");
                    active_workspace.1.set_css_classes(&[]);
                    match workspaces.binary_search_by_key(&wk_id, |w| w.0) {
                        Ok(idx) => {
                            let wk = &workspaces[idx];

                            wk.1.set_css_classes(&ACTIVE_WORKSPACE_CLASSES);

                            active_workspace = wk.clone();
                        }
                        Err(idx) => {
                            log::debug!("Workspace not found in list: wk_id={wk_id}, idx={idx}");
                            // if not found in array, add it.
                            active_workspace = create_workspace(wk_id);
                            active_workspace
                                .1
                                .set_css_classes(&ACTIVE_WORKSPACE_CLASSES);

                            let sibling = idx.checked_sub(1).map(|i| &workspaces[i].1);

                            work_box.insert_child_after(&active_workspace.1, sibling);
                            workspaces.insert(idx, active_workspace.clone());
                        }
                    };
                }
                Event::CreateWorkspace(wk_id) => {
                    match workspaces.binary_search_by_key(&wk_id, |w| w.0) {
                        Ok(_idx) => {
                            log::debug!("Creating Workspace that already exists: wk_id={wk_id}");
                            /*Workspace already exists, so don't do anything*/
                        }
                        Err(idx) => {
                            log::debug!("Creating Workspace: wk_id={wk_id} idx={idx}");
                            let nwk = create_workspace(wk_id);

                            // This check should be redundent, but it doesn't hurt to keep it.
                            if nwk.0 == active_workspace.0 {
                                nwk.1.set_css_classes(&ACTIVE_WORKSPACE_CLASSES);
                            }

                            let sibling = idx.checked_sub(1).map(|i| &workspaces[i].1);

                            work_box.insert_child_after(&nwk.1, sibling);
                            workspaces.insert(idx, nwk);
                        }
                    }
                }
                Event::DestroyWorkspace(wk_id) => {
                    match workspaces.binary_search_by_key(&wk_id, |w| w.0) {
                        Ok(idx) => {
                            log::debug!("Destroying Workspace: wk_id={wk_id}, idx={idx}");
                            let (_, wk) = workspaces.remove(idx);
                            work_box.remove(&wk);
                        }
                        Err(_idx) => {
                            log::warn!("Destroyed non-existant workspace. wk_id={wk_id}")
                        }
                    }
                }
                Event::Submap(map) => submap_label.set_label(&map),
                Event::None => {}
            };

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

#[cfg(test)]
mod test {}
