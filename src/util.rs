use anyhow::Result;

#[macro_export]
macro_rules! append_res {
    ($box:ident; $mod:ident) => {
        match $mod::element() {
            Ok(wgt) => $box.append(&wgt),
            Err(err) => log::warn!("Widget {} Disabled: {err}", stringify!($mod)),
        }
    };
    ($box:ident; $mod:ident, $($xs:ident),+) => {
        append_res!($box; $mod);
        append_res!($box; $($xs),+);
    };
}

#[inline]
pub fn read_f64(path: &str) -> Result<f64> {
    Ok(std::fs::read_to_string(path)?.trim().parse::<f64>()?)
}

#[inline]
pub fn read_trim(path: &str) -> Result<String> {
    Ok(std::fs::read_to_string(path).map(|s| s.trim().to_owned())?)
}
