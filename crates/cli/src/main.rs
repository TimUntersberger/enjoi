use clap::App;
use colored::*;
use scraping::gogoanime::*;

fn slugify(input: &str) -> String {
    let mut output = String::new();

    for c in input.chars() {
        let res = match c {
            ' ' => Some('-'),
            'a'..='z' | 'A'..='Z' | '0'..='9' => Some(c.to_lowercase().next().unwrap()),
            _ => None,
        };

        if let Some(res) = res {
            output.push(res);
        }
    }

    output
}

fn main() {
    let gogoanime = App::new("gogoanime")
        .about("the gogoanime submodule")
        .subcommand(App::new("search").arg("<text>... 'The text to search for"))
        .subcommand(App::new("details").arg("<title>... 'The title of the anime"));

    let args = App::new("enjoi")
        .version("0.1.0")
        .about("CLI version of enjoi")
        .subcommand(gogoanime)
        .get_matches();

    match args.subcommand() {
        Some(("gogoanime", m)) => match m.subcommand() {
            Some(("search", m)) => {
                let text = m.values_of("text").unwrap().collect::<Vec<_>>().join(" ");
                match scrape_search(&text) {
                    Ok(results) => {
                        for res in results {
                            println!("{} {}", res.title, format!("https://{}/category/{}", DOMAIN, res.slug).green())
                        }
                    }
                    Err(msg) => {
                        println!("{}", format!("[ERROR]: {}", msg).red())
                    }
                }
            }
            Some(("details", m)) => {
                let title = m.values_of("title").unwrap().collect::<Vec<_>>().join(" ");
                let title = slugify(&title);

                match scrape_anime_details(&title) {
                    Ok(details) => {
                        println!("{}: {}", "Id".blue(), details.id.to_string().cyan());
                        println!("{}: {}", "Cover Image".blue(), details.cover_image_url.green());
                        println!("{}: {}", "Released in".blue(), details.release_year.to_string().cyan());
                        println!("{}: {}", "Episodes".blue(), details.episode_count.to_string().cyan());
                        println!(
                            "{}: {}",
                            "Genres".blue(),
                            details
                                .genres
                                .iter()
                                .map(|x| x.yellow().to_string())
                                .collect::<Vec<String>>()
                                .join(", ")
                        );
                        println!("{}: {}", "Summary".blue(), details.summary);
                    }
                    Err(msg) => {
                        println!("{}", format!("[ERROR]: {}", msg).red())
                    }
                }
            }
            _ => {}
        },
        _ => {}
    }
}
