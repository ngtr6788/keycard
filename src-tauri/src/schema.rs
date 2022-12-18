// @generated automatically by Diesel CLI.

diesel::table! {
    decks (id) {
        id -> Integer,
        deck_name -> Text,
        deck_description -> Nullable<Text>,
    }
}
