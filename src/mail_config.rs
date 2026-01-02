use std::{fs, path::PathBuf};

use anyhow::{Context, Ok};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct MailConfig {
    pub smtp_server: String,
    pub smtp_port: u16,
    pub email_address: String,
    pub password: String,
    pub sender_name: String,
}

impl Default for MailConfig {
    fn default() -> Self {
        Self {
            smtp_server: String::new(),
            smtp_port: 587,
            email_address: String::new(),
            password: String::new(),
            sender_name: String::new(),
        }
    }
}

impl MailConfig {
    pub fn config_path() -> anyhow::Result<PathBuf> {
        let config_dir = dirs::config_dir()
            .context("无法获取配置目录")?
            .join("batch_mail");

        fs::create_dir_all(&config_dir)?;

        Ok(config_dir.join("config.json"))
    }

    pub fn load() -> anyhow::Result<Self> {
        let path = Self::config_path()?;

        if !path.exists() {
            return Ok(Self::default());
        }

        let content = fs::read_to_string(&path).context("读取配置文件失败")?;
        let config = serde_json::from_str(&content).context("解析配置文件失败")?;

        Ok(config)
    }

    pub fn save(&self) -> anyhow::Result<()> {
        let path = Self::config_path()?;
        let json = serde_json::to_string_pretty(self).context("序列化配置失败")?;
        fs::write(&path, json).context("写入配置文件失败")?;
        eprintln!("配置已保存到: {:?}", path);
        Ok(())
    }

    pub fn validate(&self) -> anyhow::Result<()> {
        if self.smtp_server.is_empty() {
            anyhow::bail!("SMTP 服务器不能为空");
        }
        if self.smtp_port == 0 {
            anyhow::bail!("端口号无效");
        }
        if self.email_address.is_empty() {
            anyhow::bail!("邮箱地址不能为空");
        }
        if !self.email_address.contains('@') {
            anyhow::bail!("邮箱地址格式不正确");
        }
        if self.password.is_empty() {
            anyhow::bail!("密码不能为空");
        }
        Ok(())
    }
}
