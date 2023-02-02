// #[macro_use]
// extern crate log;
// extern crate rand;
use rand::Rng;

// use poise::extract_slash_argument;
// use poise::futures_util::future::ok;
use poise::serenity_prelude as serenity;
use serde::{Deserialize, Serialize};

use std::collections::HashMap;
use std::env;
// use std::fmt::format;
use std::fs::File;
// use std::hash::Hash;
use std::io::BufReader;
// use std::io::Write;

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
/// 組み合わせよう
async fn generating(ctx: Context<'_>) -> Result<(), Error> {
    // ファイルオープン
    let deserialized = file_opening().await;
    println!("{:?}", deserialized.menues["first"]);

    //乱数生成
    let random_tuple = generate_randnum().await;
    let first_len: usize = deserialized.menues["first"].len();
    let second_len: usize = deserialized.menues["second"].len();
    // TODO:ワードを選択
    let mut first_words: Vec<&String> = Vec::new();
    for (key, _) in deserialized.menues["first"].iter() {
        first_words.push(key);
    }
    let mut second_words: Vec<&String> = Vec::new();
    for (key, _) in deserialized.menues["second"].iter() {
        second_words.push(key);
    }

    let fword: &String = first_words[random_tuple.0 % first_len];
    let sword: &String = second_words[random_tuple.1 % second_len];
    // 返信
    let res = format!("{}{}", fword, sword);
    ctx.say(res).await?;
    Ok(())
}

/// 乱数生成
async fn generate_randnum() -> (usize, usize) {
    let mut rng = rand::thread_rng();
    let first_i: usize = rng.gen();
    let second_i: usize = rng.gen();
    (first_i, second_i)
}

/// ファイルオープン
async fn file_opening() -> Menues {
    let file_name = "menu.json";
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);
    let deserialized_file: Menues = serde_json::from_reader(reader).unwrap();
    deserialized_file
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
