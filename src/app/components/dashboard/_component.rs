#![allow(dead_code)] // For some reason, this is needed for no warnings on derive(Properties)

// region === Imports ===

use yew::prelude::*;

use crate::app::components::shared::{HeaderComponent, FooterComponent};

// endregion

// region === Component Definition ===

#[function_component(DashboardComponent)]
pub fn dashboard_component() -> Html {
    html! {
        <div id="dashboard-component"
            class={classes!(
                "flex",
            )}>
            <HeaderComponent />

            <FooterComponent />
        </div>
    }
}

// endregion
