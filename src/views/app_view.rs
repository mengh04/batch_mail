use gpui::{AppContext, Context, Entity, IntoElement, Render};

use crate::views::{HomeView, SettingsView, Views};

pub struct AppView {
    pub active_view: Views,
    home_view: Option<Entity<HomeView>>,
    settings_view: Option<Entity<SettingsView>>,
}

impl AppView {
    pub fn new() -> Self {
        Self {
            active_view: Views::HomeView,
            home_view: None,
            settings_view: None,
        }
    }

    fn get_home_view(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        self.home_view
            .get_or_insert_with(|| {
                let view = cx.new(|_| HomeView::new());
                cx.subscribe(&view, |this, _, event, cx| match event {
                    crate::events::Events::ViewChanged(new_view) => {
                        this.active_view = *new_view;
                        cx.notify();
                    }
                })
                .detach();
                view
            })
            .clone()
    }

    fn get_settings_view(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        self.settings_view
            .get_or_insert_with(|| {
                let view = cx.new(|_| SettingsView::new());
                cx.subscribe(&view, |this, _, event, cx| match event {
                    crate::events::Events::ViewChanged(new_view) => {
                        this.active_view = *new_view;
                        cx.notify();
                    }
                })
                .detach();
                view
            })
            .clone()
    }
}

impl Render for AppView {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        match self.active_view {
            Views::HomeView => self.get_home_view(cx).into_any_element(),
            Views::SettingsView => self.get_settings_view(cx).into_any_element(),
        }
    }
}
