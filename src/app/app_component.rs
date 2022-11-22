#![allow(dead_code)] // For some reason, this is needed for no warnings on derive(Properties)

// region === Imports ===

use yew::prelude::*;
use yew_router::prelude::*;
use bounce::{BounceRoot, use_atom_value, helmet::HelmetBridge};

use crate::app::states::AppState;
use crate::routes::AppRoutes;
use crate::switch::render;

// endregion

// region === Router Component ===

#[function_component(RouterComponent)]
pub fn router_component() -> Html {
    let app_state = use_atom_value::<AppState>();
    html! {
        <div id="app-wrapper"
            class={classes!(
                if app_state.dark_mode { "dark" } else { "" },
                "flex",
                "min-w-full",
                "min-h-[100vh]",
            )}>
            <HelmetBridge default_title={ app_state.app_config.app_title.clone() } />
            <BrowserRouter>
                <Switch<AppRoutes> render={Switch::render(render)} />
            </BrowserRouter>
        </div>
           
    }
}

// endregion

// region =/= App Component =/=

#[function_component(AppComponent)]
pub fn app_component() -> Html {
    html! {
        <BounceRoot>
            <RouterComponent />
        </BounceRoot>
    }
}

// endregion
