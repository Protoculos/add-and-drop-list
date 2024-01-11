#![allow(non_snake_case)]

// use crate::app::Level;
use crate::models::item::ItemCard;
use crate::route::Route;
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use dioxus_signals::*;

#[derive(Clone, Copy, Default)]
pub struct ApplicationData {
    pub left_list: Signal<Vec<ItemCard>>,
    pub right_list: Signal<Vec<ItemCard>>,
}

impl ApplicationData {
    pub fn new(left: Signal<Vec<ItemCard>>, right: Signal<Vec<ItemCard>>) -> Self {
        Self {
            left_list: left,
            right_list: right,
        }
    }
}
pub fn use_app_data(cx: Scope) -> ApplicationData {
    *use_context(cx).unwrap()
}

pub fn App(cx: Scope) -> Element {
    let use_left_list: Signal<Vec<ItemCard>> =
        use_signal(cx, || (0..=3).map(|_| ItemCard::TextCard("Card")).collect());
    let use_right_list: Signal<Vec<ItemCard>> =
        use_signal(cx, || (0..=3).map(|_| ItemCard::TextCard("Card")).collect());
    use_context_provider(cx, || ApplicationData::new(use_left_list, use_right_list));

    tracing_wasm::set_as_global_default_with_config(
        tracing_wasm::WASMLayerConfigBuilder::new()
            .set_max_level(tracing::Level::TRACE)
            .build(),
    );

    render! { Router::<Route> {} }
}
