use gpui::{EventEmitter, ParentElement, Render, div};
use gpui_component::{button::Button, label::Label};

use crate::events::Events;

pub struct SettingsView {}

impl SettingsView {
    pub fn new() -> Self {
        Self {}
    }
}

impl EventEmitter<Events> for SettingsView {}

impl Render for SettingsView {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        let view_handle = cx.entity();
        div()
            .child(
                Button::new("back-btn")
                    .label("返回")
                    .on_click(move |_, _, cx| {
                        println!("back button clicked");
                        // Here you would typically emit an event to change the view back to HomeView
                        view_handle.update(cx, |_, cx| {
                            cx.emit(crate::events::Events::ViewChanged(
                                crate::views::Views::HomeView,
                            ));
                        })
                    }),
            )
            .child(Label::new("Settings View"))
    }
}
