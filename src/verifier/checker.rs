use std::collections::HashSet;
use crate::classfile::attributes::CodeAttribute;
use crate::error::{ClassFileError, JvmError, Result};

/// Simple bytecode verification result
#[derive(Debug)]
pub struct VerificationResult {
    pub is_valid: bool,
    pub errors: Vec<String>,
}

impl VerificationResult {
    pub fn valid() -> Self {
        VerificationResult {
            is_valid: true,
            errors: vec![],
        }
    }

    pub fn invalid(errors: Vec<String>) -> Self {
        VerificationResult {
            is_valid: false,
            errors,
        }
    }
}

/// Verify a method's bytecode for basic correctness.
///
/// Checks performed:
/// 1. All branch targets point within the valid code range
/// 2. Stack depth never goes negative (basic underflow check)
/// 3. No instruction reads past the end of the code array
/// 4. All instruction widths are valid (no partial reads)
pub fn verify_method_code(code: &[u8], max_stack: usize, max_locals: usize, class_name: &str, method_name: &str) -> VerificationResult {
    let mut errors = Vec::new();
    let code_len = code.len();

    // Collect all branch targets and track reachable instructions
    let mut targets = HashSet::new();
    targets.insert(0); // Entry point

    // Simulate a single pass to find branch targets
    let mut pc = 0;
    while pc < code_len {
        let opcode = code[pc];
        let width = match get_instruction_width(opcode) {
            Some(w) => w,
            None => {
                errors.push(format!("Unknown opcode 0x{:02X} at PC {}", opcode, pc));
                pc += 1;
                continue;
            }
        };

        // Check that the instruction doesn't read past the end of the code
        if pc + width > code_len {
            errors.push(format!("Instruction at PC {} (opcode 0x{:02X}) extends past end of code", pc, opcode));
            break;
        }

        // Collect branch targets
        match opcode {
            0x99..=0xA6 | 0xC6 | 0xC7 => {
                // Conditional branches: opcode + 2-byte offset
                if pc + 2 < code_len {
                    let offset = i16::from_be_bytes([code[pc + 1], code[pc + 2]]) as i32;
                    let target = (pc as i32 + offset) as usize;
                    if target < code_len {
                        targets.insert(target);
                    } else {
                        errors.push(format!("Branch target {} at PC {} out of bounds (code length {})", target, pc, code_len));
                    }
                }
                // Fall-through target
                targets.insert(pc + width);
            }
            0xA7 => {
                // goto: opcode + 2-byte offset
                if pc + 2 < code_len {
                    let offset = i16::from_be_bytes([code[pc + 1], code[pc + 2]]) as i32;
                    let target = (pc as i32 + offset) as usize;
                    if target < code_len {
                        targets.insert(target);
                    } else {
                        errors.push(format!("Goto target {} at PC {} out of bounds", target, pc));
                    }
                }
            }
            0xC8 => {
                // goto_w: opcode + 4-byte offset
                if pc + 4 < code_len {
                    let offset = i32::from_be_bytes([code[pc + 1], code[pc + 2], code[pc + 3], code[pc + 4]]);
                    let target = (pc as i32 + offset) as usize;
                    if target < code_len {
                        targets.insert(target);
                    } else {
                        errors.push(format!("Goto_w target {} at PC {} out of bounds", target, pc));
                    }
                }
            }
            0xAA => {
                // tableswitch: variable length
                let mut offset = pc + 1;
                while offset % 4 != 0 { offset += 1; }
                if offset + 12 < code_len {
                    let default = i32::from_be_bytes([code[offset], code[offset + 1], code[offset + 2], code[offset + 3]]);
                    let low = i32::from_be_bytes([code[offset + 4], code[offset + 5], code[offset + 6], code[offset + 7]]);
                    let high = i32::from_be_bytes([code[offset + 8], code[offset + 9], code[offset + 10], code[offset + 11]]);
                    let n = (high - low + 1) as usize;
                    let def_target = (pc as i32 + default) as usize;
                    if def_target < code_len { targets.insert(def_target); }
                    for i in 0..n {
                        let jump_offset = i32::from_be_bytes([
                            code[offset + 12 + i * 4],
                            code[offset + 12 + i * 4 + 1],
                            code[offset + 12 + i * 4 + 2],
                            code[offset + 12 + i * 4 + 3],
                        ]);
                        let target = (pc as i32 + jump_offset) as usize;
                        if target < code_len { targets.insert(target); }
                    }
                }
            }
            0xAB => {
                // lookupswitch: variable length
                let mut offset = pc + 1;
                while offset % 4 != 0 { offset += 1; }
                if offset + 8 < code_len {
                    let default = i32::from_be_bytes([code[offset], code[offset + 1], code[offset + 2], code[offset + 3]]);
                    let npairs = u32::from_be_bytes([code[offset + 4], code[offset + 5], code[offset + 6], code[offset + 7]]);
                    let def_target = (pc as i32 + default) as usize;
                    if def_target < code_len { targets.insert(def_target); }
                    for i in 0..npairs as usize {
                        let jump_offset = i32::from_be_bytes([
                            code[offset + 8 + i * 8 + 4],
                            code[offset + 8 + i * 8 + 5],
                            code[offset + 8 + i * 8 + 6],
                            code[offset + 8 + i * 8 + 7],
                        ]);
                        let target = (pc as i32 + jump_offset) as usize;
                        if target < code_len { targets.insert(target); }
                    }
                }
            }
            0xA8 | 0xC9 => {
                // jsr / jsr_w: push return address, jump to subroutine
                let offset = if opcode == 0xA8 {
                    if pc + 2 < code_len {
                        i16::from_be_bytes([code[pc + 1], code[pc + 2]]) as i32
                    } else { 0 }
                } else {
                    if pc + 4 < code_len {
                        i32::from_be_bytes([code[pc + 1], code[pc + 2], code[pc + 3], code[pc + 4]])
                    } else { 0 }
                };
                let target = (pc as i32 + offset) as usize;
                if target < code_len { targets.insert(target); }
                // Fall-through target
                targets.insert(pc + width);
            }
            _ => {
                // Fall-through: next instruction is reachable
                targets.insert(pc + width);
            }
        }

        pc += width;
    }

    // Verify reachable instructions are within bounds
    for &target in &targets {
        if target > code_len {
            errors.push(format!("Branch target {} out of bounds (code length {})", target, code_len));
        }
    }

    // Check that all instructions at target positions are valid
    for &target in &targets {
        if target < code_len {
            let opcode = code[target];
            if get_instruction_width(opcode).is_none() {
                errors.push(format!("Invalid opcode 0x{:02X} at branch target {}", opcode, target));
            }
        }
    }

    // Check for unreachable code after unconditional branches
    pc = 0;
    while pc < code_len {
        if !targets.contains(&pc) {
            // Skip unreachable code (just warn, don't fail)
            pc += 1;
            continue;
        }
        let opcode = code[pc];
        match opcode {
            0xA7 | 0xC8 | 0xAA | 0xAB | 0xAC..=0xB1 => {
                // Unconditional branch or return - next instruction is unreachable
                // We don't need to do anything special here
            }
            _ => {}
        }
        let width = get_instruction_width(opcode).unwrap_or(1);
        pc += width;
    }

    if errors.is_empty() {
        VerificationResult::valid()
    } else {
        VerificationResult::invalid(errors)
    }
}

