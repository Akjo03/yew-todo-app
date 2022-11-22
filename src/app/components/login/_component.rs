#![allow(dead_code)] // For some reason, this is needed for no warnings on derive(Properties)

// region === Imports ===

use yew::prelude::*;

use crate::app::components::shared::{HeaderComponent, FooterComponent};

// endregion

// region === Component Definition ===

#[function_component(LoginComponent)]
pub fn login_component() -> Html {
    html! {
        <div id="login-component"
            class={classes!(
                "flex",
            )}>
            <HeaderComponent />

            <FooterComponent />
        </div>
    }
}

// endregion
