cfg_if::cfg_if! {
    if #[cfg(feature = "hyprland")] {
        mod hyprland;
        pub use hyprland::element;
    } else {
        use anyhow::{anyhow, Result};
        pub fn element() -> Result<gtk::Box> {
            Err(anyhow!("No Workspace Widget enabled by build!"))
        }
    }
}
const ALPHA_CHAR: u32 = 'Α' as u32 - 1;

pub fn map_workspace(workspace: i32) -> String {
    match workspace {
        i @ 1..=17 => match char::from_u32(ALPHA_CHAR + i as u32) {
            Some(ch) => ch.to_string(),
            None => {
                log::warn!("Failed to map workspace to symbol: i={i}");
                format!("{}", i)
            }
        },
        // I needed to split this because there is a reserved character between rho and sigma.
        i @ 18..=24 => match char::from_u32((ALPHA_CHAR + 1) + i as u32) {
            Some(ch) => ch.to_string(),
            None => {
                log::warn!("Failed to map workspace to symbol: i={i}");
                format!("{}", i)
            }
        },
        i => format!("{}", i),
    }
}
