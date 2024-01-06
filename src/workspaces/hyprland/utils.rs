use super::Workspace;
use anyhow::{anyhow, Result};
use gtk::prelude::*;
use gtk::Button;
use std::env;
use std::io::prelude::*;
use std::os::unix::net::UnixStream;

lazy_static::lazy_static! {
    pub static ref HIS: String = match env::var_os("HYPRLAND_INSTANCE_SIGNATURE")
        .ok_or(anyhow!(
            "Failed to get HYPRLAND_INSTANCE_SIGNATURE environment variable"
        )) {
            Ok(var) => match var.into_string() {
                Ok(string) => string,
                Err(err) => {
                    log::warn!("HYPRLAND_INSTANCE_SIGNATURE contains invalid unicode: {err:?}");
                    String::new()
                }
            },
            Err(err) => {
                log::warn!("Failed to get HYPRLAND_INSTANCE_SIGNATURE. error={err}");
                String::new()
            }
        };
    pub static ref HYPR_SOCKET_COMMAND: String =
        format!("/tmp/hypr/{}/.socket.sock", *HIS);
    pub static ref HYPR_SOCKET_LISTEN: String =
        format!("/tmp/hypr/{}/.socket2.sock", *HIS);
}

pub fn send_hypr_command(command: String) -> Result<()> {
    let mut hypr_command_stream = UnixStream::connect(HYPR_SOCKET_COMMAND.to_string())?;

    hypr_command_stream.write_all(command.as_bytes())?;

    let mut buf = [0; 16];

    let size = hypr_command_stream.read(&mut buf)?;

    if buf[..size] == *b"unknown request" {
        Err(anyhow!("Invaid Hyprland Command!"))
    } else {
        Ok(())
    }
}

pub fn send_hypr_command_read(command: String) -> Result<String> {
    let mut hypr_command_stream = UnixStream::connect(HYPR_SOCKET_COMMAND.to_string())?;

    hypr_command_stream.write_all(command.as_bytes())?;

    hypr_command_stream.flush()?;

    let mut buf = [0; 4096];

    let mut res = String::new();

    while let Ok(size) = hypr_command_stream.read(&mut buf) {
        if size == 0 {
            break;
        }
        res.push_str(&String::from_utf8_lossy(&buf[..size]));
    }

    if res == "unknown request" {
        Err(anyhow!("Invaid Hyprland Command!"))
    } else {
        Ok(res)
    }
}

pub fn create_workspace(n: i32) -> Workspace {
    let n_str = n.to_string();
    let workspace_name = crate::workspaces::map_workspace(n);
    let button = Button::builder()
        .name(n_str.clone())
        .label(workspace_name)
        .build();

    button.connect_clicked(move |_| {
        if let Err(err) = send_hypr_command(format!("dispatch workspace {n_str}")) {
            log::warn!("Failed to send/read command to/from Hyprland. error={err}");
        }
    });

    (n, button)
}

const CMD_LINE_START: &str = "workspace ID ";
const CMD_LINE_LEN: usize = CMD_LINE_START.len();
pub fn jumpstart_workspaces() -> Result<Vec<Workspace>> {
    let res = send_hypr_command_read("workspaces".into())?;

    let mut v = vec![];
    for line in res.lines() {
        if line.starts_with(CMD_LINE_START) {
            let pos = line[CMD_LINE_LEN..].find(' ').ok_or(anyhow!(""))?;

            v.push(create_workspace(
                line[CMD_LINE_LEN..CMD_LINE_LEN + pos].parse()?,
            ));
        }
    }

    v.sort_unstable_by_key(|e| e.0);

    Ok(v)
}

pub fn jumpstart_active_workspace() -> Result<i32> {
    let res = send_hypr_command_read("activeworkspace".into())?;

    let pos = match res[CMD_LINE_LEN..].find(' ') {
        Some(p) => p,
        None => {
            return Err(anyhow!("Failed to parse Hyprctl Response"));
        }
    };
    Ok(res[CMD_LINE_LEN..CMD_LINE_LEN + pos].parse()?)
}

#[derive(Debug, PartialEq)]
pub enum Event {
    Workspace(i32),
    CreateWorkspace(i32),
    DestroyWorkspace(i32),
    Submap(String),
    None,
}

pub fn parse_message(message: &str, val: &str) -> Result<Event> {
    match message {
        "workspace" => Ok(Event::Workspace(val.parse()?)),
        "createworkspace" => Ok(Event::CreateWorkspace(val.parse()?)),
        "destroyworkspace" => Ok(Event::DestroyWorkspace(val.parse()?)),
        "submap" => Ok(Event::Submap(val.to_string())),
        _ => Ok(Event::None),
    }
}
