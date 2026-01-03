use std::path::PathBuf;

use gpui::{
    AppContext, AsyncApp, Context, Entity, EventEmitter, InteractiveElement, IntoElement,
    ParentElement, Render, StatefulInteractiveElement, Styled, WeakEntity, Window, div,
    prelude::FluentBuilder, rgb,
};
use gpui_component::{
    IconName, StyledExt,
    button::Button,
    input::{Input, InputState},
    label::Label,
    scroll::ScrollableElement,
};

use crate::{events::Events, mail_config::MailConfig, views::Views};

#[derive(Clone, Debug)]
enum SendingState {
    Idle,
    Sending,
    Success(String),
    Error(String),
}

pub struct HomeView {
    selected_file: Option<PathBuf>,
    html_content: Option<String>,
    recipients_input: Entity<InputState>,
    subject_input: Entity<InputState>,
    sending_state: SendingState,
}

impl HomeView {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let recipients_input = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("收件人列表 (每行一个邮箱地址)")
                .multi_line(true)
                .rows(10)
                .auto_grow(1, 10)
        });
        let subject_input = cx.new(|cx| InputState::new(window, cx).placeholder("邮件主题"));

        Self {
            selected_file: None,
            html_content: None,
            recipients_input,
            subject_input,
            sending_state: SendingState::Idle,
        }
    }

    fn select_file(&mut self, cx: &mut Context<Self>) {
        let task: gpui::Task<Option<rfd::FileHandle>> =
            cx.background_executor().spawn(async move {
                rfd::AsyncFileDialog::new()
                    .add_filter("HTML Files", &["html", "htm"])
                    .pick_file()
                    .await
            });

        cx.spawn(|weak_entity: WeakEntity<HomeView>, cx: &mut AsyncApp| {
            let mut cx = cx.clone();
            async move {
                if let Some(file_handle) = task.await {
                    let path = file_handle.path().to_path_buf();

                    match std::fs::read_to_string(&path) {
                        Ok(content) => {
                            if let Some(view) = weak_entity.upgrade() {
                                view.update(&mut cx, |this, cx| {
                                    this.selected_file = Some(path);
                                    this.html_content = Some(content);
                                    cx.notify();
                                })
                                .ok();
                            }
                        }
                        Err(e) => {
                            if let Some(view) = weak_entity.upgrade() {
                                view.update(&mut cx, |this, cx| {
                                    this.sending_state =
                                        SendingState::Error(format!("读取文件失败: {}", e));
                                    cx.notify();
                                })
                                .ok();
                            }
                        }
                    }
                }
            }
        })
        .detach();
    }

    fn send_email(&mut self, cx: &mut Context<Self>) {
        let html_content = match &self.html_content {
            Some(content) => content.clone(),
            None => {
                self.sending_state = SendingState::Error("请先选择 HTML 文件".to_string());
                cx.notify();
                return;
            }
        };

        let recipients_text = self.recipients_input.read(cx).value().to_string();
        let subject = self.subject_input.read(cx).value().to_string();

        if recipients_text.trim().is_empty() {
            self.sending_state = SendingState::Error("请输入收件人地址".to_string());
            cx.notify();
            return;
        }

        if subject.trim().is_empty() {
            self.sending_state = SendingState::Error("请输入邮件主题".to_string());
            cx.notify();
            return;
        }

        let config = match MailConfig::load() {
            Ok(cfg) => cfg,
            Err(e) => {
                self.sending_state = SendingState::Error(format!("加载配置失败: {}", e));
                cx.notify();
                return;
            }
        };

        if let Err(e) = config.validate() {
            self.sending_state = SendingState::Error(format!("配置验证失败: {}", e));
            cx.notify();
            return;
        }

        self.sending_state = SendingState::Sending;
        cx.notify();

        let task: gpui::Task<anyhow::Result<String>> = cx.background_executor().spawn(
            Self::send_email_async(config, recipients_text, subject, html_content),
        );

        cx.spawn(|view: WeakEntity<HomeView>, cx: &mut AsyncApp| {
            let mut cx = cx.clone();
            async move {
                match task.await {
                    Ok(success_msg) => {
                        view.update(&mut cx, |this, cx| {
                            this.sending_state = SendingState::Success(success_msg);
                            cx.notify();
                        })
                        .ok();
                    }
                    Err(e) => {
                        view.update(&mut cx, |this, cx| {
                            this.sending_state = SendingState::Error(format!("发送失败: {}", e));
                            cx.notify();
                        })
                        .ok();
                    }
                }
            }
        })
        .detach();
    }

    async fn send_email_async(
        config: MailConfig,
        recipients_text: String,
        subject: String,
        html_content: String,
    ) -> anyhow::Result<String> {
        use lettre::{
            Message, SmtpTransport, Transport,
            message::{Mailbox, header::ContentType},
            transport::smtp::authentication::Credentials,
        };

        let recipients: Vec<String> = recipients_text
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .map(|s| s.to_string())
            .collect();

        if recipients.is_empty() {
            anyhow::bail!("没有有效的收件人地址");
        }

        let from_mailbox: Mailbox = format!("{} <{}>", config.sender_name, config.email_address)
            .parse()
            .map_err(|e| anyhow::anyhow!("发件人地址格式错误: {}", e))?;

        let creds = Credentials::new(config.email_address.clone(), config.password.clone());

        let mailer = SmtpTransport::relay(&config.smtp_server)?
            .port(config.smtp_port)
            .credentials(creds)
            .build();

        let mut success_count = 0;
        let mut failed_recipients = Vec::new();

        for recipient in &recipients {
            let to_mailbox: Mailbox = recipient
                .parse()
                .map_err(|e| anyhow::anyhow!("收件人地址 {} 格式错误: {}", recipient, e))?;

            let email = Message::builder()
                .from(from_mailbox.clone())
                .to(to_mailbox)
                .subject(&subject)
                .header(ContentType::TEXT_HTML)
                .body(html_content.clone())?;

            match mailer.send(&email) {
                Ok(_) => success_count += 1,
                Err(e) => failed_recipients.push(format!("{}: {}", recipient, e)),
            }
        }

        if failed_recipients.is_empty() {
            Ok(format!("成功发送 {} 封邮件", success_count))
        } else if success_count > 0 {
            Ok(format!(
                "部分成功: {} 封成功, {} 封失败\n失败列表:\n{}",
                success_count,
                failed_recipients.len(),
                failed_recipients.join("\n")
            ))
        } else {
            anyhow::bail!("全部失败:\n{}", failed_recipients.join("\n"))
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

    fn render_file_section(&self, cx: &mut Context<Self>) -> impl IntoElement {
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
                div()
                    .flex()
                    .items_center()
                    .gap_3()
                    .child(
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
                            .child(file_name),
                    )
                    .child(
                        Button::new("select-file-btn")
                            .label("选择文件")
                            .on_click(cx.listener(|view, _, _, cx| {
                                view.select_file(cx);
                            })),
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
        let is_sending = matches!(self.sending_state, SendingState::Sending);

        div()
            .flex()
            .flex_col()
            .gap_4()
            .mt_4()
            .child(if is_sending {
                div()
                    .flex()
                    .justify_center()
                    .child(div().text_sm().text_color(rgb(0x71717a)).child("发送中..."))
            } else {
                div().flex().justify_center().child(
                    Button::new("send-btn")
                        .label("发送邮件")
                        .on_click(move |_, _, cx| {
                            view_handle.update(cx, |this, cx| {
                                this.send_email(cx);
                            });
                        }),
                )
            })
            .child(self.render_status_message())
    }

    fn render_status_message(&self) -> impl IntoElement {
        let msg = match &self.sending_state {
            SendingState::Idle => return div(),
            SendingState::Sending => (
                "正在发送邮件，请稍候...".to_string(),
                rgb(0x1e3a5f),
                rgb(0x3b82f6),
                rgb(0x60a5fa),
            ),
            SendingState::Success(msg) => {
                (msg.clone(), rgb(0x064e3b), rgb(0x10b981), rgb(0x34d399))
            }
            SendingState::Error(msg) => (msg.clone(), rgb(0x7f1d1d), rgb(0xef4444), rgb(0xf87171)),
        };

        div()
            .flex()
            .justify_center()
            .p_4()
            .bg(msg.1)
            .border_1()
            .border_color(msg.2)
            .rounded_lg()
            .child(div().text_sm().text_color(msg.3).child(msg.0))
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
                    .child(self.render_file_section(cx))
                    .child(self.render_email_info_section())
                    .child(self.render_action_section(cx)),
            )
    }
}
