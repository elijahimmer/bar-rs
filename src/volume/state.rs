use super::run_command;
use super::VOLUME_COMMAND;

const VOLUME_ICONS: [&str; 3] = ["󰕿", "󰖀", "󰕾"];
const VOLUME_CLAMP: usize = VOLUME_ICONS.len() - 1;
const VOLUME_MUTED: &str = "󰝟";
const VOLUME_OFF: &str = "󰸈";

#[derive(Clone, Copy)]
pub enum VolumeState {
    Volume(i32),
    Muted,
    Off,
}

impl VolumeState {
    pub fn to_str(&self) -> &'static str {
        match self {
            Self::Volume(percent) => {
                VOLUME_ICONS[((*percent as usize) / (100 / VOLUME_ICONS.len())).clamp(0, VOLUME_CLAMP)]
            }
            Self::Muted => VOLUME_MUTED,
            Self::Off => VOLUME_OFF,
        }
    }

    pub fn get() -> Self {
        match run_command(&["status"]) {
            Ok(str) => {
                // This is a simple- but breakable- way to avoid adding serde to
                // the program.
                const MATCH_STR: &str = "\"percentage\":";

                str.find(MATCH_STR).map_or_else(
                    || Self::Muted,
                    |index| {
                        let idx = index + MATCH_STR.len();
                        match str[idx..].find(',') {
                            Some(jdx) => str[idx..(idx + jdx)]
                                .parse()
                                .map_or_else(|err| {
                                    log::trace!("Failed to parse percent from {VOLUME_COMMAND}. error={err}");
                                    Self::Volume(100)
                                }, Self::Volume),
                            None => {
                              log::trace!("Failed to find percent's value from {VOLUME_COMMAND}. idx={idx}, str={str}");
                              Self::Muted
                            }
                        }
                    },
                )
            }
            Err(err) => {
                log::trace!("{VOLUME_COMMAND} Failed to execute. err={err}");
                Self::Off
            }
        }
    }
}
