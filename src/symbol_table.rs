use std::collections::HashMap;


const PREDEFINED_SYMBOLS: [(&str, u32); 23] = [
    ("SP", 0x0),
    ("LCL", 0x1),
    ("ARG", 0x2),
    ("THIS", 0x3),
    ("THAT", 0x4),
    ("R0", 0x0),
    ("R1", 0x1),
    ("R2", 0x2),
    ("R3", 0x3),
    ("R4", 0x4),
    ("R5", 0x5),
    ("R6", 0x6),
    ("R7", 0x7),
    ("R8", 0x8),
    ("R9", 0x9),
    ("R10", 0xA),
    ("R11", 0xB),
    ("R12", 0xC),
    ("R13", 0xD),
    ("R14", 0xE),
    ("R15", 0xF),
    ("SCREEN", 0x4000),
    ("KBD", 0x6000)
];

pub struct SymbolTable {
    table: HashMap<String, u32>
}

impl SymbolTable {
    pub fn new() -> SymbolTable {
        let mut table = HashMap::new();
        for (symbol, address) in PREDEFINED_SYMBOLS {
            table.insert(symbol.to_string(), address);
        }
        
        SymbolTable {
            table
        }
    }

    pub fn contains(&self, symbol: &str) -> bool {
        self.table.contains_key(symbol)
    }

    pub fn add_entry(&mut self, symbol: &str, address: u32) {
        self.table.insert(symbol.to_string(), address);
    }

    pub fn get_address(&self, symbol: &str) -> u32 {
        self.table[symbol]
    }
}


#[cfg(test)]
mod tests {
    use super::{SymbolTable, PREDEFINED_SYMBOLS};

    #[test]
    fn test_predefined_symbols() {
        let st = SymbolTable::new();

        for (symbol, address) in PREDEFINED_SYMBOLS {
            assert!(st.contains(symbol));
            assert_eq!(st.get_address(symbol), address);
        }
        assert!(!st.contains("LOOP"))
    }

    #[test]
    fn test_basic_functions() {
        let mut st = SymbolTable::new();

        assert!(!st.contains("LOOP"));
        st.add_entry("LOOP", 0x11);
        assert!(st.contains("LOOP"));
        assert_eq!(st.get_address("LOOP"), 0x11);
    }
}
