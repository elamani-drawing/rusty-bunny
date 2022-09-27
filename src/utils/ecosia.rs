const LINK_ECOSIA: &str ="https://www.ecosia.org";

pub fn construct_ecosia_url(query: &str) -> String {
    if query == "eco" {
        LINK_ECOSIA.to_string()
    } else {
        // it's a search
        construct_ecosia_search_url(&query[4..])
    }
}

pub fn construct_ecosia_search_url(query: &str ) -> String{
    let encoded_query = super::utf8_percent_encode(&query, super::FRAGMENT)
        .to_string();
    let ecosia_url = format!("{}/search?method=index&q={}",LINK_ECOSIA.to_string(), encoded_query);

    ecosia_url
}



#[cfg(test)]
    mod tests {
    use super::*;

    #[test]
    fn test_construct_ecosia_profile_url_with_ecosia() {
        let fake_query = "eco";
        assert_eq!(construct_ecosia_url(fake_query), LINK_ECOSIA.to_string());
    }

    #[test]
    fn test_construct_ecosia_search_url_with_query() {
        let fake_query = "eco rust";
        assert_eq!(
            construct_ecosia_url(fake_query),
            LINK_ECOSIA.to_string()+"/search?method=index&q=rust"
        );
    }
    #[test]
    fn test_construct_ecosia_search_url_with_query_space() {
        let fake_query = "eco rust lang";
        assert_eq!(
            construct_ecosia_url(fake_query),
            LINK_ECOSIA.to_string()+"/search?method=index&q=rust%20lang"
        );
    }
}