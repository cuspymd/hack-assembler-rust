use std::{io::{Read, Write}, path::Path, fs::File};
use crate::{parser::{Parser, instruction::InstructionType}, code::Code, symbol_table::SymbolTable};

struct Assembler {
    symbol_table: SymbolTable,
    empty_symbol_address: u32,
}

impl Assembler {
    pub fn new() -> Assembler {
        Assembler {
            symbol_table: SymbolTable::new(),
            empty_symbol_address: 16,
        }
    }

    pub fn run(&mut self, input_path_str: &str) {
        let input_path = Path::new(input_path_str);
        let folder_path = input_path.parent().unwrap();
        let file_base_name = input_path.file_stem().unwrap()
            .to_string_lossy().to_string();
        let output_path = &folder_path.join(format!("{}.hack", &file_base_name));

        let mut file = File::open(input_path).unwrap();
        let mut input_text = String::new();
        file.read_to_string(&mut input_text).unwrap();

        let mut file = File::create(output_path).unwrap();

        self.run_phase_one(&input_text);
        self.run_phase_two(&input_text, &mut file);
    }

    fn run_phase_one(&mut self, input_text: &str) {
        let mut line_num: u32 = 0;
        let mut parser = Parser::new(input_text);

        while parser.has_more_lines() {
            parser.advance();
            match parser.instruction_type() {
                InstructionType::L => {
                    let symbol = parser.symbol();
                    self.symbol_table.add_entry(symbol, line_num);
                },
                _ => {
                    line_num += 1;
                }
            }
        }
    }

    fn run_phase_two(&mut self, input_text: &str, file: &mut File) {
        let mut parser = Parser::new(&input_text);
        let code = Code::new();

        while parser.has_more_lines() {
            parser.advance();

            match parser.instruction_type() {
                InstructionType::A => {
                    let symbol = parser.symbol();
                    let binary = self.process_symbol(symbol);
                    let line = format!("{}\n", binary);

                    file.write(line.as_bytes()).unwrap();
                },
                InstructionType::C => {
                    let mut comp = parser.comp();
                    comp = code.comp(comp);
                    let mut dest = parser.dest();
                    dest = code.dest(dest);
                    let mut jump = parser.jump();
                    jump = code.jump(jump);

                    let line = format!("111{}{}{}\n", comp, dest, jump);
                    file.write(line.as_bytes()).unwrap();
                },
                InstructionType::L => {
                }
            }
        }
    }

    fn process_symbol(&mut self, symbol: &str) -> String {
        let mut valid_symbol = symbol;
        let address_str: String;

        if valid_symbol.parse::<u32>().is_err() {
            let address: u32;

            if self.symbol_table.contains(symbol) {
                address = self.symbol_table.get_address(symbol);
            } else {
                self.symbol_table.add_entry(symbol, self.empty_symbol_address);
                address = self.empty_symbol_address;
                self.empty_symbol_address += 1;
            }

            address_str = address.to_string();
            valid_symbol = &address_str;
        } 

        let number: u32 = valid_symbol.parse().unwrap();
        format!("{:016b}", number)
    }
}

#[cfg(test)]
mod tests {
    use std::{path::Path, fs::{self, File}, io::Read};

    use super::Assembler;
    
    #[test]
    fn test_assembler_given_no_symbol() {
        test_assembler("./test_data/RectL.asm");
        test_assembler("./test_data/Add.asm");
        test_assembler("./test_data/MaxL.asm");
        test_assembler("./test_data/PongL.asm");
    }

    #[test]
    fn test_assembler_given_symbol() {
        test_assembler("./test_data/Rect.asm");
        test_assembler("./test_data/Max.asm");
        test_assembler("./test_data/Pong.asm");
    }

    fn test_assembler(input_path: &str) {
        let path = Path::new(input_path);
        let folder_path = path.parent().unwrap();
        let file_base_name = path.file_stem().unwrap().to_string_lossy().to_string();

        let out_file = &folder_path.join(format!("{}.hack", &file_base_name));
        let solution_file = &folder_path.join(format!("Solution_{}.hack", &file_base_name));

        let _ = fs::remove_file(out_file);

        Assembler::new().run(input_path);
        assert!(fs::metadata(out_file).is_ok());

        let mut file = File::open(solution_file).unwrap();
        let mut solution_text = String::new();
        file.read_to_string(&mut solution_text).unwrap();

        let mut file = File::open(out_file).unwrap();
        let mut output_text = String::new();
        file.read_to_string(&mut output_text).unwrap();

        assert_eq!(&output_text, &solution_text);
        fs::remove_file(out_file).unwrap();
    }
}
