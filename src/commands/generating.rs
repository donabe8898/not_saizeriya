mod components;

use components::{
    generate_random_num::generate_randnum, open_json_file::open_menu,
    open_json_file::open_menuwords,
};

// use serde::de::value;

pub struct Data {}

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(slash_command, prefix_command)]
/// 組み合わせよう
pub async fn generating(ctx: Context<'_>) -> Result<(), Error> {
    // ファイルオープン
    let deserialized = open_menuwords().await;

    //乱数生成
    let random_tuple = generate_randnum().await;

    let first_len: usize = deserialized.menu_words["first"].len();

    let second_len: usize = deserialized.menu_words["second"].len();
    // TODO:ワードを選択

    let mut first_words: Vec<&String> = Vec::new();

    for (key, _) in deserialized.menu_words["first"].iter() {
        first_words.push(key);
    }

    let mut second_words: Vec<&String> = Vec::new();

    for (key, _) in deserialized.menu_words["second"].iter() {
        second_words.push(key);
    }

    let fword: &String = first_words[random_tuple.0 % first_len];

    let sword: &String = second_words[random_tuple.1 % second_len];
    // 返信
    let res = format!("{}{}", fword, sword);
    ctx.say(res).await?;
    Ok(())
}
#[poise::command(slash_command, prefix_command)]
// 1000円ガチャ
pub async fn lots(ctx: Context<'_>) -> Result<(), Error> {
    let deserialized_vec = open_menu().await;

    let vec_length = deserialized_vec.menues.len();

    let mut menues_struct = Vec::new();

    for j in deserialized_vec.menues.iter() {
        menues_struct.push((&j.number, &j.item, &j.value));
    }

    let mut selected_menues = Vec::new();

    let mut balance: isize = 1000;

    loop {
        // 乱数生成
        let random_number = generate_randnum().await;

        let index = random_number.0 % vec_length;

        let selected_menu_number = menues_struct[index].0;

        let selected_menu_item = menues_struct[index].1;

        let selected_menu_value = menues_struct[index].2;

        if balance - selected_menu_value >= 0 {
            selected_menues.push((
                selected_menu_number,
                selected_menu_item,
                selected_menu_value,
            ));
            balance -= selected_menu_value;
        }
        if balance == 0 {
            break;
        }
    }

    let mut res = String::new();

    for (num, itm, val) in selected_menues {
        println!("{} {} {}\n", num, itm, val);
        let fm = &format!("{} {} {}円\n", num, itm, val);
        res += fm;
    }
    ctx.say(res).await?;
    Ok(())
}
