#[macro_use]
extern crate log;
extern crate rand;

use rand::Rng;

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
#[derive(Serialize, Deserialize, Debug)]
struct Menues {
    menues: HashMap<String, HashMap<String, usize>>,
}

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(slash_command, prefix_command)]
/// ages
async fn age(
    ctx: Context<'_>,
    #[description = "select user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let res = format!("{}は {}にアカウントを作成しました.", u.name, u.created_at());
    ctx.say(res).await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
/// 本番コマンド
async fn generating(ctx: Context<'_>) -> Result<(), Error> {
    // TODO:ファイル開く処理. asyncにする
    // let file_name = "menu.json";
    // let file = File::open(file_name).unwrap();
    // let reader = BufReader::new(file);
    // let deserialized_file: Menues = serde_json::from_reader(reader).unwrap();

    //乱数生成
    let random_tuple = generate_randnum().await;
    // TODO:ワードを選択

    // 返信
    let res = format!("{},{}", random_tuple.0, random_tuple.1);
    ctx.say(res).await?;
    Ok(())
}

async fn generate_randnum() -> (usize, usize) {
    let mut rng = rand::thread_rng();
    let first_i: usize = rng.gen();
    let second_i: usize = rng.gen();
    (first_i, second_i)
}

/// main関数
#[tokio::main]

async fn main() {
    dotenv::dotenv().ok();
    env_logger::init();
    let token = env::var("TOKEN").expect("missing token");
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![generating()], // Botに使いするコマンドのvector
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
