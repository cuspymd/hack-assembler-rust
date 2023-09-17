use std::{io::{self, Read}, path::Path, fs::File};

use crate::{parser::{Parser, instruction::InstructionType}, code::Code};

struct Assembler {
    
}

impl Assembler {
    pub fn new() -> Assembler {
        Assembler {  }
    }

    pub fn run(&self, input_path_str: &str) -> io::Result<()> {
        let input_path = Path::new(input_path_str);
        let folder_path = input_path.parent().unwrap();
        let file_base_name = input_path.file_stem().unwrap()
            .to_string_lossy().to_string();
        let output_path = &folder_path.join(format!("{}.hack", &file_base_name));

        let mut file = File::open(input_path)?;
        let mut input_text = String::new();
        file.read_to_string(&mut input_text)?;

        let mut file = File::create(output_path)?;

        // Phase one
        
        // Phase two
        let mut parser = Parser::new(input_text);
        let code = Code::new();

        while parser.has_more_lines() {
            parser.advance();

            match parser.instruction_type() {
                InstructionType::A => {
                    let symbol = parser.symbol();
                    
                },
                InstructionType::C => {
                    
                },
                InstructionType::L => {
                    
                }
            }
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::{path::Path, fs::{self, File}, io::{self, Read}};

    use super::Assembler;
    
    #[test]
    fn test_assembler_given_no_symbol() -> io::Result<()> {
        test_assembler("./test_data/RectL.asm")?;
        test_assembler("./test_data/Add.asm")?;
        test_assembler("./test_data/MaxL.asm")?;
        test_assembler("./test_data/PongL.asm")?;

        Ok(())
    }

    fn test_assembler(input_path: &str) -> io::Result<()> {
        let path = Path::new(input_path);
        let folder_path = path.parent().unwrap();
        let file_base_name = path.file_stem().unwrap().to_string_lossy().to_string();

        let out_file = &folder_path.join(format!("{}.hack", &file_base_name));
        let solution_file = &folder_path.join(format!("Solution_{}.hack", &file_base_name));

        let _ = fs::remove_file(out_file);

        Assembler::new().run(input_path);
        assert!(fs::metadata(out_file).is_ok());

        let mut file = File::open(solution_file)?;
        let mut solution_text = String::new();
        file.read_to_string(&mut solution_text)?;

        let mut file = File::open(out_file)?;
        let mut output_text = String::new();
        file.read_to_string(&mut output_text)?;

        assert_eq!(&output_text, &solution_text);
        fs::remove_file(out_file)?;
        
        Ok(())
    }
}
