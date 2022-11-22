// region =/= Module Declaration =/=

mod error;
mod prelude;
mod util;

mod routes;
mod switch;

mod config;
mod model;
mod app;

// endregion

// region =/= Imports =/=

use crate::prelude::*;
use crate::app::AppComponent;

// endregion

// region =/= Main Function =/=

pub fn main() -> Result<()> {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<AppComponent>();
    
    Ok(())
}

// endregion
