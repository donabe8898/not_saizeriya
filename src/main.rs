use poise::serenity_prelude as serenity;

use std::env;

mod commands;

/// main関数
#[tokio::main]

async fn main() {
    dotenv::dotenv().ok();
    env_logger::init();
    let token = env::var("TOKEN").expect("missing token");
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![commands::generating::generating()], // Botに使いするコマンドのvector
            ..Default::default()
        })
        .token(token)
        .intents(serenity::GatewayIntents::non_privileged())
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(commands::generating::Data {})
            })
        });
    framework.run().await.unwrap();
}
