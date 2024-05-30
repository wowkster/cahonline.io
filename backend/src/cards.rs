use std::path::Path;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CardSet {
    pub name: String,
    pub description: Option<String>,
    pub official: bool,
    pub white: Vec<WhiteCard>,
    pub black: Vec<BlackCard>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WhiteCard {
    pub text: String,
    pub pack: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BlackCard {
    pub text: String,
    pub pick: u32,
    pub pack: usize,
}

pub fn load_card_data<P: AsRef<Path>>(data_path: P) -> std::io::Result<Vec<CardSet>> {
    let data = std::fs::read_to_string(data_path)?;

    let card_sets: Vec<CardSet> =
        serde_json::from_str(&data).expect("Failed to parse card data file");

    Ok(card_sets)
}
