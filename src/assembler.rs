struct Assembler {
    
}

impl Assembler {
    pub fn new() -> Assembler {
        Assembler {  }
    }

    pub fn run(&self, input_path: &str) {
        
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
