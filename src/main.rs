mod rom;
mod cpu;

use cpu::Hp16cCpu;
use rustyline::error::ReadlineError;
use rustyline::{Editor, Result};
use rustyline::completion::{Completer, Pair};
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::validate::Validator;
use rustyline::{Context, Helper};
use std::borrow::Cow;
use std::collections::HashSet;
use std::io;

struct Hp16cHelper {
    completer: Hp16cCompleter,
}

impl Helper for Hp16cHelper {}

impl Completer for Hp16cHelper {
    type Candidate = Pair;
    
    fn complete(
        &self,
        line: &str,
        pos: usize,
        ctx: &Context<'_>,
    ) -> Result<(usize, Vec<Pair>)> {
        self.completer.complete(line, pos, ctx)
    }
}

impl Hinter for Hp16cHelper {
    type Hint = String;
    
    fn hint(&self, _line: &str, _pos: usize, _ctx: &Context<'_>) -> Option<String> {
        None
    }
}

impl Highlighter for Hp16cHelper {
    fn highlight_prompt<'b, 's: 'b, 'p: 'b>(
        &'s self,
        prompt: &'p str,
        default: bool,
    ) -> Cow<'b, str> {
        if default {
            Cow::Borrowed(prompt)
        } else {
            Cow::Owned(format!("\x1b[1;32m{}\x1b[0m", prompt))
        }
    }
    
    fn highlight_hint<'h>(&self, hint: &'h str) -> Cow<'h, str> {
        Cow::Owned(format!("\x1b[1;30m{}\x1b[0m", hint))
    }
}

impl Validator for Hp16cHelper {}

struct Hp16cCompleter {
    commands: HashSet<String>,
}

impl Hp16cCompleter {
    fn new() -> Self {
        let mut commands = HashSet::new();
        
        // Basic commands
        commands.insert("HELP".to_string());
        commands.insert("QUIT".to_string());
        commands.insert("CLEAR".to_string());
        commands.insert("CLR".to_string());
        
        // Stack operations
        commands.insert("ENTER".to_string());
        commands.insert("DROP".to_string());
        commands.insert("SWAP".to_string());
        commands.insert("RV".to_string());
        commands.insert("R^".to_string());
        
        // Number bases
        commands.insert("HEX".to_string());
        commands.insert("DEC".to_string());
        commands.insert("OCT".to_string());
        commands.insert("BIN".to_string());
        
        // Memory operations (with space for parameter)
        for i in 0..16 {
            commands.insert(format!("STO {}", i));
            commands.insert(format!("RCL {}", i));
        }
        
        // Word size operations (common sizes)
        for size in [1, 2, 4, 8, 16, 32, 64, 128] {
            commands.insert(format!("WS {}", size));
        }
        
        // Shift operations (common shift amounts)
        for shift in 1..=8 {
            commands.insert(format!("SL {}", shift));
            commands.insert(format!("SR {}", shift));
        }
        
        Self { commands }
    }
}

impl Completer for Hp16cCompleter {
    type Candidate = Pair;
    
    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &Context<'_>,
    ) -> Result<(usize, Vec<Pair>)> {
        let line_upper = line.to_uppercase();
        let mut matches = Vec::new();
        
        // Find the start of the current word
        let start = line[..pos].rfind(' ').map_or(0, |i| i + 1);
        let word = &line_upper[start..pos];
        
        // Find matching commands
        for command in &self.commands {
            if command.starts_with(word) {
                matches.push(Pair {
                    display: command.clone(),
                    replacement: command.clone(),
                });
            }
        }
        
        // Sort matches
        matches.sort_by(|a, b| a.display.cmp(&b.display));
        
        Ok((start, matches))
    }
}