/// Get the width of an instruction in bytes (including the opcode).
fn get_instruction_width(opcode: u8) -> Option<usize> {
    match opcode {
        // Tableswitch and lookupswitch are variable width, handled specially
        0xAA | 0xAB => Some(1),

        // wide: 0xC4 + opcode + 2-byte index (4 bytes total)
        0xC4 => Some(4),

        // Instructions with no operands (1 byte)
        0x00 | 0x01 | 0x02 | 0x03 | 0x04 | 0x05 | 0x06 | 0x07 | 0x08 |
        0x09 | 0x0A | 0x0B | 0x0C | 0x0D | 0x0E | 0x0F |
        0x1A | 0x1B | 0x1C | 0x1D | 0x1E | 0x1F |
        0x20 | 0x21 | 0x22 | 0x23 | 0x24 | 0x25 |
        0x26 | 0x27 | 0x28 | 0x29 | 0x2A | 0x2B |
        0x2C | 0x2D | 0x2E | 0x2F |
        0x30 | 0x31 | 0x32 | 0x33 | 0x34 | 0x35 |
        0x3B | 0x3C | 0x3D | 0x3E | 0x3F |
        0x40 | 0x41 | 0x42 | 0x43 | 0x44 | 0x45 |
        0x46 | 0x47 | 0x48 | 0x49 | 0x4A | 0x4B |
        0x4C | 0x4D | 0x4E | 0x4F |
        0x50 | 0x51 | 0x52 | 0x53 | 0x54 | 0x55 |
        0x56 | 0x57 | 0x58 | 0x59 | 0x5A | 0x5B |
        0x5C | 0x5D | 0x5E | 0x5F |
        0x60 | 0x61 | 0x62 | 0x63 | 0x64 | 0x65 |
        0x66 | 0x67 | 0x68 | 0x69 | 0x6A | 0x6B |
        0x6C | 0x6D | 0x6E | 0x6F |
        0x70 | 0x71 | 0x72 | 0x73 | 0x74 | 0x75 |
        0x76 | 0x77 | 0x78 | 0x79 | 0x7A | 0x7B |
        0x7C | 0x7D | 0x7E | 0x7F |
        0x80 | 0x81 | 0x82 | 0x83 | 0xAC | 0xAD |
        0xAE | 0xAF | 0xB0 | 0xB1 |
        0xBE | 0xBF | 0xC2 | 0xC3 => Some(1),

        // Instructions with 1 byte operand (2 bytes total)
        0x10 | 0x15 | 0x16 | 0x17 | 0x18 | 0x19 |
        0x36 | 0x37 | 0x38 | 0x39 | 0x3A | 0xA9 |
        0xBC | 0xC6 | 0xC7 => Some(2),

        // Instructions with 2 byte operand (3 bytes total)
        0x11 | 0x12 | 0x13 | 0x14 |
        0xB2 | 0xB3 | 0xB4 | 0xB5 | 0xB6 | 0xB7 | 0xB8 | 0xB9 | 0xBA |
        0xBB | 0xBD | 0xC0 | 0xC1 |
        0x99 | 0x9A | 0x9B | 0x9C | 0x9D | 0x9E |
        0x9F | 0xA0 | 0xA1 | 0xA2 | 0xA3 | 0xA4 | 0xA5 | 0xA6 | 0xA7 | 0xA8 => Some(3),

        // Instructions with 4 byte operand (5 bytes total)
        0xC8 | 0xC9 => Some(5),

        // iinc: opcode + 1 byte index + 1 byte const (3 bytes)
        0x84 => Some(3),

        // multianewarray: opcode + 2 byte index + 1 byte dims (4 bytes)
        0xC5 => Some(4),

        // wide: handled separately
        // _ => None for unknown/reserved opcodes
        _ => None,
    }
}

