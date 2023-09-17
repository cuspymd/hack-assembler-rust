pub mod instruction;
pub use instruction::{Instruction, InstructionType};


pub struct Parser {
    lines: Vec<String>,
    current_line_number: i32,
    current_instruction: Instruction,
}

impl Parser {
    pub fn new(file_text: String) -> Parser {
        Parser {
            lines: Parser::get_valid_lines(&file_text),
            current_line_number: -1,
            current_instruction: Instruction::new(""),
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
        self.current_line_number < self.lines.len() as i32 -1
    }

    pub fn advance(&mut self) {
        self.current_line_number += 1;
        self.current_instruction = Instruction::new(&self.lines[self.current_line_number as usize]);
    }

    pub fn instruction_type(&self) -> InstructionType {
        self.current_instruction.instruction_type()
    }

    pub fn symbol(&self) -> &str {
        self.current_instruction.symbol()
    }

    pub fn dest(&self) -> &str {
        self.current_instruction.dest()
    }

    pub fn comp(&self) -> &str {
        self.current_instruction.comp()
    }

    pub fn jump(&self) -> &str {
        self.current_instruction.jump()
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

    #[test]
    fn test_advance() {
        let mut parser = Parser::new(String::from("@R0"));
        assert!(parser.has_more_lines());
        parser.advance();
        assert!(!parser.has_more_lines());
    }

    #[test]
    fn test_advance_given_two_lines() {
        let mut parser = Parser::new(String::from("@R0\nD=M"));
        assert!(parser.has_more_lines());
        parser.advance();
        assert!(parser.has_more_lines());
        parser.advance();
        assert!(!parser.has_more_lines());
    }

    #[test]
    fn test_instruction_type() {
        let mut parser = Parser::new(String::from("@01"));
        parser.advance();
        assert!(matches!(parser.instruction_type(), InstructionType::A))
    }

    #[test]
    fn test_instruction_type_given_c_instruction() {
        let mut parser = Parser::new(String::from("M=1"));
        parser.advance();
        assert!(matches!(parser.instruction_type(), InstructionType::C))
    }

    #[test]
    fn test_instruction_type_given_l_instruction() {
        let mut parser = Parser::new(String::from("(LOOP)"));
        parser.advance();
        assert!(matches!(parser.instruction_type(), InstructionType::L))
    }

    #[test]
    fn test_symbol_given_a_instruction() {
        let mut parser = Parser::new(String::from("@01"));
        parser.advance();
        assert_eq!(parser.symbol(), "01");
    }

    #[test]
    fn test_symbol_given_l_instruction() {
        let mut parser = Parser::new(String::from("(LOOP)"));
        parser.advance();
        assert_eq!(parser.symbol(), "LOOP");
    }

    #[test]
    fn test_c_instruction_given_dest() {
        let mut parser = Parser::new(String::from("M=1"));
        parser.advance();
        assert_eq!(parser.dest(), "M");
        assert_eq!(parser.comp(), "1");
        assert_eq!(parser.jump(), "");
    }

    #[test]
    fn test_c_instruction_given_jump() {
        let mut parser = Parser::new(String::from("0;JMP"));
        parser.advance();
        assert_eq!(parser.dest(), "");
        assert_eq!(parser.comp(), "0");
        assert_eq!(parser.jump(), "JMP");
    }

    #[test]
    fn test_c_instruction_given_comp() {
        let mut parser = Parser::new(String::from("D+1"));
        parser.advance();
        assert_eq!(parser.dest(), "");
        assert_eq!(parser.comp(), "D+1");
        assert_eq!(parser.jump(), "");
    }

    #[test]
    fn test_c_instruction_given_all() {
        let mut parser = Parser::new(String::from("ADM=D+1;JGT"));
        parser.advance();
        assert_eq!(parser.dest(), "ADM");
        assert_eq!(parser.comp(), "D+1");
        assert_eq!(parser.jump(), "JGT");
    }

}