fn main() {
    let mut calculator = Hp16cCpu::new();
    
    // Load ROM data
    if let Err(e) = calculator.load_rom("16c.obj") {
        eprintln!("Warning: Could not load ROM file: {}", e);
        eprintln!("Continuing without ROM data...");
    }

    println!("HP-16C RPN Calculator Emulator");
    println!("==============================");
    println!("Type HELP for detailed command information, or QUIT to exit.");
    println!("Use TAB for command completion.");
    println!();

    // Set up rustyline with completion
    let h = Hp16cHelper {
        completer: Hp16cCompleter::new(),
    };
    
    let mut rl: Editor<Hp16cHelper, _> = Editor::new().unwrap();
    rl.set_helper(Some(h));
    
    // Load history if available
    let _ = rl.load_history("hp16c_history.txt");

    loop {
        display_calculator(&calculator);
        
        let readline = rl.readline("> ");
        let input = match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str()).unwrap();
                line.trim().to_uppercase()
            }
            Err(ReadlineError::Interrupted) => {
                println!("^C");
                continue;
            }
            Err(ReadlineError::Eof) => {
                println!("^D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                continue;
            }
        };
        
        if input.is_empty() {
            continue;
        }
        
        match input.as_str() {
            "QUIT" | "Q" => break,
            "HELP" | "H" | "?" => {
                show_help();
                continue;
            },
            "CLR" | "CLEAR" => {
                calculator.x = 0;
                calculator.y = 0;
                calculator.z = 0;
                calculator.t = 0;
            },
            "ENTER" => {
                calculator.push(calculator.x);
            },
            "DROP" => {
                calculator.drop();
            },
            "SWAP" => {
                calculator.swap_xy();
            },
            "RV" => {
                calculator.roll_down();
            },
            "R^" => {
                calculator.roll_up();
            },
            "+" => {
                calculator.add();
            },
            "-" => {
                calculator.subtract();
            },
            "*" => {
                calculator.multiply();
            },
            "/" => {
                calculator.divide();
            },
            "&" => {
                calculator.and();
            },
            "|" => {
                calculator.or();
            },
            "^" => {
                calculator.xor();
            },
            "~" => {
                calculator.not();
            },
            "BIN" => {
                calculator.set_base(2);
            },
            "OCT" => {
                calculator.set_base(8);
            },
            "DEC" => {
                calculator.set_base(10);
            },
            "HEX" => {
                calculator.set_base(16);
            },
            _ => {
                // Check for memory operations
                if input.starts_with("STO ") {
                    if let Ok(reg) = input[4..].parse::<usize>() {
                        calculator.store(reg);
                    } else {
                        println!("Invalid register number");
                    }
                } else if input.starts_with("RCL ") {
                    if let Ok(reg) = input[4..].parse::<usize>() {
                        calculator.recall(reg);
                    } else {
                        println!("Invalid register number");
                    }
                } else if input.starts_with("WS ") {
                    if let Ok(size) = input[3..].parse::<u8>() {
                        calculator.set_word_size(size);
                    } else {
                        println!("Invalid word size (1-128)");
                    }
                } else if input.starts_with("SL ") {
                    if let Ok(positions) = input[3..].parse::<u8>() {
                        calculator.shift_left(positions);
                    } else {
                        println!("Invalid shift count");
                    }
                } else if input.starts_with("SR ") {
                    if let Ok(positions) = input[3..].parse::<u8>() {
                        calculator.shift_right(positions);
                    } else {
                        println!("Invalid shift count");
                    }
                } else {
                    // Try to parse as number in current base
                    let parsed_value = match calculator.base {
                        2 => u128::from_str_radix(&input, 2),
                        8 => u128::from_str_radix(&input, 8),
                        10 => input.parse::<u128>(),
                        16 => u128::from_str_radix(&input, 16),
                        _ => u128::from_str_radix(&input, 16),
                    };
                    
                    match parsed_value {
                        Ok(value) => {
                            calculator.push(value);
                        },
                        Err(_) => {
                            println!("Unknown command or invalid number: {}", input);
                        }
                    }
                }
            }
        }
    }
    
    // Save history
    let _ = rl.save_history("hp16c_history.txt");
    println!("Goodbye!");
}

