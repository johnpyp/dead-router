use std::env;

use anyhow::Context;
use dotenv::dotenv;
use env_logger::Env;
use log::error;

use crate::{checker::Checker, discord_alert::DiscordAlerter};

pub(crate) mod checker;
pub(crate) mod discord_alert;

async fn run() -> anyhow::Result<()> {
    let owner = env::var("DISCORD_OWNER_ID").context("Missing DISCORD_OWNER_ID")?;

    let webhook_url = env::var("DISCORD_WEBHOOK_URL").context("Missing DISCORD_WEBHOOK_URl")?;

    let check_interval = env::var("CHECK_INTERVAL").context("Missing check_interval")?;
    let check_interval: u64 = check_interval
        .parse()
        .context("Check interval not parseable as u64")?;

    let alerter = DiscordAlerter { owner, webhook_url };

    let check_urls: Vec<String> = vec![
        "https://google.com".to_string(),
        "https://cloudflare.com".to_string(),
    ];
    let checker = Checker {
        alerter,
        interval_ms: check_interval,
        urls: check_urls,
    };

    checker.run_loop().await;
    Ok(())
}
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let res = run().await;
    if let Err(err) = res {
        error!("Error: {:?} ", err);
    }

    Ok(())
}
