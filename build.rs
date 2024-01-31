use std::env;
use std::fs;
use std::path::Path;

fn main() {
    #[cfg(not(feature = "dynamic-css"))]
    compile_css();
}

const SCSS_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/css/style.scss");
const SCSS_STR: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/css/style.scss"));

#[cfg(not(feature = "dynamic-css"))]
fn compile_css() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("style.css");
    match grass::from_string(SCSS_STR, &grass::Options::default()) {
        Ok(str) => fs::write(dest_path, str).unwrap(),
        Err(err) => {
            eprintln!("Failed to compile SCSS. file={SCSS_PATH}, error={err}");
        }
    };
    println!("cargo:rerun-if-changed={SCSS_PATH}");
}
