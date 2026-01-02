use gpui::{AppContext, Context, Entity, EventEmitter, IntoElement, Render};

use crate::{
    events::Events,
    views::{HomeView, SettingsView, Views},
};

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

    fn observe_view<T: EventEmitter<Events>>(view: &Entity<T>, cx: &mut Context<Self>) {
        cx.subscribe(&view, |this, _, event, cx| match event {
            Events::ViewChanged(new_view) => {
                this.active_view = *new_view;
                cx.notify();
            }
        })
        .detach();
    }

    fn get_home_view(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        self.home_view
            .get_or_insert_with(|| {
                let view = cx.new(|_| HomeView::new());
                Self::observe_view(&view, cx);
                view
            })
            .clone()
    }

    fn get_settings_view(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        self.settings_view
            .get_or_insert_with(|| {
                let view = cx.new(|_| SettingsView::new());
                Self::observe_view(&view, cx);
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
