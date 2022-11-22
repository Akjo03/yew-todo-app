//! Definition of all routes for this application.

// region === Imports ===

use yew_router::prelude::*;

// endregion

// region === Route Definitions ===

#[derive(Clone, Routable, PartialEq)]
pub enum AppRoutes {
    #[at("/")]
    Dashboard,
    #[at("/register")]
    Register,
    #[at("/login")]
    Login,

    #[not_found]
    #[at("/404")]
    NotFound,
}

// endregion
