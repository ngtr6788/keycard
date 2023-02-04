#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

pub mod models;
pub mod schema;

use diesel::{sqlite::SqliteConnection, sql_query};
use diesel::sql_types::Integer;
use chrono::{Local, Duration};
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use strsim;
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
        interval: 0,
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
    
    let cards_vec: Vec<Card> = cards_list.into_iter().map(|card| card.into()).collect(); 

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

#[tauri::command]
fn get_first_card_by_date(deck_id: i32) -> Option<Card> {
    let connection = &mut establish_connection();

    let cards_list: Vec<KeyStringCard> = sql_query("SELECT * FROM cards 
                                                   WHERE deck_id = ?
                                                   ORDER BY datetime(due_datetime) 
                                                   LIMIT 1")
        .bind::<Integer, _>(deck_id)
        .load(connection)
        .expect("Error loading cards");
    
    let mut cards_vec: Vec<Card> = cards_list.into_iter().map(|card| card.into()).collect(); 

    if cards_vec.len() > 0 {
        let card = cards_vec.swap_remove(0);
        Some(card)
    } else {
        None
    }
}

/* evaulate_and_update_card uses a modified version of SM-2 algorithm
 * to determine when the next study time is.
 * Inspired by the many algorithms here: https://www.freshcardsapp.com/srs/simulator/
 */
#[tauri::command]
fn evaluate_and_update_card(card_id: i32, answer_keys_list: Vec<String>) {
    let card_opt = get_card(card_id);

    if let Some(card) = card_opt {
        let edit_distance = strsim::generic_levenshtein(&card.keys_list, &answer_keys_list);
        let edit_d_f32 = edit_distance as f32;
        
        let new_efactor = f32::max(1.3, card.efactor + (0.1 - edit_d_f32 * (0.08+ edit_d_f32 * 0.02)));

        let new_successful_reviews: i32;
        let duration: Duration;
        if edit_distance == 0 { // is correct
            new_successful_reviews = card.successful_reviews + 1; 
            if card.successful_reviews == 0 {
                duration = Duration::minutes(30);
            } else if card.successful_reviews == 1 {
                duration = Duration::hours(12);
            } else {
                duration = Duration::minutes(f32::ceil(card.interval as f32 * new_efactor) as i64);
            }
        } else {
            new_successful_reviews = 0;
            duration = Duration::minutes(2);
        }
    
        let new_interval = duration.num_minutes();
        let new_due_datetime = Local::now() + duration;
        let new_due_string = new_due_datetime.to_rfc3339();

        use crate::schema::cards;

        let connection = &mut establish_connection();
        diesel::update(cards::table.filter(cards::id.eq(card_id)))
            .set((
                cards::successful_reviews.eq(new_successful_reviews),
                cards::efactor.eq(new_efactor),
                cards::interval.eq(new_interval),
                cards::due_datetime.eq(new_due_string),
            ))
            .execute(connection);
    }
}

#[tauri::command]
fn edit_deck(deck_id: i32, deck_name: String, deck_description: String) {
    use crate::schema::decks;

    let connection = &mut establish_connection();

    diesel::update(decks::table.filter(decks::id.eq(deck_id)))
        .set((
            decks::deck_name.eq(deck_name),
            decks::deck_description.eq(deck_description)
        ))
        .execute(connection);
}

#[tauri::command]
fn delete_deck(deck_id: i32) {
    use crate::schema::decks;
    use crate::schema::cards;

    let connection = &mut establish_connection();

    diesel::delete(
        decks::table.filter(decks::id.eq(deck_id))
    ).execute(connection);

    diesel::delete(
        cards::table.filter(cards::deck_id.eq(deck_id))
    ).execute(connection);
}

#[tauri::command]
fn delete_card(card_id: i32) {
    use crate::schema::cards;

    let connection = &mut establish_connection();

    diesel::delete(
        cards::table.filter(cards::id.eq(card_id))
    ).execute(connection);
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            add_deck, 
            get_decks,
            get_deck, 
            add_card, 
            get_cards_from_deck,
            get_first_card_by_date,
            evaluate_and_update_card, 
            edit_deck,
            delete_deck,
            delete_card,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
