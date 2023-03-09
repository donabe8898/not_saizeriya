use super::generating;
use std::fs::File;
use std::io::{BufRead, BufReader};

type Error = generating::Error;
type Context<'a> = poise::Context<'a, generating::Data, Error>;

#[poise::command(slash_command, subcommands("version", "pacman"))]
/// infomation and other...
pub async fn info(_ctx: Context<'_>, arg: String) -> Result<(), Error> {
    let str_arg: &str = &arg;
    match str_arg {
        "version" => version(),
        "pacman" => pacman(),
        _ => error_message(),
    };
    Ok(())
}

#[poise::command(slash_command)]
pub async fn version(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("not_saizeriya 0.4.0").await?;
    Ok(())
}
#[poise::command(slash_command)]
pub async fn pacman(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("何もすることがありません").await?;
    Ok(())
}

#[poise::command(slash_command)]
/// help!!!!
pub async fn help(ctx: Context<'_>) -> Result<(), Error> {
    let path: &str = "help.txt";
    let input: File = File::open(path)?;
    let buffered: BufReader<File> = BufReader::new(input);
    let mut res: String = String::new();
    for line in buffered.lines() {
        res += &line?;
        res += "\n";
    }
    ctx.say(res).await?;
    Ok(())
}

#[poise::command(slash_command)]
pub async fn error_message(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("エラーが発生しました。\n引数を正しく入力しているかどうか確認してください。")
        .await?;
    Ok(())
}
