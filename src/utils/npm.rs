const LINK_NPM: &str ="https://www.npmjs.com";

pub fn construct_npm_url(query: &str) -> String {
    if query == "npm" {
        LINK_NPM.to_string()
    }
    else if &query[..5] == "npm @" {
        // it a username or a repot
        let encoded_query = super::utf8_percent_encode(&query[5..], super::FRAGMENT)
        .to_string();
        let npm_url = format!("{}/~{}",LINK_NPM, encoded_query);
            
        npm_url
    }
    else if &query[..7] == "npm pkg" {
        // it a username or a repot
        let encoded_query = super::utf8_percent_encode(&query[8..], super::FRAGMENT)
        .to_string();
        let npm_url = format!("{}/package/{}",LINK_NPM, encoded_query);
            
        npm_url
    } else {
        // it's a search
        construct_npm_search_url(&query[4..])
    }
}

pub fn construct_npm_search_url(query: &str ) -> String{
    let encoded_query = super::utf8_percent_encode(&query, super::FRAGMENT)
        .to_string();
    let npm_url = format!("{}/search?q={}",LINK_NPM.to_string(), encoded_query);

    npm_url
}



#[cfg(test)]
    mod tests {
    use super::*;

    #[test]
    fn test_construct_npm_profile_url_with_npm() {
        let fake_query = "npm";
        assert_eq!(construct_npm_url(fake_query), LINK_NPM.to_string());
    }

    #[test]
    fn test_construct_npm_profile_url_with_npm_url() {
        let fake_query = "npm @facebook";
        assert_eq!(
            construct_npm_url(fake_query),
            LINK_NPM.to_string()+"/~facebook"
        );
    }
    #[test]
    fn test_construct_npm_profile_url_with_pkg_url() {
        let fake_query = "npm pkg discord.js";
        assert_eq!(
            construct_npm_url(fake_query),
            LINK_NPM.to_string()+"/package/discord.js"
        );
    }

    #[test]
    fn test_construct_npm_search_url_with_query() {
        let fake_query = "npm rust";
        assert_eq!(
            construct_npm_url(fake_query),
            LINK_NPM.to_string()+"/search?q=rust"
        );
    }
    #[test]
    fn test_construct_npm_search_url_with_query_space() {
        let fake_query = "npm rust lang";
        assert_eq!(
            construct_npm_url(fake_query),
            LINK_NPM.to_string()+"/search?q=rust%20lang"
        );
    }
}