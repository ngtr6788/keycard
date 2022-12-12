#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[tauri::command]
fn add_deck(deck_name: String, deck_description: String) {
    println!("Deck name: {deck_name}");
    println!("Deck description: {deck_description}");
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
        .invoke_handler(tauri::generate_handler![add_deck, add_card])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
