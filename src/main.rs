//! 無いサイゼリヤのメニューと1000円ガチャのDiscodBot

// mod commands;
mod generating;
mod support;

use poise::serenity_prelude as serenity;
use std::env;

pub struct Data {}

#[tokio::main]
/// mainメソッド
async fn main() {
    // envファイルオープン
    dotenv::dotenv().ok();
    env_logger::init();
    // envファイルからtokenを取得
    let token = env::var("TOKEN").expect("missing token");
    // Botフレームワーク定義
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                generating::generating(),
                generating::lots(),
                support::info(),
                support::help(),
            ], // Botに使うコマンドを列挙
            ..Default::default()
        })
        .token(token)
        .intents(serenity::GatewayIntents::non_privileged())
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(generating::Data {})
            })
        });
    framework.run().await.unwrap();
}
