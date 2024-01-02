use gtk::{glib, CssProvider};
use std::fs;
use std::time::{Duration, SystemTime};

pub fn css() -> CssProvider {
    log::info!("Loading CSS");
    let css = CssProvider::new();
    css.connect_parsing_error(|_provider, section, error| {
        log::warn!(
            "CSS failed to parse: {} : {}",
            section.to_str(),
            error.message()
        );
    });

    if cfg!(any(debug_assertions, feature = "dynamic_css")) {
        // TODO: Replace this path so it isn't relative...
        //       but where is the question...
        const CSS_PATH: &str = "./css/style.css";

        log::info!("Dynamic CSS enabled by build.");
        let mut last_modified = SystemTime::now();
        css.load_from_path(CSS_PATH);

        let c2: CssProvider = css.clone();

        glib::timeout_add_local(Duration::from_secs(1), move || {
            match fs::metadata(&CSS_PATH) {
                Ok(m) => match m.modified() {
                    Ok(m) => {
                        if m > last_modified {
                            last_modified = m;

                            log::info!("Reloading CSS");
                            c2.load_from_path(CSS_PATH);
                        }
                    }

                    Err(e) => log::warn!("Couldn't get {CSS_PATH} Access Time: {e}"),
                },
                Err(e) => log::warn!("Couldn't get {CSS_PATH} metadata: {e}"),
            };

            glib::ControlFlow::Continue
        });
    } else {
        log::info!("Dynamic CSS disabled by build.");

        let css_str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/css/style.css"));

        css.load_from_string(css_str);
    };

    css
}
