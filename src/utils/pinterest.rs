const LINK_PINTEREST: &str ="https://www.pinterest.com";

pub fn construct_pinterest_url(query: &str) -> String {
    if query == "pin" {
        LINK_PINTEREST.to_string()
    }
    else if &query[..5] == "pin @" {
        // it a username or a repot
        let encoded_query = super::utf8_percent_encode(&query[5..], super::FRAGMENT)
        .to_string();
        let pinterest_url = format!("{}/{}",LINK_PINTEREST, encoded_query);

        pinterest_url
    } else {
        // it's a search
        construct_pinterest_search_url(&query[4..])
    }
}

pub fn construct_pinterest_search_url(query: &str ) -> String{
    let encoded_query = super::utf8_percent_encode(&query, super::FRAGMENT)
        .to_string();
    let pinterest_url = format!("{}/search/pins/?q={}",LINK_PINTEREST.to_string(), encoded_query);

    pinterest_url
}



#[cfg(test)]
    mod tests {
    use super::*;

    #[test]
    fn test_construct_pinterest_profile_url_with_pin() {
        let fake_query = "pin";
        assert_eq!(construct_pinterest_url(fake_query), LINK_PINTEREST.to_string());
    }

    #[test]
    fn test_construct_pinterest_profile_url_with_pinsrepo_url() {
        let fake_query = "pin @facebook";
        assert_eq!(
            construct_pinterest_url(fake_query),
            LINK_PINTEREST.to_string()+"/facebook"
        );
    }

    #[test]
    fn test_construct_pinterest_search_url_with_pinsrepot_url() {
        let fake_query = "pin @facebook/docusaurus";
        assert_eq!(
            construct_pinterest_url(fake_query),
            LINK_PINTEREST.to_string()+"/facebook/docusaurus"
        );
    }
    #[test]
    fn test_construct_pinterest_search_url_with_query() {
        let fake_query = "pin rust";
        assert_eq!(
            construct_pinterest_url(fake_query),
            "https://www.pinterest.com/search/pins/?q=rust"
        );
    }
    #[test]
    fn test_construct_pinterest_search_url_with_query_space() {
        let fake_query = "pin rust lang";
        assert_eq!(
            construct_pinterest_url(fake_query),
            LINK_PINTEREST.to_string()+"/search/pins/?q=rust%20lang"
        );
    }
}