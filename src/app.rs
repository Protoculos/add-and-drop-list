#![allow(non_snake_case)]

use crate::models::item::ItemCard;
use crate::route::Route;
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use dioxus_signals::*;

#[derive(Clone, Copy, Default)]
pub struct ApplicationData {
    pub left_vec: Signal<Vec<ItemCard>>,
    pub right_vec: Signal<Vec<ItemCard>>,
}

pub fn use_app_data(cx: Scope) -> ApplicationData {
    *use_context(cx).unwrap()
}

pub fn App(cx: Scope) -> Element {
    use_context_provider(cx, ApplicationData::default);

    render! { Router::<Route> {} }
}