fn display_calculator(calc: &Hp16cCpu) {
    println!();
    
    // Calculate the required width based on the longest stack display
    let stack = calc.get_stack_display();
    let title = "HP-16C Calculator";
    let status_line = format!("Base: {:2}  Word Size: {:2}", calc.base, calc.word_size);
    let flags_line = format!("Carry: {}  Overflow: {}", 
                            if calc.carry { "1" } else { "0" },
                            if calc.overflow { "1" } else { "0" });
    
    // Find the maximum width needed
    let mut max_width = title.len().max(status_line.len()).max(flags_line.len());
    for line in &stack {
        max_width = max_width.max(line.len());
    }
    
    // Ensure minimum width and add padding for borders
    let display_width = max_width.max(29) + 2; // +2 for left and right padding
    
    // Create border strings
    let top_border = format!("┌{}┐", "─".repeat(display_width));
    let mid_border = format!("├{}┤", "─".repeat(display_width));
    let bottom_border = format!("└{}┘", "─".repeat(display_width));
    
    // Display the calculator
    println!("{}", top_border);
    println!("│ {:width$} │", title, width = display_width - 2);
    println!("{}", mid_border);
    println!("│ {:width$} │", status_line, width = display_width - 2);
    println!("│ {:width$} │", flags_line, width = display_width - 2);
    println!("{}", mid_border);
    
    for line in &stack {
        println!("│ {:width$} │", line, width = display_width - 2);
    }
    
    println!("{}", bottom_border);
}

