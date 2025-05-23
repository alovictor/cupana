use crate::assembler::definitions::{InstructionType, Opcode, Operand, Register};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub enum CodeGenError {
    UnsupportedInstruction(String),
    InvalidOperandForInstruction(String),
    MissingLabel(String),
    OperandIsNotRegister(Operand),
    OperandIsNotLiteral(Operand),
    OperandIsNotDereferencedRegister(Operand),
    OperandCombinationNotSupported(InstructionType),
    RegisterIdOutOfRange(u8),
    LiteralOutOfRange(u16),
}

fn get_reg(op: &Operand) -> Result<Register, CodeGenError> {
    match op {
        Operand::Register(r) => Ok(*r),
        _ => Err(CodeGenError::OperandIsNotRegister(op.clone())),
    }
}

pub fn generate_instruction_code(
    instruction: &InstructionType,
    symbol_table: &HashMap<String, u16>,
    _current_address: u16,
) -> Result<Vec<u16>, CodeGenError> {
    match instruction {
        InstructionType::Nop => Ok(vec![(Opcode::Nop as u16) << 8]),
        InstructionType::Hlt => Ok(vec![(Opcode::Hlt as u16) << 8]),
        InstructionType::Ret => Ok(vec![(Opcode::Ret as u16) << 8]),

        InstructionType::Mov { dest, src } => match (dest, src) {
            (Operand::Register(rd), Operand::Register(rs)) => {
                Ok(vec![
                    ((Opcode::MovRegReg as u16) << 8) | ((rd.0 as u16) << 4) | (rs.0 as u16)
                ])
            }
            (Operand::Register(rd), Operand::Literal(val)) => {
                Ok(vec![
                    ((Opcode::MovRegLit as u16) << 8) | ((rd.0 as u16) << 4),
                    *val,
                ])
            }
            (Operand::Register(rd), Operand::DereferencedRegister(rs_ptr)) => {
                Ok(vec![
                    ((Opcode::MovRegRegInd as u16) << 8) | ((rd.0 as u16) << 4) | (rs_ptr.0 as u16)
                ])
            }
            (Operand::DereferencedRegister(rd_ptr), Operand::Register(rs)) => {
                Ok(vec![
                    ((Opcode::MovRegIndReg as u16) << 8) | ((rd_ptr.0 as u16) << 4) | (rs.0 as u16)
                ])
            }
            (Operand::Literal(mem_addr), Operand::Register(rs)) => { // MovMemReg (MOV [LIT], Rs)
                 Ok(vec![
                    ((Opcode::MovMemReg as u16) << 8) | (rs.0 as u16), 
                    *mem_addr,
                ])
            }
            _ => Err(CodeGenError::OperandCombinationNotSupported(instruction.clone())),
        },

        InstructionType::Add { dest, src } => match (dest, src) {
            (Operand::Register(rd), Operand::Register(rs)) => {
                Ok(vec![
                    ((Opcode::AddRegReg as u16) << 8) | ((rd.0 as u16) << 4) | (rs.0 as u16)
                ])
            }
            (Operand::Register(rd), Operand::Literal(val)) => {
                Ok(vec![
                    ((Opcode::AddRegLit as u16) << 8) | ((rd.0 as u16) << 4),
                    *val,
                ])
            }
            _ => Err(CodeGenError::OperandCombinationNotSupported(instruction.clone())),
        },
        InstructionType::Sub { dest, src } => match (dest, src) {
            (Operand::Register(rd), Operand::Register(rs)) => {
                Ok(vec![
                    ((Opcode::SubRegReg as u16) << 8) | ((rd.0 as u16) << 4) | (rs.0 as u16)
                ])
            }
            (Operand::Register(rd), Operand::Literal(val)) => {
                Ok(vec![
                    ((Opcode::SubRegLit as u16) << 8) | ((rd.0 as u16) << 4),
                    *val,
                ])
            }
            (Operand::Literal(lit), Operand::Register(rs)) => {
                 Ok(vec![
                    ((Opcode::SubLitReg as u16) << 8) | (rs.0 as u16), 
                    *lit,
                ])
            }
            _ => Err(CodeGenError::OperandCombinationNotSupported(instruction.clone())),
        },
        InstructionType::Mul { dest, src } => match (dest, src) {
            (Operand::Register(rd), Operand::Register(rs)) => {
                Ok(vec![
                    ((Opcode::MulRegReg as u16) << 8) | ((rd.0 as u16) << 4) | (rs.0 as u16)
                ])
            }
            (Operand::Register(rd), Operand::Literal(val)) => {
                Ok(vec![
                    ((Opcode::MulRegLit as u16) << 8) | ((rd.0 as u16) << 4),
                    *val,
                ])
            }
            _ => Err(CodeGenError::OperandCombinationNotSupported(instruction.clone())),
        },
        InstructionType::Div { dest, src } => match (dest, src) {
            (Operand::Register(rd), Operand::Register(rs)) => {
                Ok(vec![
                    ((Opcode::DivRegReg as u16) << 8) | ((rd.0 as u16) << 4) | (rs.0 as u16)
                ])
            }
            (Operand::Register(rd), Operand::Literal(val)) => {
                Ok(vec![
                    ((Opcode::DivRegLit as u16) << 8) | ((rd.0 as u16) << 4),
                    *val,
                ])
            }
            (Operand::Literal(lit), Operand::Register(rs)) => {
                 Ok(vec![
                    ((Opcode::DivLitReg as u16) << 8) | (rs.0 as u16),
                    *lit,
                ])
            }
            _ => Err(CodeGenError::OperandCombinationNotSupported(instruction.clone())),
        },
         InstructionType::Mod { dest, src } => match (dest, src) {
            (Operand::Register(rd), Operand::Register(rs)) => {
                Ok(vec![
                    ((Opcode::ModRegReg as u16) << 8) | ((rd.0 as u16) << 4) | (rs.0 as u16)
                ])
            }
            (Operand::Register(rd), Operand::Literal(val)) => {
                Ok(vec![
                    ((Opcode::ModRegLit as u16) << 8) | ((rd.0 as u16) << 4),
                    *val,
                ])
            }
            (Operand::Literal(lit), Operand::Register(rs)) => {
                 Ok(vec![
                    ((Opcode::ModLitReg as u16) << 8) | (rs.0 as u16),
                    *lit,
                ])
            }
            _ => Err(CodeGenError::OperandCombinationNotSupported(instruction.clone())),
        },
        InstructionType::Inc { reg } => {
            let r = get_reg(reg)?;
            Ok(vec![((Opcode::IncReg as u16) << 8) | ((r.0 as u16) << 4)])
        }
        InstructionType::Dec { reg } => {
            let r = get_reg(reg)?;
            Ok(vec![((Opcode::DecReg as u16) << 8) | ((r.0 as u16) << 4)])
        }
        InstructionType::Not { reg } => {
            let r = get_reg(reg)?;
            Ok(vec![((Opcode::NotReg as u16) << 8) | ((r.0 as u16) << 4)])
        }
        InstructionType::And { dest, src } | InstructionType::Or { dest, src } | InstructionType::Xor { dest, src } => {
            let rd = get_reg(dest)?;
            let rs = get_reg(src)?;
            let opcode_val = match instruction {
                InstructionType::And {..} => Opcode::AndRegReg,
                InstructionType::Or {..} => Opcode::OrRegReg,
                InstructionType::Xor {..} => Opcode::XorRegReg,
                _ => unreachable!(),
            };
            Ok(vec![ ((opcode_val as u16) << 8) | ((rd.0 as u16) << 4) | (rs.0 as u16) ])
        }
        InstructionType::Cmp { op1, op2 } => match (op1, op2) {
            (Operand::Register(r1), Operand::Register(r2)) => {
                Ok(vec![
                    ((Opcode::CmpRegReg as u16) << 8) | ((r1.0 as u16) << 4) | (r2.0 as u16)
                ])
            }
            (Operand::Register(r1), Operand::Literal(val)) => {
                Ok(vec![
                    ((Opcode::CmpRegLit as u16) << 8) | ((r1.0 as u16) << 4),
                    *val,
                ])
            }
            _ => Err(CodeGenError::OperandCombinationNotSupported(instruction.clone())),
        },
        InstructionType::Jmp { target } => match target {
            Operand::Register(r) => Ok(vec![((Opcode::JmpReg as u16) << 8) | ((r.0 as u16) << 4)]),
            Operand::Literal(addr) => Ok(vec![((Opcode::JmpLit as u16) << 8), *addr]),
            Operand::Label(name) => {
                let addr = symbol_table.get(name).ok_or_else(|| CodeGenError::MissingLabel(name.clone()))?;
                Ok(vec![((Opcode::JmpLit as u16) << 8), *addr])
            }
            _ => Err(CodeGenError::InvalidOperandForInstruction("JMP expects Register, Literal or Label".to_string())),
        },
        InstructionType::Jz { target } => match target {
            Operand::Register(r) => Ok(vec![((Opcode::JzReg as u16) << 8) | (r.0 as u16) << 4]),
            Operand::Literal(addr) => Ok(vec![((Opcode::JzLit as u16) << 8), *addr]),
            Operand::Label(name) => {
                let addr = symbol_table.get(name).ok_or_else(|| CodeGenError::MissingLabel(name.clone()))?;
                Ok(vec![((Opcode::JzLit as u16) << 8), *addr])
            }
            _ => Err(CodeGenError::InvalidOperandForInstruction("JZ expects Register, Literal or Label".to_string())),
        },
        InstructionType::Jnz { target } => match target {
            Operand::Register(r) => Ok(vec![((Opcode::JnzReg as u16) << 8) | (r.0 as u16) << 4]),
            Operand::Literal(addr) => Ok(vec![((Opcode::JnzLit as u16) << 8), *addr]),
            Operand::Label(name) => {
                let addr = symbol_table.get(name).ok_or_else(|| CodeGenError::MissingLabel(name.clone()))?;
                Ok(vec![((Opcode::JnzLit as u16) << 8), *addr])
            }
            _ => Err(CodeGenError::InvalidOperandForInstruction("JNZ expects Register, Literal or Label".to_string())),
        },
        InstructionType::Jn { target } => match target {
            Operand::Register(r) => Ok(vec![((Opcode::JnReg as u16) << 8) | (r.0 as u16) << 4]),
            Operand::Literal(addr) => Ok(vec![((Opcode::JnLit as u16) << 8), *addr]),
            Operand::Label(name) => {
                let addr = symbol_table.get(name).ok_or_else(|| CodeGenError::MissingLabel(name.clone()))?;
                Ok(vec![((Opcode::JnLit as u16) << 8), *addr])
            }
            _ => Err(CodeGenError::InvalidOperandForInstruction("JN expects Register, Literal or Label".to_string())),
        },
        InstructionType::Jnn { target } => match target {
            Operand::Register(r) => Ok(vec![((Opcode::JnnReg as u16) << 8) | (r.0 as u16) << 4]),
            Operand::Literal(addr) => Ok(vec![((Opcode::JnnLit as u16) << 8), *addr]),
            Operand::Label(name) => {
                let addr = symbol_table.get(name).ok_or_else(|| CodeGenError::MissingLabel(name.clone()))?;
                Ok(vec![((Opcode::JnnLit as u16) << 8), *addr])
            }
            _ => Err(CodeGenError::InvalidOperandForInstruction("JNN expects Register, Literal or Label".to_string())),
        },
        InstructionType::Jc { target } => match target {
            Operand::Register(r) => Ok(vec![((Opcode::JcReg as u16) << 8) | (r.0 as u16) << 4]),
            Operand::Literal(addr) => Ok(vec![((Opcode::JcLit as u16) << 8), *addr]),
            Operand::Label(name) => {
                let addr = symbol_table.get(name).ok_or_else(|| CodeGenError::MissingLabel(name.clone()))?;
                Ok(vec![((Opcode::JcLit as u16) << 8), *addr])
            }
            _ => Err(CodeGenError::InvalidOperandForInstruction("JC expects Register, Literal or Label".to_string())),
        },
        InstructionType::Jnc { target } => match target {
            Operand::Register(r) => Ok(vec![((Opcode::JncReg as u16) << 8) | (r.0 as u16) << 4]),
            Operand::Literal(addr) => Ok(vec![((Opcode::JncLit as u16) << 8), *addr]),
            Operand::Label(name) => {
                let addr = symbol_table.get(name).ok_or_else(|| CodeGenError::MissingLabel(name.clone()))?;
                Ok(vec![((Opcode::JncLit as u16) << 8), *addr])
            }
            _ => Err(CodeGenError::InvalidOperandForInstruction("JNC expects Register, Literal or Label".to_string())),
        },
        InstructionType::Call { target } => match target {
            Operand::Literal(addr) => Ok(vec![((Opcode::CallLit as u16) << 8), *addr]),
            Operand::Label(name) => {
                let addr = symbol_table.get(name).ok_or_else(|| CodeGenError::MissingLabel(name.clone()))?;
                Ok(vec![((Opcode::CallLit as u16) << 8), *addr])
            }
            _ => Err(CodeGenError::InvalidOperandForInstruction("CALL expects Literal or Label".to_string())),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assembler::definitions::{InstructionType, Opcode, Operand, Register};
    use std::collections::HashMap;

    fn assert_code_with_symtable(
        instruction: InstructionType,
        expected_code: Vec<u16>,
        symbol_table: &HashMap<String, u16>,
    ) {
        let current_address = 0u16;
        match generate_instruction_code(&instruction, symbol_table, current_address) {
            Ok(code) => assert_eq!(code, expected_code),
            Err(e) => panic!(
                "Code generation failed for {:?}: {:?}, expected {:?}, symtable: {:?}",
                instruction, e, expected_code, symbol_table
            ),
        }
    }

    fn assert_code(instruction: InstructionType, expected_code: Vec<u16>) {
        let symbol_table = HashMap::new();
        assert_code_with_symtable(instruction, expected_code, &symbol_table);
    }

    fn assert_error_with_symtable(
        instruction: InstructionType,
        expected_error_variant: CodeGenError, 
        symbol_table: &HashMap<String, u16>,
    ) {
        let current_address = 0u16;
        match generate_instruction_code(&instruction, symbol_table, current_address) {
            Ok(code) => panic!(
                "Expected error {:?} for {:?}, but got code {:?}",
                expected_error_variant, instruction, code
            ),
            Err(e) => {
                match (&e, &expected_error_variant) {
                    (CodeGenError::UnsupportedInstruction(_), CodeGenError::UnsupportedInstruction(_)) => assert!(true),
                    (CodeGenError::InvalidOperandForInstruction(_), CodeGenError::InvalidOperandForInstruction(_)) => assert!(true),
                    (CodeGenError::MissingLabel(_), CodeGenError::MissingLabel(_)) => assert!(true),
                    (CodeGenError::OperandIsNotRegister(_), CodeGenError::OperandIsNotRegister(_)) => assert!(true),
                    (CodeGenError::OperandIsNotLiteral(_), CodeGenError::OperandIsNotLiteral(_)) => assert!(true),
                    (CodeGenError::OperandIsNotDereferencedRegister(_), CodeGenError::OperandIsNotDereferencedRegister(_)) => assert!(true),
                    (CodeGenError::OperandCombinationNotSupported(_), CodeGenError::OperandCombinationNotSupported(_)) => assert!(true),
                    _ => assert_eq!(e, expected_error_variant) 
                }
            }
        }
    }
    
    fn assert_error(instruction: InstructionType, expected_error_variant: CodeGenError) {
        let symbol_table = HashMap::new();
        assert_error_with_symtable(instruction, expected_error_variant, &symbol_table);
    }

    #[test]
    fn test_nop() {
        assert_code(InstructionType::Nop, vec![(Opcode::Nop as u16) << 8]);
    }

    #[test]
    fn test_mov_reg_reg() {
        assert_code(
            InstructionType::Mov {
                dest: Operand::Register(Register(1)),
                src: Operand::Register(Register(2)),
            },
            vec![((Opcode::MovRegReg as u16) << 8) | (1 << 4) | 2],
        );
    }
    
    #[test]
    fn test_inc_reg_error_literal() {
        assert_error(
            InstructionType::Inc { reg: Operand::Literal(7) },
            CodeGenError::OperandIsNotRegister(Operand::Literal(7)),
        );
    }

    #[test]
    fn test_jmp_label() {
        let mut sym = HashMap::new();
        sym.insert("MY_LABEL".to_string(), 0x1000);
        assert_code_with_symtable(
            InstructionType::Jmp { target: Operand::Label("MY_LABEL".to_string()) },
            vec![(Opcode::JmpLit as u16) << 8, 0x1000],
            &sym,
        );
    }

    #[test]
    fn test_jmp_label_missing() {
        assert_error(
            InstructionType::Jmp { target: Operand::Label("MISSING_LABEL".to_string()) },
            CodeGenError::MissingLabel("MISSING_LABEL".to_string()),
        );
    }
    
    #[test]
    fn test_call_reg_error() { 
        assert_error(
            InstructionType::Call { target: Operand::Register(Register(1)) },
            CodeGenError::InvalidOperandForInstruction("CALL expects Literal or Label".to_string()),
        );
    }
}
