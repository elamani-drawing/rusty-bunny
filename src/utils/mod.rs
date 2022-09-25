pub mod google;
pub mod twitter;
pub mod github;
pub mod instagram;

// utiliser pour analyser la chaîne de requête
pub fn get_command_from_query_string(query_string: &str) -> &str {
    //il vérifie la chaîne de requête pour un caractère d'espacement à l'aide de la méthode .contains . 
    if query_string.contains(' ') {
        // We need to this to know where to slice the string
        //S'il en contient un, il trouve l'index de l'espace blanc et renvoie la chaîne de requête du début jusqu'au caractère d'espace blanc en utilisant [...index_of_space] . 
        let index_of_space = query_string.find(' ').unwrap_or(0);
            return &query_string[..index_of_space];
    }
    // Otherwise, return the query string as is
    //S'il ne trouve pas d'espace, il suppose que la chaîne de requête est la commande et la renvoie telle quell
    &query_string
}

pub fn remove_space_at_query(query_string: &str)-> String {
    query_string.replace(' ', "")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_command_from_query_string_no_whitespace() {
        // Test with command only
        let actual = get_command_from_query_string("tw");
        let expected = "tw";
        assert_eq!(actual, expected);
    }
    #[test]
    fn test_get_command_from_query_string_with_whitespace() {
        let actual = get_command_from_query_string
        ("tw @fbOpenSource");
        let expected = "tw";
        assert_eq!(actual, expected);
    }
}


