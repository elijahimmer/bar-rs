use anyhow::Result;

#[inline]
pub fn read_f64(path: &str) -> Result<f64> {
    Ok(std::fs::read_to_string(path)?.trim().parse::<f64>()?)
}

#[inline]
pub fn read_trim(path: &str) -> Result<String> {
    Ok(std::fs::read_to_string(path).map(|s| s.trim().to_owned())?)
}
