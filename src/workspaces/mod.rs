#[cfg_attr(feature = "hyprland", path = "hyprland/mod.rs")]
pub mod wk;

pub use wk::element;

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
