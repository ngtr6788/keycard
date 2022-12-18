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
}

#[derive(Serialize)]
pub struct Card {
  pub deck_id: i32,
  pub id: i32,
  pub card_question: String,
  pub keys_list: Vec<String>,
}

#[derive(Insertable)]
#[diesel(table_name = cards)]
pub struct NewCard<'a> {
  pub deck_id: i32,
  pub card_question: &'a str,
  pub keys_list: &'a str,
}