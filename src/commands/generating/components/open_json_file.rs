use serde::{Deserialize, Serialize};

use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

#[derive(Serialize, Deserialize, Debug)]
pub struct Menues {
    pub menues: HashMap<String, HashMap<String, usize>>,
}

pub async fn file_opening() -> Menues {
    let file_name = "menu_words.json";
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);
    let deserialized_file: Menues = serde_json::from_reader(reader).unwrap();
    deserialized_file
}
