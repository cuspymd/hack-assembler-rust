pub enum InstructionType {
    A,
    C,
    L,
}

pub struct Instruction {
    text: String,
}

impl Instruction {
    pub fn new(text: &str) -> Instruction {
        Instruction {
            text: String::from(text),
        }
    }

    pub fn instruction_type(&self) -> InstructionType {
        if let Some(first_char) = self.text.chars().nth(0) {
            match first_char {
                '@' => InstructionType::A,
                '(' => InstructionType::L,
                _ => InstructionType::C
            }
        } else {
            InstructionType::A
        }
    }

    pub fn symbol(&self) -> &str {
        match self.instruction_type() {
            InstructionType::A => &self.text[1..],
            InstructionType::L => &self.text[1..self.text.len()-1],
            _ => "",
        }
    }

    pub fn dest(&self) -> &str {
        let mut parts = self.text.split("=");
        
        if let Some(first_part) = parts.next() {
            if let Some(_) = parts.next() {
                return first_part;
            }
        }
        
        ""
    }

    pub fn comp(&self) -> &str {
        let valid_text = match self.text.split("=").nth(1) {
            Some(second_part) => second_part,
            _ => &self.text,
        };

        match valid_text.split(";").nth(0) {
            Some(first_part) => first_part,
            _ => valid_text,
        }
    }

    pub fn jump(&self) -> &str {
        match self.text.split(";").nth(1) {
            Some(second_part) => second_part,
            _ => ""
        }
    }
}
