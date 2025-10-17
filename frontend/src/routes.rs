use dioxus::prelude::*;

use components::Hero;
use views::{Blog, Home, Navbar};


#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
        #[route("/")]
        Home {},
        #[route("/blog/:id")]
        Blog { id: i32 },
}
