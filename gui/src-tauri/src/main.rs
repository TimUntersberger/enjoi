#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use scraping::gogoanime;

#[tauri::command]
async fn gogoanime_search(text: String) -> Vec<gogoanime::SearchResult> {
    gogoanime::scrape_search(&text).await.unwrap_or_default()
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![gogoanime_search])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
