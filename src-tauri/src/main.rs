#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

pub mod models;
pub mod schema;

use diesel::sqlite::SqliteConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use crate::models::{Deck, NewDeck};
use crate::schema::decks::dsl::decks;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[tauri::command]
fn add_deck(deck_name: String, deck_description: String) {
    let new_deck = NewDeck { 
        deck_name: &deck_name, 
        deck_description: &deck_description 
    };

    let connection = &mut establish_connection();

    diesel::insert_into(decks)
        .values(&new_deck)
        .execute(connection);
}

#[tauri::command]
fn get_decks() -> Vec<Deck> {
    let connection = &mut establish_connection();
    let results = decks
        .load::<Deck>(connection)
        .expect("Error loading decks");
    
    results
}

#[tauri::command]
fn add_card(deck_name: String, card_description: String, keys_list: Vec<String>) {
    println!("Deck name: {deck_name}");
    println!("Card description: {card_description}");
    println!("Keys list: ");
    for key in keys_list {
        println!("{key} ");
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![add_deck, get_decks, add_card])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
