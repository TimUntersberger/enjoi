use scraper::{ElementRef, Html, Selector};

pub type ParseResult<T> = Result<T, String>;

#[derive(Debug, serde::Serialize)]
pub struct Anime {
    pub slug: String,
    pub title: String,
}

#[derive(Debug, serde::Serialize)]
pub struct SearchResult {
    pub slug: String,
    pub title: String,
    pub cover_image_url: String,
}

#[derive(Debug, serde::Serialize)]
pub struct AnimeDetails {
    pub id: usize,
    pub title: String,
    pub cover_image_url: String,
    pub summary: String,
    pub genres: Vec<String>,
    pub release_year: usize,
    pub default_episode: usize,
    pub episode_count: usize,
}

#[derive(Debug, serde::Serialize)]
pub struct Episode {
    pub providers: Vec<(String, String)>,
}

pub fn parse_anime_list(html: &str) -> ParseResult<Vec<Anime>> {
    let doc = Html::parse_document(html);
    let list_items_selector = Selector::parse(".listing li a").unwrap();

    doc.select(&list_items_selector)
        .map(|item| {
            let title = item.inner_html();
            // remove the first 10 chars, because each href starts with `/category/`
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
    let title_selector = Selector::parse(".anime_info_body_bg > h1:nth-child(2)").unwrap();
    let cover_image_selector = Selector::parse(".anime_info_body_bg > img:nth-child(1)").unwrap();
    let summary_selector = Selector::parse("p.type:nth-child(5)").unwrap();
    let genres_selector = Selector::parse("p.type:nth-child(6) > a").unwrap();
    let released_selector = Selector::parse("p.type:nth-child(7)").unwrap();
    let default_episode_selector = Selector::parse("#default_ep").unwrap();
    let episode_pages_selector = Selector::parse("#episode_page li a").unwrap();

    let title = doc
        .select(&title_selector)
        .next()
        .ok_or("Title is missing")?
        .text()
        .next()
        .unwrap_or_default()
        .to_string();

    let id = doc
        .select(&id_selector)
        .next()
        .ok_or("Id is missing")?
        .value()
        .attr("value")
        .ok_or("Id input is missing the value attribute")?
        .parse::<usize>()
        .map_err(|_| "Invalid id")?;

    let cover_image_url = doc
        .select(&cover_image_selector)
        .next()
        .ok_or("Cover image is missing")?
        .value()
        .attr("src")
        .ok_or("Cover image is missing the src attribute")?
        .to_string();

    let summary = doc
        .select(&summary_selector)
        .next()
        .ok_or("Summary is missing")?
        .text()
        .skip(1)
        .collect::<String>();

    let genres = doc
        .select(&genres_selector)
        .map(|e| e.value().attr("title").unwrap().to_string())
        .collect::<Vec<String>>();

    let release_year = doc
        .select(&released_selector)
        .next()
        .ok_or("Release year is missing")?
        .text()
        .skip(1)
        .collect::<String>()
        .parse::<usize>()
        .map_err(|_| "Not a valid release year")?;

    let default_episode = doc
        .select(&default_episode_selector)
        .next()
        .ok_or("Default episode is missing")?
        .value()
        .attr("value")
        .ok_or("Default episode input is missing the value attribute")?
        .parse::<usize>()
        .map_err(|_| "Invalid default episode")?;

    let episode_pages: Vec<(usize, usize)> = doc
        .select(&episode_pages_selector)
        .map(|e| {
            let e = e.value();
            let start = e
                .attr("ep_start")
                .ok_or("Page is missing ep_start attr")?
                .parse::<usize>()
                .map_err(|_| "Invalid ep_start")?;
            let end = e
                .attr("ep_end")
                .ok_or("Page is missing ep_end attr")?
                .parse::<usize>()
                .map_err(|_| "Invalid ep_end")?;

            Ok((start, end))
        })
        .collect::<Result<_, String>>()?;

    let episode_count = episode_pages.last().ok_or("Episode pages are missing")?.1;

    Ok(AnimeDetails {
        id,
        title,
        cover_image_url,
        summary,
        genres,
        release_year,
        default_episode,
        episode_count,
    })
}

pub fn parse_episode(html: &str) -> ParseResult<Episode> {
    let doc = Html::parse_document(html);
    let anime_links_selector = Selector::parse(".anime_muti_link > ul > li").unwrap();
    let anime_link_selector = Selector::parse("a").unwrap();
    let mut providers = Vec::new();

    for link in doc.select(&anime_links_selector) {
        let video_url = link
            .select(&anime_link_selector)
            .next()
            .ok_or("Episode provider link is missing the <a> tag")?
            .value()
            .attr("data-video")
            .ok_or("Episode provider is missing the data-video attr")?
            .to_string();

        let link = link.value();
        let provider_name = link.classes().next().unwrap().to_string();

        providers.push((provider_name, video_url));
    }

    Ok(Episode { providers })
}

pub fn parse_search_result(json: &str) -> ParseResult<Vec<SearchResult>> {
    // the html has to be escaped inside the json that is why we have to unescape it
    let html = &json.trim()[12..json.len() - 2]
        .replace("\\/", "/")
        .replace("\\\"", "\"");
    let frag = Html::parse_fragment(html);
    let list_items_selector = Selector::parse("a").unwrap();

    frag.select(&list_items_selector)
        .map(|item| {
            let cover_image_url = ElementRef::wrap(item.first_child().unwrap())
                .unwrap()
                .value()
                .attr("style")
                .unwrap();
            let cover_image_url = cover_image_url[17..cover_image_url.len() - 2].to_string();

            let title = item.text().next().unwrap().to_string();
            let slug = item
                .value()
                .attr("href")
                .ok_or(format!("SearchResult `{}` is missing href", &title))?
                .split("/")
                .last()
                .unwrap()
                .to_string();

            Ok(SearchResult {
                slug,
                title,
                cover_image_url,
            })
        })
        .collect()
}
