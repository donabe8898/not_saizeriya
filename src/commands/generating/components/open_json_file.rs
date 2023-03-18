use serde::{Deserialize, Serialize};

use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

#[derive(Serialize, Deserialize)]
pub struct MenuWords {
    pub menu_words: HashMap<String, HashMap<String, usize>>,
}

#[derive(Serialize, Deserialize)]
pub struct Menu {
    pub number: String,
    pub item: String,
    pub value: usize,
    pub alcohol: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Menues {
    pub menues: Vec<Menu>,
}

pub async fn open_menuwords() -> MenuWords {
    let file = File::open("menu_words.json").unwrap();
    let reader = BufReader::new(file);
    let deserialized_file: MenuWords = serde_json::from_reader(reader).unwrap();
    deserialized_file
}

pub async fn open_menu() -> Menues {
    let file = File::open("menu.json").unwrap();
    let reader = BufReader::new(file);
    let deserialized_file: Menues = serde_json::from_reader(reader).unwrap();
    deserialized_file
}
