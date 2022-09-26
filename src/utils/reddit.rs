const LINK_REDDIT: &str ="https://www.REDDIT.com";

pub fn construct_reddit_url(query: &str) -> String {
    if query == "red" {
        LINK_REDDIT.to_string()
    }
    else if &query[..5] == "red @" {
        // it a username or a repot
        let encoded_query = super::utf8_percent_encode(&query[5..], super::FRAGMENT)
        .to_string();
        let reddit_url = format!("{}/r/{}",LINK_REDDIT, encoded_query);
            
        reddit_url
    } else {
        // it's a search
        construct_reddit_search_url(&query[4..])
    }
}

pub fn construct_reddit_search_url(query: &str ) -> String{
    let encoded_query = super::utf8_percent_encode(&query, super::FRAGMENT)
        .to_string();
    let reddit_url = format!("{}/search/?q={}",LINK_REDDIT.to_string(), encoded_query);

    reddit_url
}



#[cfg(test)]
    mod tests {
    use super::*;

    #[test]
    fn test_construct_reddit_profile_url_with_red() {
        let fake_query = "red";
        assert_eq!(construct_reddit_url(fake_query), LINK_REDDIT.to_string());
    }

    #[test]
    fn test_construct_reddit_profile_url_with_reds_url() {
        let fake_query = "red @facebook";
        assert_eq!(
            construct_reddit_url(fake_query),
            LINK_REDDIT.to_string()+"/r/facebook"
        );
    }

    #[test]
    fn test_construct_reddit_search_url_with_query() {
        let fake_query = "red rust";
        assert_eq!(
            construct_reddit_url(fake_query),
            LINK_REDDIT.to_string()+"/search/?q=rust"
        );
    }
    #[test]
    fn test_construct_reddit_search_url_with_query_space() {
        let fake_query = "red rust lang";
        assert_eq!(
            construct_reddit_url(fake_query),
            LINK_REDDIT.to_string()+"/search/?q=rust%20lang"
        );
    }
}