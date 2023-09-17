use std::collections::HashMap;

const COMP_DATA: [(&str, &str); 28] = [
    ("0", "0101010"),
    ("1", "0111111"),
    ("-1", "0111010"),
    ("D", "0001100"),
    ("A", "0110000"),
    ("M", "1110000"),
    ("!D", "0001101"),
    ("!A", "0110001"),
    ("!M", "1110001"),
    ("-D", "0001111"),
    ("-A", "0110011"),
    ("-M", "1110011"),
    ("D+1", "0011111"),
    ("A+1", "0110111"),
    ("M+1", "1110111"),
    ("D-1", "0001110"),
    ("A-1", "0110010"),
    ("M-1", "1110010"),
    ("D+A", "0000010"),
    ("D+M", "1000010"),
    ("D-A", "0010011"),
    ("D-M", "1010011"),
    ("A-D", "0000111"),
    ("M-D", "1000111"),
    ("D&A", "0000000"),
    ("D&M", "1000000"),
    ("D|A", "0010101"),
    ("D|M", "1010101")
];
const DEST_DATA: [(&str, &str); 8] = [
    ("", "000"),
    ("M", "001"),
    ("D", "010"),
    ("MD", "011"),
    ("A", "100"),
    ("AM", "101"),
    ("AD", "110"),
    ("AMD", "111")
];
const JUMP_DATA: [(&str, &str); 8] = [
    ("", "000"),
    ("JGT", "001"),
    ("JEQ", "010"),
    ("JGE", "011"),
    ("JLT", "100"),
    ("JNE", "101"),
    ("JLE", "110"),
    ("JMP", "111")
];


pub struct Code {
    comp_table: HashMap<String, String>,
    dest_table: HashMap<String, String>,
    jump_table: HashMap<String, String>,
}

impl Code {
    pub fn new() -> Code {
        Code {
            comp_table: Self::create_table(&COMP_DATA),
            dest_table: Self::create_table(&DEST_DATA),
            jump_table: Self::create_table(&JUMP_DATA),
        }
    }

    fn create_table(data: &[(&str, &str)]) -> HashMap<String, String> {
        let mut table = HashMap::new();
        for (word, digit_expression) in data.iter() {
            table.insert(word.to_string(), digit_expression.to_string());
        }
        table
    }
    
    pub fn comp(&self, instruction: &str) -> &str {
        self.comp_table.get(instruction).unwrap()
    }

    pub fn dest(&self, instruction: &str) -> &str {
        self.dest_table.get(instruction).unwrap()
    }

    pub fn jump(&self, instruction: &str) -> &str {
        self.jump_table.get(instruction).unwrap()
    }
}


#[cfg(test)]
mod tests {
    use super::{Code, COMP_DATA, DEST_DATA, JUMP_DATA};

    #[test]
    fn test_comp() {
        let code = Code::new();
        
        for (word, digit_expression) in COMP_DATA.iter() {
            assert_eq!(
                code.comp(word), *digit_expression, "wrong digit for '{}'", word);
        }
    }

    #[test]
    fn test_dest() {
        let code = Code::new();
        
        for (word, digit_expression) in DEST_DATA.iter() {
            assert_eq!(
                code.dest(word), *digit_expression, "wrong digit for '{}'", word);
        }
    }

    #[test]
    fn test_jump() {
        let code = Code::new();
        
        for (word, digit_expression) in JUMP_DATA.iter() {
            assert_eq!(
                code.jump(word), *digit_expression, "wrong digit for '{}'", word);
        }
    }
}
