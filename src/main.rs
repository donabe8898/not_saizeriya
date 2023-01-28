#[macro_use]
extern crate log;

use poise::extract_slash_argument;
use poise::futures_util::future::ok;
use poise::serenity_prelude as serenity;
use serde::{Deserialize, Serialize};

use std::collections::HashMap;
use std::env;
use std::fmt::format;
use std::fs::File;
use std::hash::Hash;
use std::io::BufReader;
use std::io::Write;

struct Data {}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(slash_command, prefix_command)]
async fn age(
    ctx: Context<'_>,
    #[description = "select user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let res = format!("{}'s account was created at {}", u.name, u.created_at());
    ctx.say(res).await?;
    Ok(())
}

/// main関数
#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    env_logger::init();
    let token = env::var("TOKEN").expect("missing token");
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![age()], // Botに使いするコマンドのvector
            ..Default::default()
        })
        .token(token)
        .intents(serenity::GatewayIntents::non_privileged())
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        });
    framework.run().await.unwrap();
}
