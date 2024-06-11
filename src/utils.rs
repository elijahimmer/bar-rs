use anyhow::Result;

#[macro_export]
macro_rules! append_res {
    ($box:ident; $app:ident; $mod:ident) => {
        match $mod::new($app.clone()) {
            Ok(wgt) => $box.append(&wgt),
            Err(err) => log::error!("{} widget disabled. error={err}", stringify!($mod)),
        }
    };
    ($box:ident; $app:ident; $mod:ident, $($xs:ident),+) => {
        append_res!($box; $app; $mod);
        append_res!($box; $app; $($xs),+);
    };
}

pub fn read_f64(path: &str) -> Result<f64> {
    Ok(std::fs::read_to_string(path)?.trim().parse::<f64>()?)
}

pub fn read_trim(path: &str) -> Result<Box<str>> {
    Ok(std::fs::read_to_string(path).map(|s| s.trim().into())?)
}
