use anyhow::Ok;
use gpui::{AppContext, Application, WindowBounds, WindowOptions, px, size};
use gpui_component::Root;
use gpui_component_assets::Assets;

use crate::views::app_view::AppView;

mod events;
mod mail_config;
mod views;

fn main() {
    let app = Application::new().with_assets(Assets);
    app.run(move |cx| {
        gpui_component::init(cx);

        let window_bounds = WindowBounds::centered(size(px(400.), px(600.)), cx);

        cx.spawn(async move |cx| -> anyhow::Result<()> {
            cx.open_window(
                WindowOptions {
                    window_bounds: Some(window_bounds),
                    ..Default::default()
                },
                |window, cx| {
                    let main_view = cx.new(|_| AppView::new());
                    cx.new(|cx| Root::new(main_view, window, cx))
                },
            )?;
            Ok(())
        })
        .detach();
    });
}
