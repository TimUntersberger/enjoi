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
