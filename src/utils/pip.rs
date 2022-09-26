const LINK_PIP: &str ="https://www.pypi.org";

pub fn construct_pip_url(query: &str) -> String {
    if query == "pip" {
        LINK_PIP.to_string()
    }
    else if &query[..5] == "pip @" {
        // it a username or a repot
        let encoded_query = super::utf8_percent_encode(&query[5..], super::FRAGMENT)
        .to_string();
        let pip_url = format!("{}/user/{}",LINK_PIP, encoded_query);
            
        pip_url
    }
    else if &query[..7] == "pip pkg" {
        // it a username or a repot
        let encoded_query = super::utf8_percent_encode(&query[8..], super::FRAGMENT)
        .to_string();
        let pip_url = format!("{}/project/{}",LINK_PIP, encoded_query);
            
        pip_url
    } else {
        // it's a search
        construct_pip_search_url(&query[4..])
    }
}

pub fn construct_pip_search_url(query: &str ) -> String{
    let encoded_query = super::utf8_percent_encode(&query, super::FRAGMENT)
        .to_string();
    let pip_url = format!("{}/search/?q={}",LINK_PIP.to_string(), encoded_query);

    pip_url
}



#[cfg(test)]
    mod tests {
    use super::*;

    #[test]
    fn test_construct_pip_profile_url_with_pip() {
        let fake_query = "pip";
        assert_eq!(construct_pip_url(fake_query), LINK_PIP.to_string());
    }

    #[test]
    fn test_construct_pip_profile_url_with_pip_url() {
        let fake_query = "pip @facebook";
        assert_eq!(
            construct_pip_url(fake_query),
            LINK_PIP.to_string()+"/user/facebook"
        );
    }
    #[test]
    fn test_construct_pip_profile_url_with_pkg_url() {
        let fake_query = "pip pkg discord.js";
        assert_eq!(
            construct_pip_url(fake_query),
            LINK_PIP.to_string()+"/project/discord.js"
        );
    }

    #[test]
    fn test_construct_pip_search_url_with_query() {
        let fake_query = "pip rust";
        assert_eq!(
            construct_pip_url(fake_query),
            LINK_PIP.to_string()+"/search/?q=rust"
        );
    }
    #[test]
    fn test_construct_pip_search_url_with_query_space() {
        let fake_query = "pip rust lang";
        assert_eq!(
            construct_pip_url(fake_query),
            LINK_PIP.to_string()+"/search/?q=rust%20lang"
        );
    }
}