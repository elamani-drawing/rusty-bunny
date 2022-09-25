const LINK_INSTAGRAM: &str ="https://www.instagram.com";

pub fn construct_instagram_url(query: &str) -> String {
    if query == "ig" {
        LINK_INSTAGRAM.to_string()
    } else if &query[..4] == "ig @" {
        //instagram username profile
        construct_instagram_profile_url(&query[4..])
    }else{
        construct_instagram_tag_url_space(&query[3..])
    }
}

pub fn construct_instagram_profile_url(profile: &str) -> String {
    format!("{}/{}", LINK_INSTAGRAM, profile)
}

pub fn construct_instagram_tag_url(tag: String) -> String {
    println!("---{}",tag);
    format!("{}/tags/{}",LINK_INSTAGRAM, tag)
}

pub fn construct_instagram_tag_url_space(tag: &str) -> String {
    construct_instagram_tag_url(super::remove_space_at_query(tag))
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_instagram_url() {
        let fake_query = "ig";
        assert_eq!(construct_instagram_url(fake_query), 
        LINK_INSTAGRAM.to_string());
    }

    #[test]
    fn test_construct_instagram_url_profile() {
        let fake_username = "test";
        assert_eq!(construct_instagram_profile_url(fake_username), 
            LINK_INSTAGRAM.to_string()+"/test");
    }
    #[test]
    fn test_construct_instagram_url_tag() {
        let fake_tag = "test";
        assert_eq!(construct_instagram_tag_url_space(fake_tag), 
        LINK_INSTAGRAM.to_string()+"/tags/test");
    }

    #[test]
    fn test_construct_instagram_url_tag_space() {
        let fake_tag = "test hello world";
        assert_eq!(
            construct_instagram_tag_url_space(fake_tag),
            LINK_INSTAGRAM.to_string()+"/tags/testhelloworld"
        );
    }
}