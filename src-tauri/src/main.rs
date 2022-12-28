#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

pub mod models;
pub mod schema;

use diesel::sqlite::SqliteConnection;
use chrono::{Local, DateTime};
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use crate::models::{Deck, NewDeck, Card, KeyStringCard, NewCard};

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[tauri::command]
fn add_deck(deck_name: String, deck_description: String) {
    use crate::schema::decks::dsl::decks;
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
    use crate::schema::decks::dsl::decks;
    let connection = &mut establish_connection();
    let results = decks
        .load::<Deck>(connection)
        .expect("Error loading decks");
    
    results
}

#[tauri::command]
fn get_deck(deck_id: i32) -> Option<Deck> {
    use crate::schema::decks::id;
    use crate::schema::decks::dsl::decks;

    let connection = &mut establish_connection();
    let mut results = decks
        .filter(id.eq(deck_id))
        .load::<Deck>(connection)
        .expect("Error loading decks");
    
    if results.len() > 0 {
        let card = results.swap_remove(0);
        Some(card)
    } else {
        None
    }
}

#[tauri::command]
fn add_card(deck_id: i32, card_question: String, keys_list: Vec<String>) {
    use crate::schema::cards::dsl::cards;

    let keys_list_string: String = keys_list.join(",");

    let now = Local::now();
    let now_string = now.to_rfc3339();
        
    let new_card = NewCard {
        deck_id,
        card_question: &card_question,
        keys_list: &keys_list_string,
        successful_reviews: 0,
        efactor: 2.5,
        due_datetime: &now_string,
    };

    let connection = &mut establish_connection();
    diesel::insert_into(cards)
        .values(&new_card)
        .execute(connection);
}

#[tauri::command]
fn get_cards_from_deck(deck_id: i32) -> Vec<Card> {
    use crate::schema::cards;

    let connection = &mut establish_connection();
    let cards_list = cards::table
        .filter(cards::deck_id.eq(deck_id))
        .load::<KeyStringCard>(connection)
        .expect("Error loading cards");
    
    let mut cards_vec: Vec<Card> = cards_list.into_iter().map(|card| card.into()).collect(); 

    cards_vec
}

fn get_card(card_id: i32) -> Option<Card> {
    use crate::schema::cards;
    let connection = &mut establish_connection();
    let cards_list = cards::table
        .filter(cards::id.eq(card_id))
        .load::<KeyStringCard>(connection)
        .expect("Error loading cards");

    let mut cards_vec: Vec<Card> = cards_list.into_iter().map(|card| card.into()).collect(); 

    if cards_vec.len() > 0 {
        let card = cards_vec.swap_remove(0);
        Some(card)
    } else {
        None
    }
}

fn evaluate_answer(card_id: i32, answer_keys_list: Vec<String>) {
    let card_opt = get_card(card_id);

    if let Some(card) = card_opt {

    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![add_deck, get_decks, get_deck, add_card, get_cards_from_deck])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
