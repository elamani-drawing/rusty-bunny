const LINK_DDG: &str ="https://duckduckgo.com";

pub fn construct_ddg_url(query: &str) -> String {
    if query == "ddg" {
        LINK_DDG.to_string()
    } else {
        // it's a search
        construct_ddg_search_url(&query[4..])
    }
}

pub fn construct_ddg_search_url(query: &str ) -> String{
    let encoded_query = super::utf8_percent_encode(&query, super::FRAGMENT)
        .to_string();
    let ddg_url = format!("{}/?q={}",LINK_DDG.to_string(), encoded_query);

    ddg_url
}



#[cfg(test)]
    mod tests {
    use super::*;

    #[test]
    fn test_construct_ddg_profile_url_with_ddg() {
        let fake_query = "ddg";
        assert_eq!(construct_ddg_url(fake_query), LINK_DDG.to_string());
    }

    #[test]
    fn test_construct_ddg_search_url_with_query() {
        let fake_query = "ddg rust";
        assert_eq!(
            construct_ddg_url(fake_query),
            LINK_DDG.to_string()+"/?q=rust"
        );
    }
    #[test]
    fn test_construct_ddg_search_url_with_query_space() {
        let fake_query = "ddg rust lang";
        assert_eq!(
            construct_ddg_url(fake_query),
            LINK_DDG.to_string()+"/?q=rust%20lang"
        );
    }
}