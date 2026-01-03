use gpui::{
    AppContext, Context, Entity, EventEmitter, InteractiveElement, IntoElement, ParentElement,
    Render, StatefulInteractiveElement, Styled, Window, div, rgb,
};
use gpui_component::{
    StyledExt,
    button::Button,
    input::{Input, InputState},
    label::Label,
    scroll::ScrollableElement,
};

use crate::{events::Events, mail_config::MailConfig};

pub struct SettingsView {
    config: MailConfig,
    smtp_server: Entity<InputState>,
    smtp_port: Entity<InputState>,
    email_address: Entity<InputState>,
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
            email_address: emil_address,
            password,
            sender_name,
        }
    }

    fn save_config(&mut self, cx: &mut Context<Self>) {
        let smtp_server = self.smtp_server.read(cx).value();
        let smtp_port = self.smtp_port.read(cx).value();
        let emil_address = self.email_address.read(cx).value();
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

    fn render_form_field(
        &self,
        label_text: impl Into<String>,
        input: &Entity<InputState>,
    ) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_2()
            .child(
                div()
                    .text_sm()
                    .font_semibold()
                    .text_color(rgb(0xe4e4e7))
                    .child(label_text.into()),
            )
            .child(Input::new(input))
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
            .bg(gpui::rgb(0x18181b))
            .flex()
            .flex_col()
            .child(
                div()
                    .flex()
                    .justify_between()
                    .p_4()
                    .border_b_1()
                    .border_color(gpui::rgb(0x27272a))
                    .child(Label::new("设置").text_xl().text_color(rgb(0xe4e4e7)))
                    .child(Button::new("back-btn").label("返回").on_click({
                        let view_handle = view_handle.clone();
                        move |_, _, cx| {
                            view_handle.update(cx, |_this, cx| {
                                cx.emit(Events::ViewChanged(crate::views::Views::HomeView));
                            });
                        }
                    })),
            )
            .child(
                div()
                    .id("form-container")
                    .flex()
                    .flex_col()
                    .gap_6()
                    .p_6()
                    .overflow_y_scroll()
                    .overflow_scrollbar()
                    .flex_1()
                    .child(self.render_form_field("SMTP 服务器", &self.smtp_server))
                    .child(self.render_form_field("SMTP 端口", &self.smtp_port))
                    .child(self.render_form_field("邮箱地址", &self.email_address))
                    .child(self.render_form_field("邮箱密码", &self.password))
                    .child(self.render_form_field("发件人名称", &self.sender_name))
                    .child(
                        div()
                            .mt_4()
                            .child(Button::new("confirm-btn").label("确定").on_click(
                                move |_, _, cx| {
                                    view_handle.update(cx, |this, cx| {
                                        this.save_config(cx);
                                        cx.emit(Events::ViewChanged(crate::views::Views::HomeView));
                                        eprintln!("配置已经保存");
                                    });
                                },
                            )),
                    ),
            )
    }
}
