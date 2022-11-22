#![allow(dead_code)] // For some reason, this is needed for no warnings on derive(Properties)

// region === Imports ===

use yew::prelude::*;

// endregion

// region === Component Definition ===

#[function_component(FooterComponent)]
pub fn footer_component() -> Html {
    html! {
        <div id="footer-component"
            class={classes!(
                "flex",
            )}>

        </div>
    }
}

// endregion
