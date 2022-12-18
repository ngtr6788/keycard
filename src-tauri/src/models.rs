use diesel::prelude::*;
use serde::Serialize;
use crate::schema::decks;

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