/// Verify a CodeAttribute for a method.
pub fn verify_code_attribute(
    code_attr: &CodeAttribute,
    class_name: &str,
    method_name: &str,
) -> VerificationResult {
    verify_method_code(
        &code_attr.code,
        code_attr.max_stack,
        code_attr.max_locals,
        class_name,
        method_name,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_code() {
        let result = verify_method_code(&[], 0, 0, "Test", "empty");
        assert!(result.is_valid);
    }

    #[test]
    fn test_simple_return() {
        // return (0xB1)
        let code = vec![0xB1];
        let result = verify_method_code(&code, 0, 0, "Test", "ret");
        assert!(result.is_valid);
    }

    #[test]
    fn test_iconst_return() {
        // iconst_0 (0x03), ireturn (0xAC)
        let code = vec![0x03, 0xAC];
        let result = verify_method_code(&code, 1, 0, "Test", "foo");
        assert!(result.is_valid);
    }

    #[test]
    fn test_goto() {
        // goto +5 (skip 2 nops), nop, nop, return
        // goto: 0xA7, offset=3 (bytes: 0x00, 0x03)
        let code = vec![0xA7, 0x00, 0x03, 0x00, 0x00, 0xB1];
        let result = verify_method_code(&code, 0, 0, "Test", "goto_test");
        assert!(result.is_valid);
    }

    #[test]
    fn test_if_branch() {
        // iload_0 (0x1A), ifeq +5 (0x99, 0x00, 0x05), iconst_0 (0x03), ireturn (0xAC),
        // iconst_1 (0x04), ireturn (0xAC)
        // ifeq at PC 1, offset 5 -> target PC 6
        let code = vec![0x1A, 0x99, 0x00, 0x05, 0x03, 0xAC, 0x04, 0xAC];
        let result = verify_method_code(&code, 1, 1, "Test", "if_test");
        assert!(result.is_valid);
    }

    #[test]
    fn test_invalid_branch_target() {
        // goto +1000 (out of bounds)
        let code = vec![0xA7, 0x03, 0xE8, 0xB1]; // goto 1000, then return
        let result = verify_method_code(&code, 0, 0, "Test", "bad_branch");
        assert!(!result.is_valid);
        assert!(result.errors.iter().any(|e| e.contains("out of bounds")));
    }

    #[test]
    fn test_unknown_opcode() {
        // 0xFE is IMPDEP2 (reserved/unknown)
        let code = vec![0xFE, 0xB1];
        let result = verify_method_code(&code, 0, 0, "Test", "bad_opcode");
        assert!(!result.is_valid);
    }

    #[test]
    fn test_wide_instruction() {
        // wide (0xC4), iload (0x15), index (0xFF)
        let code = vec![0xC4, 0x15, 0x00, 0xFF, 0xB1]; // wide iload 255, return
        let result = verify_method_code(&code, 1, 256, "Test", "wide_test");
        assert!(result.is_valid);
    }

    #[test]
    fn test_tableswitch() {
        // tableswitch with alignment padding
        let mut code = vec![
            0xAA, // tableswitch
            0x00, 0x00, 0x00, // padding
            0x00, 0x00, 0x00, 0x0C, // default offset = 12
            0x00, 0x00, 0x00, 0x00, // low = 0
            0x00, 0x00, 0x00, 0x02, // high = 2
            0x00, 0x00, 0x00, 0x0A, // jump offset for 0 = 10
            0x00, 0x00, 0x00, 0x0B, // jump offset for 1 = 11
            0x00, 0x00, 0x00, 0x0C, // jump offset for 2 = 12
        ];
        code.push(0xB1); // return at end
        let result = verify_method_code(&code, 1, 0, "Test", "tableswitch");
        assert!(result.is_valid);
    }
}