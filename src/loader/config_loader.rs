use std::fs;

use super::util::{Config, SherlockError, SherlockErrorType, SherlockFlags};
use super::Loader;

impl Loader {
    pub fn load_config(
        sherlock_flags: &SherlockFlags,
    ) -> Result<(Config, Vec<SherlockError>), SherlockError> {
        match fs::read_to_string(&sherlock_flags.config) {
            Ok(config_str) => Ok((
                toml::de::from_str(&config_str).map_err(|e| SherlockError {
                    error: SherlockErrorType::FileParseError(sherlock_flags.config.to_string()),
                    traceback: e.to_string(),
                })?,
                vec![],
            )),
            Err(e) => match e.kind() {
                std::io::ErrorKind::NotFound => {
                    let mut non_breaking = vec![SherlockError {
                        error: SherlockErrorType::FileExistError(sherlock_flags.config.to_string()),
                        traceback: Default::default(),
                    }];

                    // Unpack non-breaking errors and default config
                    let (config, n) = Config::default();
                    non_breaking.extend(n);
                    Ok((config, non_breaking))
                }
                _ => Err(SherlockError {
                    error: SherlockErrorType::FileReadError(sherlock_flags.config.to_string()),
                    traceback: e.to_string(),
                })?,
            },
        }
    }
}
