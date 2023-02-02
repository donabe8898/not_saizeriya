mod components;

use components::{generate_random_num::generate_randnum, open_json_file::file_opening};

pub struct Data {}

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(slash_command, prefix_command)]
/// 組み合わせよう
pub async fn generating(ctx: Context<'_>) -> Result<(), Error> {
    // ファイルオープン
    let deserialized = file_opening().await;

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
