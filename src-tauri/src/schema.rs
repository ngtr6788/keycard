// @generated automatically by Diesel CLI.

diesel::table! {
    cards (id) {
        deck_id -> Integer,
        id -> Integer,
        card_question -> Text,
        keys_list -> Text,
    }
}

diesel::table! {
    decks (id) {
        id -> Integer,
        deck_name -> Text,
        deck_description -> Nullable<Text>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    cards,
    decks,
);
