struct Parser {
    lines: Vec<String>
}

impl Parser {
    fn new(file_text: String) -> Parser {
        Parser {
            lines: Parser::get_valid_lines(&file_text)
        }
    }

    fn get_valid_lines(file_text: &String) -> Vec<String> {
        file_text
            .lines()
            .map(|line| Parser::get_valid_text(line))
            .filter(|line| !line.is_empty())
            .collect()
    }

    fn get_valid_text(text: &str) -> String {
        let valid_text: &str = match text.split("//").next() {
            Some(first_part) => first_part,
            None => text
        };

        valid_text.trim().to_string()
    }

    pub fn has_more_lines(&self) -> bool {
        self.lines.len() > 0
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_more_lines_given_one_lines() {
        let parser = Parser::new(String::from("@01"));
        assert!(parser.has_more_lines());
    }

    #[test]
    fn test_has_more_lines_given_empty_lines() {
        let parser = Parser::new(String::from(""));
        assert!(!parser.has_more_lines());

        let parser = Parser::new(String::from("\n   \n     \n"));
        assert!(!parser.has_more_lines());

        let parser = Parser::new(String::from("\n   \n     @R1\n"));
        assert!(parser.has_more_lines());
    }

    #[test]
    fn test_has_more_lines_given_comment_line() {
        let parser = Parser::new(String::from("// comment"));
        assert!(!parser.has_more_lines());
    }
}
