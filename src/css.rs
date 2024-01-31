use gtk::CssProvider;
pub fn css() -> CssProvider {
    log::info!("Loading CSS");

    let css = CssProvider::new();
    css.connect_parsing_error(|_provider, section, error| {
        log::warn!(
            "CSS failed to parse. section={}, error={}",
            section.to_str(),
            error.message()
        );
    });

    compile_css(&css);

    css
}

#[cfg(feature = "dynamic_css")]
fn compile_css(css: &CssProvider) {
    use gtk::glib;
    use std::fs;
    use std::time::SystemTime;

    const SCSS_PATH: &str = "./css/style.scss";

    log::info!("Dynamic CSS enabled by build.");
    let mut last_modified = SystemTime::now();
    match grass::from_path(SCSS_PATH, &grass::Options::default()) {
        Ok(str) => css.load_from_string(&str),

        Err(err) => {
            log::warn!("Failed to compile SCSS. error={err}");
        }
    };

    let c2: CssProvider = css.clone();

    glib::timeout_add_seconds_local(1, move || {
        match fs::metadata(&SCSS_PATH) {
            Ok(m) => match m.modified() {
                Ok(m) => {
                    if m > last_modified {
                        last_modified = m;

                        log::info!("Reloading CSS");
                        match grass::from_path(SCSS_PATH, &grass::Options::default()) {
                            Ok(str) => c2.load_from_string(&str),

                            Err(err) => {
                                log::warn!(
                                    "Failed to compile SCSS. SCSS_PATH={SCSS_PATH}, error={err}"
                                );
                            }
                        };
                    }
                }

                Err(err) => {
                    log::warn!("Couldn't get SCSS Access Time file={SCSS_PATH}, error={err}")
                }
            },
            Err(err) => log::warn!("Couldn't get SCSS Metadata file={SCSS_PATH}, error={err}"),
        };

        glib::ControlFlow::Continue
    });
}

#[cfg(not(feature = "dynamic_css"))]
fn compile_css(css: &CssProvider) {
    log::info!("Dynamic CSS disabled by build.");

    const SCSS_STR: &str = include_str!(concat!(env!("OUT_DIR"), "/style.css"));
    css.load_from_string(SCSS_STR);
}
