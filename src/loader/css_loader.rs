use gtk4::gdk::Display;
use gtk4::CssProvider;
use std::path::Path;


use super::util::{SherlockError, SherlockFlags, SherlockErrorType};
use super::Loader;

impl Loader {
    pub fn load_css(sherlock_flags: &SherlockFlags) -> Result<Vec<SherlockError>, SherlockError> {
        let mut non_breaking: Vec<SherlockError> = Vec::new();
        let provider = CssProvider::new();
        
        // Load the default css
        provider.load_from_resource("/dev/skxxtz/sherlock/main.css");

        // Load the custom css
        if Path::new(&sherlock_flags.style).exists() {
            provider.load_from_path(&sherlock_flags.style);
        } else {
            non_breaking.push(SherlockError {
                error: SherlockErrorType::EnvVarNotFoundError(sherlock_flags.style.clone()),
                traceback: "Using default css".to_string(),
            });
        }

        let display = Display::default().ok_or_else(|| SherlockError {
            error: SherlockErrorType::DisplayError,
            traceback: "No display available".to_string(),
        })?;

        gtk4::style_context_add_provider_for_display(
            &display,
            &provider,
            gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
        Ok(non_breaking)
    }
}
