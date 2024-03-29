pub mod gogoanime {
    pub use parsing::gogoanime::*;
    use reqwest::{get as http_get, StatusCode};
    use urlencoding::encode as url_encode;

    pub type ScrapeResult<T> = Result<T, String>;

    pub const DOMAIN: &'static str = "www1.gogoanime.ai";

    pub async fn scrape_anime_list(page: usize) -> ScrapeResult<Vec<Anime>> {
        let html = http_get(format!("https://{}/anime-list.html?page={}", DOMAIN, page))
            .await
            .map_err(|e| e.to_string())?
            .text()
            .await
            .map_err(|e| e.to_string())?;

        parse_anime_list(&html)
    }

    pub async fn scrape_search(text: &str) -> ScrapeResult<Vec<SearchResult>> {
        let text = url_encode(text);
        let html = http_get(format!(
            "https://ajax.gogo-load.com/site/loadAjaxSearch?keyword={}&id=-1&link_web=https://{}/",
            text, DOMAIN
        ))
        .await
        .map_err(|e| e.to_string())?
        .text()
        .await
        .map_err(|e| e.to_string())?;

        parse_search_result(&html)
    }

    pub async fn scrape_anime_details(slug: &str) -> ScrapeResult<AnimeDetails> {
        let res =
            http_get(format!("https://{}/category/{}", DOMAIN, slug)).await.map_err(|e| e.to_string())?;

        match res.status() {
            StatusCode::OK => {
                let html = res.text().await.map_err(|e| e.to_string())?;

                parse_anime_details(&html)
            }
            StatusCode::NOT_FOUND => Err(format!("Anime not found: {}", slug)),
            code => Err(format!("Error {}", code)),
        }
    }

    pub async fn scrape_episode(slug: &str, episode: usize) -> ScrapeResult<Episode> {
        let res =
            http_get(format!("https://{}/{}-episode-{}", DOMAIN, slug, episode)).await.map_err(|e| e.to_string())?;

        match res.status() {
            StatusCode::OK => {
                let html = res.text().await.map_err(|e| e.to_string())?;

                parse_episode(&html)
            }
            StatusCode::NOT_FOUND => Err(format!("Anime not found: {}", slug)),
            code => Err(format!("Error {}", code)),
        }
    }
}
