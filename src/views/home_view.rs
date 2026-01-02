use gpui::{EventEmitter, ParentElement, Render, Styled, div, rgb};
use gpui_component::{IconName, button::Button, label::Label};

use crate::{events::Events, views::Views};

pub struct HomeView;

impl HomeView {
    pub fn new() -> Self {
        Self {}
    }
}

impl EventEmitter<Events> for HomeView {}

impl Render for HomeView {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        let view_handle = cx.entity();
        div().relative().size_full().bg(rgb(0x18181b)).child(
            div()
                .flex()
                .justify_between()
                .p_4()
                .border_b_1()
                .border_color(rgb(0x27272a))
                .child(Label::new("Batch Mail"))
                .child(
                    Button::new("settings-btn")
                        .icon(IconName::Settings)
                        .on_click(move |_, _, cx| {
                            println!("setting button clicked");
                            view_handle.update(cx, |_, cx| {
                                cx.emit(Events::ViewChanged(Views::SettingsView));
                            })
                        }),
                ),
        )
    }
}
