// region === Application Config ===

impl Default for AppConfig {
    fn default() -> Self {
        Self { 
            app_title: "Yew Demo App".to_string() 
        }
    }
}

// endregion

// region =/= AppConfig Struct =/=

#[derive(PartialEq)]
pub struct AppConfig {
    pub app_title: String,
}

// endregion
