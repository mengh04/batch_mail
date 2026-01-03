use std::path::PathBuf;

use gpui::{
    AppContext, Context, Entity, EventEmitter, InteractiveElement, IntoElement, ParentElement,
    Render, StatefulInteractiveElement, Styled, Window, div, prelude::FluentBuilder, rgb,
};
use gpui_component::{
    IconName, StyledExt,
    button::Button,
    input::{Input, InputState},
    label::Label,
    scroll::ScrollableElement,
};

use crate::{events::Events, views::Views};

pub struct HomeView {
    selected_file: Option<PathBuf>,
    recipients_input: Entity<InputState>,
    subject_input: Entity<InputState>,
}

impl HomeView {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let recipients_input = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("收件人列表 (每行一个邮箱地址)")
                .multi_line(true)
                .rows(10)
        });
        let subject_input = cx.new(|cx| InputState::new(window, cx).placeholder("邮件主题"));

        Self {
            selected_file: None,
            recipients_input,
            subject_input,
        }
    }

    fn render_header(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let view_handle = cx.entity();
        div()
            .flex()
            .justify_between()
            .p_4()
            .border_b_1()
            .border_color(rgb(0x27272a))
            .child(
                Label::new("Batch Mail")
                    .text_xl()
                    .font_semibold()
                    .text_color(rgb(0xe4e4e7)),
            )
            .child(
                Button::new("settings-btn")
                    .icon(IconName::Settings)
                    .on_click(move |_, _, cx| {
                        println!("setting button clicked");
                        view_handle.update(cx, |_, cx| {
                            cx.emit(Events::ViewChanged(Views::SettingsView));
                        })
                    }),
            )
    }

    fn render_file_section(&self) -> impl IntoElement {
        let file_name = self
            .selected_file
            .as_ref()
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "未选择文件".to_string());

        let file_path = self.selected_file.as_ref().map(|p| p.display().to_string());

        div()
            .flex()
            .flex_col()
            .gap_4()
            .p_4()
            .bg(rgb(0x27272a))
            .rounded_lg()
            .child(
                Label::new("选择 HTML 文件")
                    .text_color(rgb(0xe4e4e7))
                    .font_semibold()
                    .text_lg(),
            )
            .child(
                div().flex().items_center().gap_3().child(
                    div()
                        .flex_1()
                        .p_3()
                        .bg(rgb(0x18181b))
                        .rounded_md()
                        .text_sm()
                        .text_color(if self.selected_file.is_some() {
                            rgb(0xee4e4e7)
                        } else {
                            rgb(0x71717a)
                        })
                        .child(file_name)
                        .child(
                            Button::new("select-fild-btn")
                                .label("选择文件")
                                .on_click(|_, _, _| eprintln!("点击了选择文件按钮")),
                        ),
                ),
            )
            .when_some(file_path, move |this, path| {
                this.child(
                    div()
                        .text_xs()
                        .text_color(rgb(0x71717a))
                        .child(format!("路径: {}", path)),
                )
            })
    }

    fn render_email_info_section(&self) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_4()
            .p_4()
            .bg(rgb(0x27272a))
            .rounded_lg()
            .child(
                div()
                    .text_lg()
                    .font_semibold()
                    .text_color(rgb(0xe4e4e7))
                    .child("邮件信息"),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(
                        div()
                            .text_sm()
                            .font_semibold()
                            .text_color(rgb(0xe4e4e7))
                            .child("邮件主题"),
                    )
                    .child(Input::new(&self.subject_input)),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(
                        div()
                            .text_sm()
                            .font_semibold()
                            .text_color(rgb(0xe4e4e7))
                            .child("收件人"),
                    )
                    .child(Input::new(&self.recipients_input))
                    .child(
                        div()
                            .text_xs()
                            .text_color(rgb(0x71717a))
                            .child("收件人列表 (每行一个邮箱地址)"),
                    ),
            )
    }

    fn render_action_section(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let view_handle = cx.entity();
        div()
            .flex()
            .justify_center() // 居中显示
            .mt_4()
            .child(
                Button::new("send-btn")
                    .label("发送邮件")
                    .on_click(move |_, _, cx| {
                        view_handle.update(cx, |this, cx| {
                            this.send_emails(cx);
                        });
                    }),
            )
    }

    // 发送邮件的方法（先留空）
    fn send_emails(&mut self, cx: &mut Context<Self>) {
        eprintln!("准备发送邮件...");
        eprintln!("HTML 文件: {:?}", self.selected_file);
        eprintln!("主题: {}", self.subject_input.read(cx).value());
        eprintln!("收件人: {}", self.recipients_input.read(cx).value());

        // TODO: 实际的发送逻辑
    }
}

impl EventEmitter<Events> for HomeView {}

impl Render for HomeView {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        div()
            .id("home-view")
            .size_full()
            .bg(rgb(0x18181b))
            .flex()
            .flex_col()
            .child(self.render_header(cx))
            .child(
                div()
                    .id("body")
                    .flex()
                    .flex_col()
                    .gap_6()
                    .p_6()
                    .flex_1()
                    .overflow_y_scroll()
                    .overflow_scrollbar()
                    .child(self.render_file_section())
                    .child(self.render_email_info_section())
                    .child(self.render_action_section(cx)),
            )
    }
}
