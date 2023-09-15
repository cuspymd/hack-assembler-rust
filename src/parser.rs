struct Parser {
    
}

impl Parser {
    fn new(file_text: String) -> Parser {
        Parser {}
    }

    fn has_more_lines(&self) -> bool {
        true
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_more_lines_given_one_lines() {
        let parser = Parser::new(String::from("@01"));
        assert!(parser.has_more_lines())
    }
}
