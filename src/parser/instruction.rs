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
}
