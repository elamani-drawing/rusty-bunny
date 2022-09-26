const LINK_TWITCH: &str ="https://www.twitch.tv";

pub fn construct_twitch_url(query: &str) -> String {
    if query == "twi" {
        LINK_TWITCH.to_string()
    }
    else if &query[..5] == "twi @" {
        // it a username or a repot
        let encoded_query = super::utf8_percent_encode(&query[5..], super::FRAGMENT)
        .to_string();
        let twitch_url = format!("{}/{}",LINK_TWITCH, encoded_query);
            
        twitch_url
    } else {
        // it's a search
        construct_twitch_search_url(&query[4..])
    }
}

pub fn construct_twitch_search_url(query: &str ) -> String{
    let encoded_query = super::utf8_percent_encode(&query, super::FRAGMENT)
        .to_string();
    let twitch_url = format!("{}/search/?term={}",LINK_TWITCH.to_string(), encoded_query);

    twitch_url
}



#[cfg(test)]
    mod tests {
    use super::*;

    #[test]
    fn test_construct_twitch_profile_url_with_twi() {
        let fake_query = "twi";
        assert_eq!(construct_twitch_url(fake_query), LINK_TWITCH.to_string());
    }

    #[test]
    fn test_construct_twitch_profile_url_with_twisrepo_url() {
        let fake_query = "twi @facebook";
        assert_eq!(
            construct_twitch_url(fake_query),
            LINK_TWITCH.to_string()+"/facebook"
        );
    }

    #[test]
    fn test_construct_twitch_search_url_with_query() {
        let fake_query = "twi rust";
        assert_eq!(
            construct_twitch_url(fake_query),
            LINK_TWITCH.to_string()+"/search/?term=rust"
        );
    }
    #[test]
    fn test_construct_twitch_search_url_with_query_space() {
        let fake_query = "twi rust lang";
        assert_eq!(
            construct_twitch_url(fake_query),
            LINK_TWITCH.to_string()+"/search/?term=rust%20lang"
        );
    }
}