use crate::ui::pages::error_page::Err404;
use crate::ui::pages::home::Home;
use crate::ui::pages::vec_del::DelFromVec;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[derive(Routable, Clone, PartialEq)]
pub enum Route {
    // The home page is at the / route
    #[route("/")]
    Home {},
    #[route("/del-from-vec")]
    DelFromVec {},
    #[route("/:..segments")]
    Err404 { segments: Vec<String> },
}
