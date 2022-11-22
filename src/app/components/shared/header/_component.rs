#![allow(dead_code)] // For some reason, this is needed for no warnings on derive(Properties)

// region === Imports ===

use yew::prelude::*;

// endregion

// region === Component Definition ===

#[function_component(HeaderComponent)]
pub fn header_component() -> Html {
    html! {
        <div id="header-component"
            class={classes!(
                "flex",
            )}>

        </div>
    }
}

// endregion
