//! Definition of all error types for this application.

// region === Imports ===

use wasm_bindgen::JsValue;

// endregion

// region === Error Type Definitions ===

#[derive(thiserror::Error, Debug)]
pub enum Error {
    

    #[error("An unknown error occurred: {0}")]
    UnknownError(String),
} 

// endregion

// region =/= JS Error Conversion =/=

impl From<Error> for JsValue {
    fn from(error: Error) -> Self {
        JsValue::from_str(&error.to_string())
    }
}

// endregion
