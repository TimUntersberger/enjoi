pub mod gogoanime {
    use parsing::gogoanime::*;
    use reqwest::{blocking::get as http_get, StatusCode};
    use urlencoding::encode as url_encode;

    pub type ScrapeResult<T> = Result<T, String>;

    pub const DOMAIN: &'static str = "www1.gogoanime.ai";

    pub fn scrape_anime_list(page: usize) -> ScrapeResult<Vec<Anime>> {
        let html = http_get(format!("https://{}/anime-list.html?page={}", DOMAIN, page))
            .map_err(|e| e.to_string())?
            .text()
            .map_err(|e| e.to_string())?;

        parse_anime_list(&html)
    }

    pub fn scrape_search(text: &str) -> ScrapeResult<Vec<SearchResult>> {
        let text = url_encode(text);
        let html = http_get(format!(
            "https://ajax.gogo-load.com/site/loadAjaxSearch?keyword={}&id=-1&link_web=https://{}/",
            text, DOMAIN
        ))
        .map_err(|e| e.to_string())?
        .text()
        .map_err(|e| e.to_string())?;

        parse_search_result(&html)
    }

    pub fn scrape_anime_details(slug: &str) -> ScrapeResult<AnimeDetails> {
        let res =
            http_get(format!("https://{}/category/{}", DOMAIN, slug)).map_err(|e| e.to_string())?;

        match res.status() {
            StatusCode::OK => {
                let html = res.text().map_err(|e| e.to_string())?;

                parse_anime_details(&html)
            }
            StatusCode::NOT_FOUND => Err(format!("Anime not found: {}", slug)),
            code => Err(format!("Error {}", code)),
        }
    }
}
