// region === Imports ===

use bounce::Atom;

use crate::config::AppConfig;

// endregion

// region === State Definition ===

#[derive(Atom, PartialEq)]
pub struct AppState {
    pub app_config: AppConfig,
    pub dark_mode: bool,
} impl Default for AppState {
    fn default() -> Self {
        Self {
            app_config: AppConfig::default(),
            dark_mode: true,
        }
    }
}

// endregion
