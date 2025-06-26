pub mod rom;
pub mod cpu;

#[cfg(test)]
mod tests {
    use super::*;
    use cpu::Hp16cCpu;

    #[test]
    fn test_rpn_stack_push_pop() {
        let mut calc = Hp16cCpu::new();
        
        calc.push(42);
        assert_eq!(calc.x, 42);
        assert_eq!(calc.y, 0);
        
        calc.push(100);
        assert_eq!(calc.x, 100);
        assert_eq!(calc.y, 42);
        
        let popped = calc.pop();
        assert_eq!(popped, 100);
        assert_eq!(calc.x, 42);
    }

    #[test]
    fn test_basic_arithmetic() {
        let mut calc = Hp16cCpu::new();
        
        // Test addition: 10 + 5 = 15
        calc.push(10);
        calc.push(5);
        calc.add();
        assert_eq!(calc.x, 15);
        
        // Test subtraction: 15 - 3 = 12
        calc.push(3);
        calc.subtract();
        assert_eq!(calc.x, 12);
        
        // Test multiplication: 12 * 2 = 24
        calc.push(2);
        calc.multiply();
        assert_eq!(calc.x, 24);
        
        // Test division: 24 / 4 = 6
        calc.push(4);
        calc.divide();
        assert_eq!(calc.x, 6);
    }

    #[test]
    fn test_bitwise_operations() {
        let mut calc = Hp16cCpu::new();
        
        // Test AND: 0xF0 & 0x0F = 0x00
        calc.push(0xF0);
        calc.push(0x0F);
        calc.and();
        assert_eq!(calc.x, 0x00);
        
        // Test OR: 0xF0 | 0x0F = 0xFF
        calc.push(0xF0);
        calc.push(0x0F);
        calc.or();
        assert_eq!(calc.x, 0xFF);
        
        // Test XOR: 0xFF ^ 0xAA = 0x55
        calc.push(0xFF);
        calc.push(0xAA);
        calc.xor();
        assert_eq!(calc.x, 0x55);
    }

    #[test]
    fn test_stack_operations() {
        let mut calc = Hp16cCpu::new();
        
        calc.push(1);
        calc.push(2);
        calc.push(3);
        calc.push(4);
        
        assert_eq!(calc.x, 4);
        assert_eq!(calc.y, 3);
        assert_eq!(calc.z, 2);
        assert_eq!(calc.t, 1);
        
        calc.swap_xy();
        assert_eq!(calc.x, 3);
        assert_eq!(calc.y, 4);
        
        calc.roll_down();
        assert_eq!(calc.x, 4);
        assert_eq!(calc.y, 2);
        assert_eq!(calc.z, 1);
        assert_eq!(calc.t, 3);
    }

    #[test]
    fn test_word_size_masking() {
        let mut calc = Hp16cCpu::new();
        
        calc.set_word_size(8);
        calc.push(0x1FF); // 511 in decimal, should be masked to 8 bits
        assert_eq!(calc.x, 0xFF); // Should be 255 (max for 8 bits)
        
        calc.set_word_size(4);
        calc.push(0x20); // 32 in decimal, should be masked to 4 bits
        assert_eq!(calc.x, 0x0); // Should be 0 (32 & 0xF = 0)
    }

    #[test]
    fn test_memory_operations() {
        let mut calc = Hp16cCpu::new();
        
        calc.push(0xDEAD);
        calc.store(5);
        assert_eq!(calc.memory[5], 0xDEAD);
        
        calc.x = 0;
        calc.recall(5);
        assert_eq!(calc.x, 0xDEAD);
    }

    #[test]
    fn test_rom_loading() {
        let mut rom = rom::Rom::new();
        
        // Test with a mock ROM file (this would normally load from 16c.obj)
        // For now, just test the basic functionality
        assert_eq!(rom.size(), 0);
        assert_eq!(rom.read(0x1000), 0); // Should return 0 for uninitialized memory
    }
}