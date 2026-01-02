use std::collections::HashMap;

use gpui::{AnyView, AppContext, Context, Entity, EventEmitter, Render, Window};

use crate::{
    events::Events,
    views::{HomeView, SettingsView, Views},
};

pub struct AppView {
    pub active_view: Views,
    views: HashMap<Views, AnyView>,
}

impl AppView {
    pub fn new() -> Self {
        Self {
            active_view: Views::HomeView,
            views: HashMap::new(),
        }
    }

    fn observe_view<T: EventEmitter<Events>>(view: &Entity<T>, cx: &mut Context<Self>) {
        cx.subscribe(view, |this, _, event, cx| match event {
            Events::ViewChanged(new_view) => {
                this.active_view = *new_view;
                cx.notify();
            }
        })
        .detach();
    }

    fn get_or_create_view(
        &mut self,
        window: &mut Window,
        view_type: Views,
        cx: &mut Context<Self>,
    ) -> AnyView {
        if let Some(view) = self.views.get(&view_type) {
            return view.clone();
        }

        let view: AnyView = match view_type {
            Views::HomeView => {
                let v = cx.new(|_| HomeView::new());
                Self::observe_view(&v, cx);
                v.into()
            }
            Views::SettingsView => {
                let v = cx.new(|cx| SettingsView::new(window, cx));
                Self::observe_view(&v, cx);
                v.into()
            }
        };

        self.views.insert(view_type, view.clone());
        view
    }
}

impl Render for AppView {
    fn render(
        &mut self,
        window: &mut gpui::Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        self.get_or_create_view(window, self.active_view, cx)
    }
}
