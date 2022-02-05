use chrono::prelude::*;
use humantime::format_duration;
use log::{error, info};
use reqwest::Client;
use tokio::time::{interval, sleep, Duration};

use crate::discord_alert::DiscordAlerter;

pub enum CheckState {
    Failing {
        failing: Vec<String>,
        started: DateTime<Utc>,
    },
    Up,
}
pub struct Checker {
    pub alerter: DiscordAlerter,
    pub interval_ms: u64,
    pub urls: Vec<String>,
}

impl Checker {
    pub async fn run_loop(&self) {
        let period = Duration::from_millis(self.interval_ms);

        let mut interval = interval(period);

        let mut check_state = CheckState::Up;

        sleep(Duration::from_secs(5)).await;
        loop {
            interval.tick().await;
            info!("Checking {} urls...", &self.urls.len());
            let mut failing_urls = vec![];
            for url in &self.urls {
                let url_res = self.check_url(url.clone()).await;
                if !url_res {
                    failing_urls.push(url.clone())
                }
            }

            if !failing_urls.is_empty() {
                match &check_state {
                    CheckState::Up => {
                        error!("Entering failure state !!");
                        check_state = CheckState::Failing {
                            started: Utc::now(),
                            failing: failing_urls.clone(),
                        }
                    }
                    CheckState::Failing { started, .. } => {
                        error!("Still failing");
                        check_state = CheckState::Failing {
                            started: *started,
                            failing: failing_urls.clone(),
                        }
                    }
                }
            } else {
                match &check_state {
                    CheckState::Up => {}
                    CheckState::Failing { failing, started } => {
                        info!("Stopped failing!");

                        let since_display = started.to_rfc3339_opts(SecondsFormat::Secs, true);
                        let time_diff = Duration::from_millis(
                            Utc::now()
                                .signed_duration_since(*started)
                                .num_milliseconds() as u64,
                        );
                        let formatted_duration = format_duration(time_diff).to_string();
                        self.alerter
                            .error(format!(
                                "{} url(s) checks failed for {} (since {}): \n{}",
                                failing.len(),
                                formatted_duration,
                                since_display,
                                failing.join("\n"),
                            ))
                            .await;

                        check_state = CheckState::Up;
                    }
                }
            }
        }
    }

    pub async fn check_url(&self, url: String) -> bool {
        let result = Client::new().get(url).send().await;
        result.is_ok()
    }
}
