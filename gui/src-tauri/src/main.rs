#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use scraping::gogoanime;

#[tauri::command]
async fn gogoanime_search(text: String) -> Vec<gogoanime::SearchResult> {
    gogoanime::scrape_search(&text).await.unwrap_or_default()
}

#[tauri::command]
async fn gogoanime_details(slug: String) -> Option<gogoanime::AnimeDetails> {
    match gogoanime::scrape_anime_details(&slug).await {
        Ok(x) => Some(x),
        Err(msg) => {
            println!("[ERROR]: {}", msg);
            None
        }
    }
}

#[tauri::command]
async fn gogoanime_episode(slug: String, episode: usize) -> Option<gogoanime::Episode> {
    match gogoanime::scrape_episode(&slug, episode).await {
        Ok(x) => Some(x),
        Err(msg) => {
            println!("[ERROR]: {}", msg);
            None
        }
    }
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![gogoanime_search, gogoanime_details, gogoanime_episode])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
