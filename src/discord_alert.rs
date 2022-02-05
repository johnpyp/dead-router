use anyhow::Result;
use log::error;
use reqwest::Client;
use serde_json::json;

pub enum AlertLevels {
    Error,
    Warn,
    Info,
}

pub struct Alert {
    level: AlertLevels,
    message: String,
}

pub struct DiscordAlerter {
    pub webhook_url: String,
    pub owner: String,
}

impl DiscordAlerter {
    pub async fn info(&self, message: String) {
        self.alert(Alert {
            level: AlertLevels::Info,
            message,
        })
        .await;
    }
    pub async fn warn(&self, message: String) {
        self.alert(Alert {
            level: AlertLevels::Warn,
            message,
        })
        .await;
    }
    pub async fn error(&self, message: String) {
        self.alert(Alert {
            level: AlertLevels::Error,
            message,
        })
        .await;
    }
    pub async fn alert(&self, alert: Alert) {
        let owner_mention = format!("<@{}>", &self.owner);
        let webhook_url = self.webhook_url.clone();

        let body = match alert.level {
            AlertLevels::Error => {
                json!({
                    "content": owner_mention,
                    "embeds": [
                        {
                            "title": "ERROR",
                            "description": alert.message,
                            "color": 0xff0000_i32
                        }
                    ]
                })
            }
            AlertLevels::Warn => {
                json!({
                    "content": format!("**WARNING** {}", alert.message),
                })
            }
            AlertLevels::Info => {
                json!({
                    "content": format!("**INFO** {}", alert.message),
                })
            }
        };
        let response = Client::new().post(webhook_url).json(&body).send().await;

        if let Err(e) = response {
            error!("Error sending discord alert: {:?}", e)
        };
    }
}