fn show_help() {
    println!();
    println!("═══════════════════════════════════════════════════════════════════════");
    println!("                          HP-16C CALCULATOR HELP");
    println!("═══════════════════════════════════════════════════════════════════════");
    println!();
    
    println!("📋 BASIC USAGE:");
    println!("  • Enter numbers in the current base and press ENTER to push to stack");
    println!("  • Operations consume stack values (RPN - Reverse Polish Notation)");
    println!("  • Use TAB key for command completion while typing");
    println!("  • Example: To calculate 10 + 5: type '10', 'ENTER', '5', '+'");
    println!();
    
    println!("🔢 NUMBER ENTRY:");
    println!("  Command    Description                    Example");
    println!("  ─────────  ──────────────────────────────  ───────────────────────");
    println!("  [number]   Enter number in current base   FF (hex), 255 (dec)");
    println!("  ENTER      Push X to stack (duplicate)    10 ENTER → stack: [10,10]");
    println!();
    println!("  Example sequence:");
    println!("    • Type 'A' → X register shows A (10 in hex)");
    println!("    • Type 'ENTER' → Push A to Y, X still shows A");
    println!("    • Type '5' → X shows 5, Y shows A");
    println!();
    
    println!("🧮 ARITHMETIC OPERATIONS:");
    println!("  Command    Description                    Example");
    println!("  ─────────  ──────────────────────────────  ───────────────────────");
    println!("  +          Add Y + X                      10 ENTER 5 + → 15");
    println!("  -          Subtract Y - X                 10 ENTER 3 - → 7");
    println!("  *          Multiply Y × X                 6 ENTER 7 * → 42");
    println!("  /          Divide Y ÷ X                   20 ENTER 4 / → 5");
    println!();
    println!("  Example: Calculate (15 + 25) × 2:");
    println!("    15 ENTER 25 + 2 * → Result: 80");
    println!();
    
    println!("🔧 BITWISE OPERATIONS:");
    println!("  Command    Description                    Example");
    println!("  ─────────  ──────────────────────────────  ───────────────────────");
    println!("  &          Bitwise AND of Y & X           F0 ENTER 0F & → 0");
    println!("  |          Bitwise OR of Y | X            F0 ENTER 0F | → FF");
    println!("  ^          Bitwise XOR of Y ^ X           FF ENTER AA ^ → 55");
    println!("  ~          Bitwise NOT of X               FF ~ → 0 (in 8-bit mode)");
    println!();
    println!("  Example: Mask lower 4 bits of FF:");
    println!("    FF ENTER 0F & → Result: 0F");
    println!();
    
    println!("↕️  STACK MANIPULATION:");
    println!("  Command    Description                    Example");
    println!("  ─────────  ──────────────────────────────  ───────────────────────");
    println!("  DROP       Remove X, lift stack up        [4,3,2,1] DROP → [3,2,1,1]");
    println!("  SWAP       Exchange X and Y               [4,3,2,1] SWAP → [3,4,2,1]");
    println!("  RV         Roll stack down               [4,3,2,1] RV → [3,2,1,4]");
    println!("  R^         Roll stack up                 [4,3,2,1] R^ → [1,4,3,2]");
    println!();
    println!("  Note: Stack format shown as [T,Z,Y,X] where X is display register");
    println!();
    
    println!("🔢 NUMBER BASE CONVERSION:");
    println!("  Command    Description                    Example");
    println!("  ─────────  ──────────────────────────────  ───────────────────────");
    println!("  HEX        Switch to hexadecimal         255 HEX → displays as FF");
    println!("  DEC        Switch to decimal             FF DEC → displays as 255");
    println!("  OCT        Switch to octal               255 OCT → displays as 377");
    println!("  BIN        Switch to binary              255 BIN → displays as 11111111");
    println!();
    println!("  Example: Convert hex FF to decimal:");
    println!("    FF → shows FF, then DEC → shows 255");
    println!();
    
    println!("📏 WORD SIZE CONTROL:");
    println!("  Command    Description                    Example");
    println!("  ─────────  ──────────────────────────────  ───────────────────────");
    println!("  WS [n]     Set word size (1-128 bits)    WS 8 → 8-bit arithmetic");
    println!();
    println!("  Example: Set 4-bit mode and see overflow:");
    println!("    WS 4 → 4-bit mode");
    println!("    10 → shows 0 (10 masked to 4 bits)");
    println!("    F → shows F (15, max for 4 bits)");
    println!();
    
    println!("🔄 SHIFT OPERATIONS:");
    println!("  Command    Description                    Example");
    println!("  ─────────  ──────────────────────────────  ───────────────────────");
    println!("  SL [n]     Shift left n positions        5 SL 1 → A (5<<1 = 10)");
    println!("  SR [n]     Shift right n positions       A SR 1 → 5 (10>>1 = 5)");
    println!();
    println!("  Example: Multiply by 4 using shifts:");
    println!("    7 SL 2 → 1C (7 shifted left 2 = 7×4 = 28)");
    println!();
    
    println!("💾 MEMORY OPERATIONS:");
    println!("  Command    Description                    Example");
    println!("  ─────────  ──────────────────────────────  ───────────────────────");
    println!("  STO [n]    Store X in register n (0-15)  42 STO 5 → saves 42 to R5");
    println!("  RCL [n]    Recall register n to stack    RCL 5 → pushes R5 to stack");
    println!();
    println!("  Example: Store intermediate result:");
    println!("    10 ENTER 5 + STO 1 → store 15 in R1");
    println!("    20 ENTER 3 * → calculate 60");
    println!("    RCL 1 + → add stored 15, result: 75");
    println!();
    
    println!("🧹 UTILITY COMMANDS:");
    println!("  Command    Description                    Example");
    println!("  ─────────  ──────────────────────────────  ───────────────────────");
    println!("  CLR        Clear all stack registers     CLR → all registers = 0");
    println!("  HELP       Show this help (also H, ?)    HELP → shows this screen");
    println!("  QUIT       Exit calculator (also Q)      QUIT → exits program");
    println!("  TAB        Auto-complete commands         HE<TAB> → completes to HELP");
    println!();
    
    println!("📊 CALCULATOR DISPLAY:");
    println!("  • T, Z, Y, X: The four-level RPN stack");
    println!("  • Base: Current number base (2, 8, 10, or 16)");
    println!("  • Word Size: Current bit width (1-64)");
    println!("  • Carry: Set when arithmetic operation carries/borrows");
    println!("  • Overflow: Set when result exceeds word size");
    println!();
    
    println!("💡 SAMPLE CALCULATIONS:");
    println!();
    println!("  1. Convert 255 to different bases:");
    println!("     255 DEC → shows 255");
    println!("     HEX → shows FF");
    println!("     OCT → shows 377");
    println!("     BIN → shows 11111111");
    println!();
    
    println!("  2. Calculate percentage using bitwise (what % of FF is 80?):");
    println!("     80 ENTER FF / 100 * → shows percentage");
    println!();
    
    println!("  3. Check if a number is power of 2:");
    println!("     8 ENTER 8 ENTER 1 - & → result 0 means power of 2");
    println!();
    
    println!("  4. Extract lower nibble (4 bits):");
    println!("     A5 ENTER F & → result: 5");
    println!();
    
    println!("  5. Set specific bit (set bit 3 in value 10):");
    println!("     10 ENTER 1 3 SL | → result: 18 (10 | 8)");
    println!();
    
    println!("═══════════════════════════════════════════════════════════════════════");
    println!("Press any key to continue...");
    println!();
    
    // Wait for user input
    let mut dummy = String::new();
    let _ = io::stdin().read_line(&mut dummy);
}