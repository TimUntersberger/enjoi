use insta;
use parsing::gogoanime::*;

#[test]
pub fn test_parse_anime_list() {
    insta::assert_debug_snapshot!(parse_anime_list(include_str!(
        "assets/gogoanime_anime_list.html"
    )));
}

#[test]
pub fn test_parse_anime_details() {
    insta::assert_debug_snapshot!(parse_anime_details(include_str!(
        "assets/gogoanime_anime_details.html"
    )));
}

#[test]
pub fn test_parse_episode() {
    insta::assert_debug_snapshot!(parse_episode(include_str!("assets/gogoanime_episode.html")));
}

#[test]
pub fn test_search_result() {
    insta::assert_debug_snapshot!(parse_search_result(include_str!(
        "assets/gogoanime_search_result.json"
    )));
}
