#[cfg(feature = "hyprland")]
mod hyprland;

#[cfg(feature = "hyprland")]
pub use hyprland::element;

#[cfg(not(feature = "hyprland"))]
pub fn element() -> Option<gtk::Box> {
    None
}

const ALPHA_CHAR: u32 = 912; // the unicode character Alpha
const SIGMA_CHAR: u32 = ALPHA_CHAR + 19; // the unicode character Sigma

pub fn map_workspace(workspace: i32) -> String {
    match workspace {
        // I needed to split this because there is a reserved character between rho and sigma.
        i @ 1..=17 => char::from_u32(ALPHA_CHAR + i as u32).unwrap().to_string(),
        i @ 19..=25 => char::from_u32(SIGMA_CHAR + i as u32).unwrap().to_string(),
        i => format!("{}", i),
    }
}

//There is a
