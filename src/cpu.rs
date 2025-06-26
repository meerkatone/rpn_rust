use crate::rom::Rom;

#[derive(Debug, Clone)]
pub struct Hp16cCpu {
    // RPN Stack (X, Y, Z, T registers)
    pub x: u128,  // Display register
    pub y: u128,  // First operand
    pub z: u128,  // Second operand  
    pub t: u128,  // Third operand

    // Program counter and ROM
    pub pc: u16,
    pub rom: Rom,
    
    // Word size (1-128 bits)
    pub word_size: u8,
    
    // Number base (2, 8, 10, 16)
    pub base: u8,
    
    // Flags
    pub carry: bool,
    pub overflow: bool,
    
    // Memory
    pub memory: [u128; 16],  // HP-16C has 16 memory registers
    
    pub running: bool,
}

impl Hp16cCpu {
    pub fn new() -> Self {
        Hp16cCpu {
            x: 0,
            y: 0,
            z: 0,
            t: 0,
            pc: 0,
            rom: Rom::new(),
            word_size: 16,
            base: 16,
            carry: false,
            overflow: false,
            memory: [0; 16],
            running: true,
        }
    }

    pub fn load_rom(&mut self, filename: &str) -> Result<(), std::io::Error> {
        self.rom.load_from_file(filename)
    }

    // RPN Stack operations
    pub fn push(&mut self, value: u128) {
        self.t = self.z;
        self.z = self.y;
        self.y = self.x;
        self.x = self.mask_value(value);
    }

    pub fn pop(&mut self) -> u128 {
        let result = self.x;
        self.x = self.y;
        self.y = self.z;
        self.z = self.t;
        // T register duplicates when stack drops
        result
    }

    pub fn drop(&mut self) {
        self.x = self.y;
        self.y = self.z;
        self.z = self.t;
    }

    pub fn swap_xy(&mut self) {
        let temp = self.x;
        self.x = self.y;
        self.y = temp;
    }

    pub fn roll_down(&mut self) {
        let temp = self.x;
        self.x = self.y;
        self.y = self.z;
        self.z = self.t;
        self.t = temp;
    }

    pub fn roll_up(&mut self) {
        let temp = self.t;
        self.t = self.z;
        self.z = self.y;
        self.y = self.x;
        self.x = temp;
    }

    // Apply word size mask
    fn mask_value(&self, value: u128) -> u128 {
        if self.word_size == 128 {
            value
        } else if self.word_size == 64 {
            value & u64::MAX as u128
        } else {
            value & ((1u128 << self.word_size) - 1)
        }
    }

    // Arithmetic operations
    pub fn add(&mut self) {
        let result = self.x.wrapping_add(self.y);
        self.carry = result < self.x || result < self.y;
        self.drop();
        self.x = self.mask_value(result);
    }

    pub fn subtract(&mut self) {
        let result = self.y.wrapping_sub(self.x);
        self.carry = self.y < self.x;
        self.drop();
        self.x = self.mask_value(result);
    }

    pub fn multiply(&mut self) {
        let (result, overflow) = self.x.overflowing_mul(self.y);
        self.carry = overflow;
        self.drop();
        self.x = self.mask_value(result);
    }

    pub fn divide(&mut self) {
        if self.x != 0 {
            let result = self.y / self.x;
            self.drop();
            self.x = self.mask_value(result);
            self.carry = false;
        } else {
            // Division by zero - set overflow
            self.overflow = true;
        }
    }

    // Bitwise operations
    pub fn and(&mut self) {
        let result = self.x & self.y;
        self.drop();
        self.x = result;
    }

    pub fn or(&mut self) {
        let result = self.x | self.y;
        self.drop();
        self.x = result;
    }

    pub fn xor(&mut self) {
        let result = self.x ^ self.y;
        self.drop();
        self.x = result;
    }

    pub fn not(&mut self) {
        self.x = self.mask_value(!self.x);
    }

    // Shift operations
    pub fn shift_left(&mut self, positions: u8) {
        let result = self.x << positions;
        self.carry = (self.x >> (self.word_size - positions)) != 0;
        self.x = self.mask_value(result);
    }

    pub fn shift_right(&mut self, positions: u8) {
        self.carry = (self.x & ((1 << positions) - 1)) != 0;
        self.x = self.x >> positions;
    }

    // Memory operations
    pub fn store(&mut self, register: usize) {
        if register < 16 {
            self.memory[register] = self.x;
        }
    }

    pub fn recall(&mut self, register: usize) {
        if register < 16 {
            self.push(self.memory[register]);
        }
    }

    // Number base conversion
    pub fn set_base(&mut self, base: u8) {
        if base == 2 || base == 8 || base == 10 || base == 16 {
            self.base = base;
        }
    }

    pub fn set_word_size(&mut self, size: u8) {
        if size >= 1 && size <= 128 {
            self.word_size = size;
            // Re-mask current values
            self.x = self.mask_value(self.x);
            self.y = self.mask_value(self.y);
            self.z = self.mask_value(self.z);
            self.t = self.mask_value(self.t);
        }
    }

    // Display formatting
    pub fn format_display(&self) -> String {
        match self.base {
            2 => format!("{:b}", self.x),
            8 => format!("{:o}", self.x),
            10 => format!("{}", self.x),
            16 => format!("{:X}", self.x),
            _ => format!("{:X}", self.x),
        }
    }

    pub fn get_stack_display(&self) -> [String; 4] {
        [
            format!("T: {}", match self.base {
                2 => format!("{:b}", self.t),
                8 => format!("{:o}", self.t),
                10 => format!("{}", self.t),
                16 => format!("{:X}", self.t),
                _ => format!("{:X}", self.t),
            }),
            format!("Z: {}", match self.base {
                2 => format!("{:b}", self.z),
                8 => format!("{:o}", self.z),
                10 => format!("{}", self.z),
                16 => format!("{:X}", self.z),
                _ => format!("{:X}", self.z),
            }),
            format!("Y: {}", match self.base {
                2 => format!("{:b}", self.y),
                8 => format!("{:o}", self.y),
                10 => format!("{}", self.y),
                16 => format!("{:X}", self.y),
                _ => format!("{:X}", self.y),
            }),
            format!("X: {}", match self.base {
                2 => format!("{:b}", self.x),
                8 => format!("{:o}", self.x),
                10 => format!("{}", self.x),
                16 => format!("{:X}", self.x),
                _ => format!("{:X}", self.x),
            }),
        ]
    }
}