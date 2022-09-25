extern crate percent_encoding;

use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

// Used as part of the percent_encoding library
const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<')
    .add(b'>').add(b'`');

const LINK_YOUTUBE: &str ="https://www.youtube.com";

pub fn construct_youtube_url(query: &str) -> String {
    if query == "yt" {
        LINK_YOUTUBE.to_string()
    } else if &query[..4] == "yt @" {
        let encoded_query: &str = &utf8_percent_encode(&query[4..], FRAGMENT)
            .to_string()[..];
        //youtube username profile
        construct_youtube_profile_url(encoded_query)
    }else{
        let encoded_query: &str = &utf8_percent_encode(&query[3..], FRAGMENT)
            .to_string()[..];
        construct_youtube_search_url(encoded_query)
    }
}

pub fn construct_youtube_profile_url(profile: &str) -> String {
    format!("{}/{}", LINK_YOUTUBE, profile)
}

pub fn construct_youtube_search_url(query: &str) -> String {
    format!("{}/results?search_query={}",LINK_YOUTUBE, query)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_youtube_url() {
        let fake_query = "yt";
        assert_eq!(construct_youtube_url(fake_query), 
        LINK_YOUTUBE.to_string());
    }

    #[test]
    fn test_construct_youtube_url_profile() {
        let fake_username = "test";
        assert_eq!(construct_youtube_profile_url(fake_username), 
            LINK_YOUTUBE.to_string()+"/test");
    }

    #[test]
    fn test_construct_youtube_url_profile_space() {
        let fake_username = "test test";
        assert_eq!(construct_youtube_profile_url(fake_username), 
            LINK_YOUTUBE.to_string()+"/test test");
    }
    #[test]
    fn test_construct_youtube_query_url() {
        let fake_query = "test";
        assert_eq!(construct_youtube_search_url(fake_query), 
        LINK_YOUTUBE.to_string()+"/results?search_query=test");
    }
    #[test]
    fn test_construct_youtube_query_url_space() {
        let fake_query = "test test";
        assert_eq!(construct_youtube_search_url(fake_query), 
        LINK_YOUTUBE.to_string()+"/results?search_query=test test");
    }
}