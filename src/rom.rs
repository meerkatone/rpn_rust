use std::collections::HashMap;
use std::fs;
use std::io::{self, BufRead, BufReader};

#[derive(Debug, Clone)]
pub struct Rom {
    data: HashMap<u16, u16>,
}

impl Rom {
    pub fn new() -> Self {
        Rom {
            data: HashMap::new(),
        }
    }

    pub fn load_from_file(&mut self, filename: &str) -> io::Result<()> {
        let file = fs::File::open(filename)?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line?;
            let line = line.trim();
            
            // Skip comments and empty lines
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            // Parse address:value format
            if let Some((addr_str, val_str)) = line.split_once(':') {
                if let (Ok(addr), Ok(val)) = (
                    u16::from_str_radix(addr_str, 16),
                    u16::from_str_radix(val_str, 16)
                ) {
                    self.data.insert(addr, val);
                }
            }
        }

        Ok(())
    }

    pub fn read(&self, address: u16) -> u16 {
        self.data.get(&address).copied().unwrap_or(0)
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }
}