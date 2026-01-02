use gpui::{ParentElement, Render, div};
use gpui_component::label::Label;

pub struct SettingsView {}

impl SettingsView {
    pub fn new() -> Self {
        Self {}
    }
}

impl Render for SettingsView {
    fn render(
        &mut self,
        window: &mut gpui::Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        div().child(Label::new("这是设置界面"))
    }
}
