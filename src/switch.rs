//! Defines the components to be used for each route.

// region === Imports ===

use yew::{html, Html};

use crate::routes::AppRoutes;
use crate::app::components::{
    dashboard::DashboardComponent,
    register::RegisterComponent,
    login::LoginComponent,
    not_found::NotFoundComponent,
};

// endregion

// region === Render Mapping ===

pub fn render(route: &AppRoutes) -> Html {
    match route {
        AppRoutes::Dashboard => {html! { <DashboardComponent /> }},
        AppRoutes::Register => {html! { <RegisterComponent /> }},
        AppRoutes::Login => {html! { <LoginComponent /> }},
        AppRoutes::NotFound => {html! { <NotFoundComponent /> }},
    }
}

// endregion
