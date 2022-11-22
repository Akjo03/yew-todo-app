#![allow(dead_code)] // For some reason, this is needed for no warnings on derive(Properties)

// region === Imports ===

use yew::prelude::*;

// endregion

// region === Component Definition ===

#[function_component(NotFoundComponent)]
pub fn not_found_component() -> Html {
    html! {
        <p class={classes!("text-center", "text-2xl", "mt-4")}>{ "404 - Page Not Found" }</p>
    }
}

// endregion
