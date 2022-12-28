use chrono::{DateTime, FixedOffset};
use diesel::prelude::*;
use serde::Serialize;
use crate::schema::{decks, cards};

#[derive(Queryable, Serialize)]
pub struct Deck {
  pub id: i32,
  pub deck_name: String,
  pub deck_description: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = decks)]
pub struct NewDeck<'a> {
  pub deck_name: &'a str,
  pub deck_description: &'a str,
}

#[derive(Queryable)]
pub struct KeyStringCard {
  pub deck_id: i32,
  pub id: i32,
  pub card_question: String,
  pub keys_list: String,
  pub successful_reviews: i32,
  pub due_datetime: String,
  pub efactor: f32,
}

#[derive(Serialize)]
pub struct Card {
  pub deck_id: i32,
  pub id: i32,
  pub card_question: String,
  pub keys_list: Vec<String>,
  pub successful_reviews: i32,
  pub due_datetime: DateTime<FixedOffset>,
  pub efactor: f32,
}

impl From<KeyStringCard> for Card {
    fn from(card: KeyStringCard) -> Self {
        let keys_list_vec: Vec<String> = card.keys_list.split(",").map(|s| s.to_string()).collect();
        let due_datetime = DateTime::parse_from_rfc3339(&card.due_datetime).unwrap();

        Card {
            deck_id: card.deck_id,
            id: card.id, 
            card_question: card.card_question,
            keys_list: keys_list_vec,
            successful_reviews: card.successful_reviews,
            due_datetime,
            efactor: card.efactor,
        }
    }
}

#[derive(Insertable)]
#[diesel(table_name = cards)]
pub struct NewCard<'a> {
  pub deck_id: i32,
  pub card_question: &'a str,
  pub keys_list: &'a str,
  pub successful_reviews: i32,
  pub due_datetime: &'a str,
  pub efactor: f32,
}
