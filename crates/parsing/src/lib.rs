pub mod gogoanime {
    use scraper::{Html, Selector};

    pub type ParseResult<T> = Result<T, String>;

    #[derive(Debug)]
    pub struct Anime {
        slug: String,
        title: String,
    }

    #[derive(Debug)]
    pub struct AnimeDetails {
        id: usize,
        cover_image_url: String,
        summary: String,
        genres: Vec<String>,
        release_year: usize,
        default_episode: usize,
    }

    pub fn parse_anime_list(html: &str) -> ParseResult<Vec<Anime>> {
        let doc = Html::parse_document(html);
        let list_items_selector = Selector::parse(".listing li a").unwrap();

        doc.select(&list_items_selector)
            .map(|item| {
                // remove the first 10 chars, because each href starts with `/category/`
                let title = item.inner_html();
                let slug = item
                    .value()
                    .attr("href")
                    .ok_or(format!("Anime `{}` is missing href", &title))?[10..]
                    .to_string();

                Ok(Anime { slug, title })
            })
            .collect()
    }

    pub fn parse_anime_details(html: &str) -> ParseResult<AnimeDetails> {
        let doc = Html::parse_document(html);
        let id_selector = Selector::parse("#movie_id").unwrap();
        let cover_image_selector =
            Selector::parse(".anime_info_body_bg > img:nth-child(1)").unwrap();
        let summary_selector = Selector::parse("p.type:nth-child(5)").unwrap();
        let genres_selector = Selector::parse("p.type:nth-child(6) > a").unwrap();
        let released_selector = Selector::parse("p.type:nth-child(7)").unwrap();
        let default_episode_selector = Selector::parse("#default_ep").unwrap();

        let id = doc.select(&id_selector)
            .next()
            .ok_or("Id is missing")?
            .value()
            .attr("value")
            .ok_or("Id input is missing the value attribute")?
            .parse::<usize>()
            .map_err(|_| "Invalid id")?;

        let cover_image_url = doc.select(&cover_image_selector)
            .next()
            .ok_or("Cover image is missing")?
            .value()
            .attr("src")
            .ok_or("Cover image is missing the src attribute")?
            .to_string();

        let summary = doc.select(&summary_selector)
            .next()
            .ok_or("Summary is missing")?
            .text()
            .skip(1)
            .collect::<String>();

        let genres = doc.select(&genres_selector)
            .map(|e| e.value().attr("title").unwrap().to_string())
            .collect::<Vec<String>>();

        let release_year = doc.select(&released_selector)
            .next()
            .ok_or("Release year is missing")?
            .text()
            .skip(1)
            .collect::<String>()
            .parse::<usize>()
            .map_err(|_| "Not a valid release year")?;

        let default_episode = doc.select(&default_episode_selector)
            .next()
            .ok_or("Default episode is missing")?
            .value()
            .attr("value")
            .ok_or("Default episode input is missing the value attribute")?
            .parse::<usize>()
            .map_err(|_| "Invalid default episode")?;

        Ok(AnimeDetails {
            id,
            cover_image_url,
            summary,
            genres,
            release_year,
            default_episode
        })
    }
}
