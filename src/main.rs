//! 無いサイゼリヤのメニューを考える

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::hash::Hash;
use std::io::BufReader;
use std::io::Write;

#[derive(Serialize, Deserialize, Debug)]

struct Member {
    record: HashMap<String, usize>,
}

/// メインのかんすうー
fn main() {
    serialize_and_deserialize();
}

fn serialize_and_deserialize() {
    let member = Member {
        record: vec![("Donabe".to_string(), 1221), ("Minamin".to_string(), 0)]
            .into_iter()
            .collect(),
    };
    let serialized = serde_json::to_string(&member).unwrap();
    println!("serialized: {}", serialized);

    let deserialized: Member = serde_json::from_str(&serialized).unwrap();
    println!("deserialized: {:?}", deserialized);
}
