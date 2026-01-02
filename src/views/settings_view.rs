use gpui::{
    AppContext, Context, Entity, EventEmitter, InteractiveElement, IntoElement, ParentElement,
    Render, StatefulInteractiveElement, Styled, Window, div,
};
use gpui_component::{
    button::Button,
    input::{Input, InputState},
    scroll::ScrollableElement,
};

use crate::{events::Events, mail_config::MailConfig};

pub struct SettingsView {
    config: MailConfig,
    smtp_server: Entity<InputState>,
    smtp_port: Entity<InputState>,
    emil_address: Entity<InputState>,
    password: Entity<InputState>,
    sender_name: Entity<InputState>,
}

impl SettingsView {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let config = MailConfig::load().unwrap_or_default();
        let smtp_server = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("SMTP 服务器地址")
                .default_value(&config.smtp_server)
        });
        let smtp_port = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("SMTP 端口")
                .default_value(&config.smtp_port.to_string())
        });
        let emil_address = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("邮箱地址")
                .default_value(&config.email_address)
        });
        let password = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("邮箱密码")
                .default_value(&config.password)
        });
        let sender_name = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("发件人名称")
                .default_value(&config.sender_name)
        });
        Self {
            config,
            smtp_server,
            smtp_port,
            emil_address,
            password,
            sender_name,
        }
    }

    fn save_config(&mut self, cx: &mut Context<Self>) {
        let smtp_server = self.smtp_server.read(cx).value();
        let smtp_port = self.smtp_port.read(cx).value();
        let emil_address = self.emil_address.read(cx).value();
        let password = self.password.read(cx).value();
        let sender_name = self.sender_name.read(cx).value();

        self.config = MailConfig {
            smtp_server: smtp_server.to_string(),
            smtp_port: smtp_port.parse().unwrap_or(587),
            email_address: emil_address.to_string(),
            password: password.to_string(),
            sender_name: sender_name.to_string(),
        };

        match self.config.save() {
            Ok(_) => eprintln!("配置保存成功"),
            Err(e) => eprintln!("配置保存失败: {:?}", e),
        }
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
            .id("settings-view")
            .size_full()
            .overflow_scroll()
            .overflow_scrollbar()
            .child(Input::new(&self.smtp_server))
            .child(Input::new(&self.smtp_port))
            .child(Input::new(&self.password))
            .child(Input::new(&self.sender_name))
            .child(
                Button::new("confirm-btn")
                    .label("确定")
                    .on_click(move |_, _, cx| {
                        view_handle.update(cx, |this, cx| {
                            this.save_config(cx);
                            cx.emit(Events::ViewChanged(crate::views::Views::HomeView));
                            eprintln!("配置已经保存");
                        });
                    }),
            )
    }
}
