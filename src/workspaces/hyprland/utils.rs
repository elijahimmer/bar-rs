use super::Workspace;
use anyhow::Result;
use gtk::prelude::*;
use gtk::Button;
use regex::Regex;
use std::process::Command;

pub fn create_workspace(n: i32) -> Workspace {
    let n_str = format!("{n}");
    let workspace_name = crate::workspaces::map_workspace(n);
    let button = Button::builder()
        .name(n_str.clone())
        .label(workspace_name)
        .build();

    button.connect_clicked(move |_| {
        if let Err(e) = Command::new("hyprctl")
            .args(["dispatch", "workspace", &n_str])
            .output()
        {
            log::warn!("Failed to execute Hyprctl: {e}");
        }
    });

    (n, button)
}

pub fn jumpstart_workspaces() -> Result<Vec<Workspace>> {
    let workspace_regex = Regex::new(r"\((\d+)\)").unwrap();

    let stdout = Command::new("hyprctl").arg("workspaces").output()?.stdout;

    let res = String::from_utf8(stdout)?;

    let mut v = vec![];
    for (_, [cap]) in workspace_regex.captures_iter(&res).map(|c| c.extract()) {
        v.push(create_workspace(cap.parse()?));
    }

    v.sort_unstable_by_key(|e| e.0);

    Ok(v)
}

pub fn jumpstart_active_workspace() -> Result<i32> {
    let re = Regex::new(r"\d+").unwrap();

    let res: String = String::from_utf8(
        Command::new("hyprctl")
            .args(["activeworkspace"])
            .output()?
            .stdout,
    )?;

    let mat = re.find(&res).unwrap();

    Ok((res[mat.range()]).parse::<i32>()?)
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
        "submap" => Ok(Event::Submap(String::from(val))),
        _ => Ok(Event::None),
    }
}
