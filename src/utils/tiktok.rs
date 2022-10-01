extern crate percent_encoding;

const LINK_TIKTOK: &str ="https://www.tiktok.com";

pub fn construct_tiktok_url(query: &str) -> String {
    if query == "tik" {
        LINK_TIKTOK.to_string()
    } else if &query[..5] == "tik @" {
        construct_tiktok_profile_url(&query[4..])
    } else {
        // and search on tiktok
        construct_tiktok_search_url(&query[4..])
    }
}

pub fn construct_tiktok_profile_url(profile: &str) -> String {
    format!("{}/{}",LINK_TIKTOK.to_string(), profile)
}

pub fn construct_tiktok_search_url(query: &str) -> String {
    let encoded_query = super::utf8_percent_encode(query, super::FRAGMENT).to_string();
    let tiktok_search_url = format!("{}/search/user?q={}", LINK_TIKTOK.to_string() ,encoded_query);
    tiktok_search_url
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_tiktok_url() {
        let fake_query = "tik";
        assert_eq!(construct_tiktok_url(fake_query), 
            LINK_TIKTOK.to_string());
    }

    #[test]
    fn test_construct_tiktok_url_query() {
    let fake_query = "tik hello world";
        assert_eq!(construct_tiktok_url(fake_query),     
            LINK_TIKTOK.to_string()+"/search/user?q=hello%20world");
    }

    #[test]
    fn test_construct_tiktok_url_profile() {
        let fake_query = "tik @fbOpenSource";
        assert_eq!(construct_tiktok_url(fake_query), 
            LINK_TIKTOK.to_string()+"/@fbOpenSource");
    }

    #[test]
    fn test_construct_tiktok_profile_url() {
        let fake_profile = "@jsjoeio";
        assert_eq!(
            construct_tiktok_profile_url(fake_profile),
            LINK_TIKTOK.to_string()+"/@jsjoeio"
        );
    }

    #[test]
    fn test_construct_tiktok_search_url() {
        let fake_query = "hello world";
        assert_eq!(
            construct_tiktok_search_url(fake_query),
            LINK_TIKTOK.to_string()+"/search/user?q=hello%20world"
        );
    }
}