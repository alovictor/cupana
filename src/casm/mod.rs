pub mod lexer;
pub mod parser;

use crate::casm::parser::{Instruction, Operand, Parser, Program, Statement};
use crate::error::AssembleError;
use indexmap::IndexMap;
use std::fs;
use std::path::Path;

// Helper enum for get_instruction_size
#[derive(Debug, Clone, PartialEq)]
enum ResolvedOperandType {
    RegisterLike, // Register or RegisterIndirect
    LiteralLike,  // Literal or LabelRef (which implies a literal address)
}

pub struct Assembler {
    // program: Option<Program>, // Not directly used in methods after parsing, consider removing or using
    output: [u8; 0x8000],
    current_address: u16,
    org_address: u16,
}

impl Assembler {
    pub fn new() -> Self {
        Self {
            // program: None, // Parsed program is passed by arg where needed
            output: [0; 0x8000],
            current_address: 0,
            org_address: 0,
        }
    }

    pub fn assemble_file<P: AsRef<Path>>(
        &mut self,
        input_path: P,
    ) -> Result<&[u8; 0x8000], AssembleError> {
        let content = fs::read_to_string(input_path)?;
        self.assemble_string(&content)
    }

    pub fn assemble_to_file<P: AsRef<Path>>(
        &mut self,
        input_path: P,
        output_path: P,
    ) -> Result<(), AssembleError> {
        let content = fs::read_to_string(input_path)?;
        let output = self.assemble_string(&content)?;
        fs::write(output_path, output)?;
        Ok(())
    }

    pub fn assemble_string(&mut self, input: &str) -> Result<&[u8; 0x8000], AssembleError> {
        let mut parser = Parser::new(input);
        let mut program = parser.parse()?;

        self.first_pass(&mut program)?;
        self.second_pass(&program)?;

        Ok(&self.output)
    }

    fn first_pass(&mut self, program: &mut Program) -> Result<(), AssembleError> {
        let mut address = self.org_address; // Start with the initial org address

        for statement in &program.statements {
            match statement {
                Statement::Label(name) => {
                    if program.labels.contains_key(name) {
                        return Err(AssembleError::GenericError(format!(
                            "Duplicate label definition: {}",
                            name
                        )));
                    }
                    program.labels.insert(name.clone(), address);
                }
                Statement::Directive(name, value) => match name.to_lowercase().as_str() {
                    "org" => match value {
                        Operand::Literal(lit) => {
                            address = *lit;
                        }
                        _ => {}
                    },
                    "word" => match value {
                        Operand::LabelRef(_) | Operand::Literal(_) => {
                            address += 2;
                        }
                        _ => {}
                    },
                    _ => {}
                },
                Statement::Instruction(instruction) => {
                    // Pass program.aliases, program.labels is not fully populated yet for forward refs,
                    // but get_instruction_size should handle LabelRef as a known size type.
                    address += self.get_instruction_size(instruction, &program.aliases)?;
                }
                _ => {}
            }
        }
        // self.current_address = self.org_address; // Reset for the second pass start
        Ok(())
    }

    fn second_pass(&mut self, program: &Program) -> Result<(), AssembleError> {
        for statement in &program.statements {
            match statement {
                Statement::Instruction(instruction) => {
                    self.generate_instruction(instruction, &program.aliases, &program.labels)?;
                }
                Statement::Directive(name, value) => {
                    match name.to_lowercase().as_str() {
                        "org" => match value {
                            Operand::Literal(lit) => {
                                self.current_address = *lit;
                            }
                            _ => {}
                        },
                        "word" => match value {
                            Operand::Literal(lit) => {
                                self.emit_u16(*lit);
                            }
                            Operand::LabelRef(label) => {
                                if let Some(addr) = program.labels.get(label) {
                                    self.emit_u16(*addr);
                                }
                            }
                            _ => {}
                        },
                        _ => {}
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }

    // Helper to resolve operand type for size calculation in the first pass
    fn resolve_operand_for_size(
        &self,
        operand: &Operand,
        aliases: &IndexMap<String, Operand>,
        depth: usize,
    ) -> Result<ResolvedOperandType, AssembleError> {
        const MAX_ALIAS_DEPTH: usize = 10;
        if depth > MAX_ALIAS_DEPTH {
            return Err(AssembleError::GenericError(
                "Alias resolution depth exceeded for size calculation".to_string(),
            ));
        }

        match operand {
            Operand::Register(_) | Operand::RegisterIndirect(_) | Operand::Acc => {
                Ok(ResolvedOperandType::RegisterLike)
            }
            Operand::Literal(_) | Operand::LabelRef(_) => Ok(ResolvedOperandType::LiteralLike),
            Operand::Alias(name) => {
                let resolved = aliases.get(name).ok_or_else(|| {
                    AssembleError::GenericError(format!(
                        "Unknown alias for size calculation: '{}'",
                        name
                    ))
                })?;
                self.resolve_operand_for_size(resolved, aliases, depth + 1)
            }
        }
    }

    fn get_instruction_size(
        &self,
        instruction: &Instruction,
        aliases: &IndexMap<String, Operand>,
    ) -> Result<u16, AssembleError> {
        match instruction {
            Instruction::Nop | Instruction::Hlt | Instruction::Ret | Instruction::Rti | Instruction::Cli | Instruction::Sei => Ok(1),
            Instruction::Inc(op) | Instruction::Dec(op) | Instruction::Not(op) => {
                                match self.resolve_operand_for_size(op, aliases, 0)? {
                                    ResolvedOperandType::RegisterLike => Ok(1 + 1), // Opcode + Reg
                                    _ => Err(AssembleError::GenericError(format!(
                                        "Invalid operand for INC/DEC/NOT: {:?}. Must be register-like.",
                                        op
                                    ))),
                                }
                            }
            Instruction::Mov(dest, src) => {
                                let resolved_dest_type = self.resolve_operand_for_size(dest, aliases, 0)?;
                                let resolved_src_type = self.resolve_operand_for_size(src, aliases, 0)?;

                                match (resolved_dest_type, resolved_src_type) {
                                    (ResolvedOperandType::RegisterLike, ResolvedOperandType::RegisterLike) => {
                                        Ok(1 + 1 + 1)
                                    } // MOV Reg, Reg (0x10)
                                    (ResolvedOperandType::RegisterLike, ResolvedOperandType::LiteralLike) => {
                                        Ok(1 + 1 + 2)
                                    } // MOV Reg, Lit (0x11) / MOV Reg, Mem (0x12) - casm uses 0x11
                                    (ResolvedOperandType::LiteralLike, ResolvedOperandType::RegisterLike) => {
                                        Ok(1 + 2 + 1)
                                    } // MOV Mem, Reg (0x14)
                                    (ResolvedOperandType::LiteralLike, ResolvedOperandType::LiteralLike) => {
                                        Ok(1 + 2 + 2)
                                    }
                                }
                            }
            Instruction::Add(op1, op2)
                            | Instruction::Mul(op1, op2)
                            | Instruction::Cmp(op1, op2) => {
                                let type1 = self.resolve_operand_for_size(op1, aliases, 0)?;
                                let type2 = self.resolve_operand_for_size(op2, aliases, 0)?;
                                match (type1, type2) {
                                    (ResolvedOperandType::RegisterLike, ResolvedOperandType::RegisterLike) => {
                                        Ok(1 + 1 + 1)
                                    }
                                    (ResolvedOperandType::RegisterLike, ResolvedOperandType::LiteralLike) => {
                                        Ok(1 + 1 + 2)
                                    }
                                    _ => Err(AssembleError::GenericError(format!(
                                        "Invalid operands for ADD/MUL/CMP: {:?}, {:?}",
                                        op1, op2
                                    ))),
                                }
                            }
            Instruction::Sub(op1, op2)
                            | Instruction::Div(op1, op2)
                            | Instruction::Mod(op1, op2) => {
                                let type1 = self.resolve_operand_for_size(op1, aliases, 0)?;
                                let type2 = self.resolve_operand_for_size(op2, aliases, 0)?;
                                match (type1, type2) {
                                    (ResolvedOperandType::RegisterLike, ResolvedOperandType::RegisterLike) => {
                                        Ok(1 + 1 + 1)
                                    }
                                    (ResolvedOperandType::RegisterLike, ResolvedOperandType::LiteralLike) => {
                                        Ok(1 + 1 + 2)
                                    }
                                    (ResolvedOperandType::LiteralLike, ResolvedOperandType::RegisterLike) => {
                                        Ok(1 + 2 + 1)
                                    }
                                    _ => Err(AssembleError::GenericError(format!(
                                        "Invalid operands for SUB/DIV/MOD: {:?}, {:?}",
                                        op1, op2
                                    ))),
                                }
                            }
            Instruction::And(op1, op2) | Instruction::Or(op1, op2) | Instruction::Xor(op1, op2) => {
                                let type1 = self.resolve_operand_for_size(op1, aliases, 0)?;
                                let type2 = self.resolve_operand_for_size(op2, aliases, 0)?;
                                match (type1, type2) {
                                    (ResolvedOperandType::RegisterLike, ResolvedOperandType::RegisterLike) => {
                                        Ok(1 + 1 + 1)
                                    }
                                    _ => Err(AssembleError::GenericError(format!(
                                        "AND/OR/XOR operands must be register-like: {:?}, {:?}",
                                        op1, op2
                                    ))),
                                }
                            }
            Instruction::Jmp(op)
                            | Instruction::Jz(op)
                            | Instruction::Jnz(op)
                            | Instruction::Jn(op)
                            | Instruction::Jnn(op)
                            | Instruction::Jc(op)
                            | Instruction::Jnc(op) => {
                                match self.resolve_operand_for_size(op, aliases, 0)? {
                                    ResolvedOperandType::LiteralLike => Ok(1 + 2), // Opcode + Addr
                                    ResolvedOperandType::RegisterLike => Ok(1 + 1), // Opcode + Reg
                                }
                            }
            Instruction::Call(op) => {
                                // CALL Lit (0x60)
                                match self.resolve_operand_for_size(op, aliases, 0)? {
                                    ResolvedOperandType::LiteralLike => Ok(1 + 2), // Opcode + Addr
                                    _ => Err(AssembleError::GenericError(format!(
                                        "CALL operand must be literal-like: {:?}",
                                        op
                                    ))),
                                }
                            }
            Instruction::Phr(operand) => {
                        match self.resolve_operand_for_size(operand, aliases, 0)? {
                            ResolvedOperandType::RegisterLike => Ok(1 + 1), // Opcode + Reg
                            _ => Err(AssembleError::GenericError(format!(
                                "PHR operand must be register-like: {:?}",
                                operand
                            ))),
                        }
                    }
            Instruction::Plr(operand) => {
                        match self.resolve_operand_for_size(operand, aliases, 0)? {
                            ResolvedOperandType::RegisterLike => Ok(1 + 1), // Opcode + Reg
                            _ => Err(AssembleError::GenericError(format!(
                                "PLR operand must be register-like: {:?}",
                                operand
                            ))),
                        }
                    },
        }
    }

    // Not used by get_instruction_size, but was in user's template
    // fn get_operand_size(&self, operand: &Operand, aliases: &IndexMap<String, Operand>) -> Result<u16, AssembleError> { ... }

    fn emit_byte(&mut self, byte: u8) {
        self.output[self.current_address as usize] = byte;
        self.current_address += 1;
    }

    fn emit_u16(&mut self, value: u16) {
        self.emit_byte((value & 0xFF) as u8); // Little-endian: low byte first
        self.emit_byte(((value >> 8) & 0xFF) as u8); // Little-endian: high byte second
    }

    fn resolve_operand_fully(
        &self,
        operand: &Operand,
        aliases: &IndexMap<String, Operand>,
        labels: &IndexMap<String, u16>,
        depth: usize,
    ) -> Result<Operand, AssembleError> {
        const MAX_ALIAS_DEPTH: usize = 10;
        if depth > MAX_ALIAS_DEPTH {
            return Err(AssembleError::GenericError(
                "Alias resolution depth exceeded".to_string(),
            ));
        }

        match operand {
            Operand::Alias(name) => {
                let resolved_alias = aliases.get(name).ok_or_else(|| {
                    AssembleError::GenericError(format!("Unknown alias: {}", name))
                })?;
                // Recursively resolve if the alias points to another alias or a label reference
                self.resolve_operand_fully(resolved_alias, aliases, labels, depth + 1)
            }
            Operand::LabelRef(name) => labels
                .get(name)
                .map(|addr| Operand::Literal(*addr))
                .ok_or_else(|| AssembleError::GenericError(format!("Unknown label: {}", name))),
            _ => Ok(operand.clone()), // Register, RegisterIndirect, Literal are base types
        }
    }

    fn emit_operand_reg(
        &mut self,
        operand: &Operand,
        aliases: &IndexMap<String, Operand>,
        labels: &IndexMap<String, u16>,
    ) -> Result<(), AssembleError> {
        let resolved = self.resolve_operand_fully(operand, aliases, labels, 0)?;
        match resolved {
            Operand::Register(r) | Operand::RegisterIndirect(r) => {
                if r > 15 || r < 1 {
                    // Assuming 16 registers R0-R15
                    return Err(AssembleError::GenericError(format!(
                        "Invalid register identifier: R{}",
                        r
                    )));
                }
                self.emit_byte(r);
                Ok(())
            }
            Operand::Acc => {
                self.emit_byte(0x00);
                Ok(())
            }
            _ => Err(AssembleError::GenericError(format!(
                "Expected register operand, found {:?} (resolved from {:?})",
                resolved, operand
            ))),
        }
    }

    fn emit_operand_literal(
        &mut self,
        operand: &Operand,
        aliases: &IndexMap<String, Operand>,
        labels: &IndexMap<String, u16>,
    ) -> Result<(), AssembleError> {
        let resolved = self.resolve_operand_fully(operand, aliases, labels, 0)?;
        match resolved {
            Operand::Literal(val) => {
                self.emit_u16(val);
                Ok(())
            }
            _ => Err(AssembleError::GenericError(format!(
                "Expected literal operand, found {:?} (resolved from {:?})",
                resolved, operand
            ))),
        }
    }

    fn generate_instruction(
        &mut self,
        instruction: &Instruction,
        aliases: &IndexMap<String, Operand>,
        labels: &IndexMap<String, u16>,
    ) -> Result<(), AssembleError> {
        match instruction {
            Instruction::Nop => self.emit_byte(0x00),
            Instruction::Hlt => self.emit_byte(0x01),
            Instruction::Mov(dest, src) => self.generate_mov(dest, src, aliases, labels)?,
            Instruction::Add(op1, op2) => {
                        self.generate_binary_arithmetic(0x20, 0x21, op1, op2, aliases, labels)?
                    }
            Instruction::Sub(op1, op2) => self.generate_sub(op1, op2, aliases, labels)?,
            Instruction::Mul(op1, op2) => {
                        self.generate_binary_arithmetic(0x25, 0x26, op1, op2, aliases, labels)?
                    }
            Instruction::Div(op1, op2) => self.generate_div(op1, op2, aliases, labels)?,
            Instruction::Mod(op1, op2) => self.generate_mod(op1, op2, aliases, labels)?,
            Instruction::Inc(op) => {
                        self.emit_byte(0x2D);
                        self.emit_operand_reg(op, aliases, labels)?;
                    }
            Instruction::Dec(op) => {
                        self.emit_byte(0x2E);
                        self.emit_operand_reg(op, aliases, labels)?;
                    }
            Instruction::And(op1, op2) => {
                        // Opcode 0x30 (Reg Reg)
                        self.emit_byte(0x30);
                        self.emit_operand_reg(op1, aliases, labels)?;
                        self.emit_operand_reg(op2, aliases, labels)?;
                    }
            Instruction::Or(op1, op2) => {
                        // Opcode 0x31 (Reg Reg)
                        self.emit_byte(0x31);
                        self.emit_operand_reg(op1, aliases, labels)?;
                        self.emit_operand_reg(op2, aliases, labels)?;
                    }
            Instruction::Xor(op1, op2) => {
                        // Opcode 0x32 (Reg Reg)
                        self.emit_byte(0x32);
                        self.emit_operand_reg(op1, aliases, labels)?;
                        self.emit_operand_reg(op2, aliases, labels)?;
                    }
            Instruction::Not(op) => {
                        // Opcode 0x33 (Reg)
                        self.emit_byte(0x33);
                        self.emit_operand_reg(op, aliases, labels)?;
                    }
            Instruction::Cmp(op1, op2) => self.generate_cmp(op1, op2, aliases, labels)?,
            Instruction::Jmp(op) => self.generate_jump(0x50, 0x51, op, aliases, labels)?,
            Instruction::Jz(op) => self.generate_jump(0x52, 0x53, op, aliases, labels)?,
            Instruction::Jnz(op) => self.generate_jump(0x54, 0x55, op, aliases, labels)?,
            Instruction::Jn(op) => self.generate_jump(0x56, 0x57, op, aliases, labels)?,
            Instruction::Jnn(op) => self.generate_jump(0x58, 0x59, op, aliases, labels)?,
            Instruction::Jc(op) => self.generate_jump(0x5A, 0x5B, op, aliases, labels)?,
            Instruction::Jnc(op) => self.generate_jump(0x5C, 0x5D, op, aliases, labels)?,
            Instruction::Call(op) => {
                        // Opcode 0x60 (Lit)
                        self.emit_byte(0x60);
                        self.emit_operand_literal(op, aliases, labels)?;
                    }
            Instruction::Ret => self.emit_byte(0x61),
            Instruction::Rti => self.emit_byte(0x62),
            Instruction::Cli => self.emit_byte(0x70),
            Instruction::Sei => self.emit_byte(0x70),
            Instruction::Phr(operand) => {
                self.emit_byte(0x18);
                self.emit_operand_reg(operand, aliases, labels)?;
            },
            Instruction::Plr(operand) => {
                self.emit_byte(0x19);
                self.emit_operand_reg(operand, aliases, labels)?;
            },
        }
        Ok(())
    }

    fn generate_mov(
        &mut self,
        dest: &Operand,
        src: &Operand,
        aliases: &IndexMap<String, Operand>,
        labels: &IndexMap<String, u16>,
    ) -> Result<(), AssembleError> {
        let resolved_dest = self.resolve_operand_fully(dest, aliases, labels, 0)?;
        let resolved_src = self.resolve_operand_fully(src, aliases, labels, 0)?;

        match (&resolved_dest, &resolved_src) {
            (Operand::Register(_) | Operand::Acc, Operand::Register(_) | Operand::Acc) => {
                // MOV Reg, Reg
                self.emit_byte(0x10);
                self.emit_operand_reg(&resolved_dest, aliases, labels)?;
                self.emit_operand_reg(&resolved_src, aliases, labels)?;
            }
            (Operand::Register(_) | Operand::Acc, Operand::Literal(_)) => {
                // MOV Reg, Lit (covers resolved LabelRef and Alias to Literal)
                self.emit_byte(0x11); // This is MOV Reg, LiteralValue
                self.emit_operand_reg(&resolved_dest, aliases, labels)?;
                self.emit_operand_literal(&resolved_src, aliases, labels)?;
            }
            (Operand::Register(_) | Operand::Acc, Operand::RegisterIndirect(_)) => {
                // MOV Reg, Reg*
                self.emit_byte(0x13);
                self.emit_operand_reg(&resolved_dest, aliases, labels)?;
                self.emit_operand_reg(&resolved_src, aliases, labels)?; // Emits the register number part of Reg*
            }
            (Operand::Literal(_), Operand::Register(_) | Operand::Acc) => {
                // MOV Mem, Reg (where Mem is a literal address)
                self.emit_byte(0x14);
                self.emit_operand_literal(&resolved_dest, aliases, labels)?; // The memory address
                self.emit_operand_reg(&resolved_src, aliases, labels)?; // The source register
            }
            (Operand::Literal(_), Operand::Literal(_)) => {
                // MOV Mem, Lit
                self.emit_byte(0x15);
                self.emit_operand_literal(&resolved_dest, aliases, labels)?;
                self.emit_operand_literal(&resolved_src, aliases, labels)?;
            }
            (Operand::RegisterIndirect(_), Operand::Register(_) | Operand::Acc) => {
                // MOV Reg*, Reg
                self.emit_byte(0x16);
                self.emit_operand_reg(&resolved_dest, aliases, labels)?; // Emits the register number part of Reg*
                self.emit_operand_reg(&resolved_src, aliases, labels)?;
            }
            (Operand::RegisterIndirect(_), Operand::Literal(_)) => {
                // MOV Reg*, Lit
                self.emit_byte(0x17);
                self.emit_operand_reg(&resolved_dest, aliases, labels)?;
                self.emit_operand_literal(&resolved_src, aliases, labels)?;
            }
            _ => {
                return Err(AssembleError::GenericError(format!(
                "Invalid MOV operand combination: dest={:?}, src={:?} (Original: D={:?}, S={:?})",
                resolved_dest, resolved_src, dest, src
            )))
            }
        }
        Ok(())
    }

    fn generate_binary_arithmetic(
        &mut self,
        reg_reg_opcode: u8,
        reg_lit_opcode: u8,
        op1: &Operand,
        op2: &Operand,
        aliases: &IndexMap<String, Operand>,
        labels: &IndexMap<String, u16>,
    ) -> Result<(), AssembleError> {
        let resolved_op1 = self.resolve_operand_fully(op1, aliases, labels, 0)?;
        let resolved_op2 = self.resolve_operand_fully(op2, aliases, labels, 0)?;

        match (&resolved_op1, &resolved_op2) {
            (Operand::Register(r1_val), Operand::Register(r2_val)) => {
                self.emit_byte(reg_reg_opcode);
                self.emit_byte(*r1_val); // No need to call emit_operand_reg, already resolved
                self.emit_byte(*r2_val);
            }
            (Operand::Register(r1_val), Operand::Literal(l2_val)) => {
                self.emit_byte(reg_lit_opcode);
                self.emit_byte(*r1_val);
                self.emit_u16(*l2_val);
            }
            _ => {
                return Err(AssembleError::GenericError(format!(
                    "Invalid operand combination for binary arithmetic (e.g., ADD, MUL): {:?}, {:?} (Original: Op1={:?}, Op2={:?})",
                    resolved_op1, resolved_op2, op1, op2
                )));
            }
        }
        Ok(())
    }

    fn generate_sub(
        &mut self,
        op1: &Operand,
        op2: &Operand,
        aliases: &IndexMap<String, Operand>,
        labels: &IndexMap<String, u16>,
    ) -> Result<(), AssembleError> {
        let resolved_op1 = self.resolve_operand_fully(op1, aliases, labels, 0)?;
        let resolved_op2 = self.resolve_operand_fully(op2, aliases, labels, 0)?;

        match (&resolved_op1, &resolved_op2) {
            (Operand::Register(r1), Operand::Register(r2)) => {
                self.emit_byte(0x22); // SUB Reg Reg
                self.emit_byte(*r1);
                self.emit_byte(*r2);
            }
            (Operand::Register(r1), Operand::Literal(l2)) => {
                self.emit_byte(0x23); // SUB Reg Lit
                self.emit_byte(*r1);
                self.emit_u16(*l2);
            }
            (Operand::Literal(l1), Operand::Register(r2)) => {
                self.emit_byte(0x24); // SUB Lit Reg
                self.emit_u16(*l1);
                self.emit_byte(*r2);
            }
            _ => {
                return Err(AssembleError::GenericError(format!(
                    "Invalid operand combination for SUB: {:?}, {:?} (Original: Op1={:?}, Op2={:?})",
                    resolved_op1, resolved_op2, op1, op2
                )));
            }
        }
        Ok(())
    }

    fn generate_div(
        &mut self,
        op1: &Operand,
        op2: &Operand,
        aliases: &IndexMap<String, Operand>,
        labels: &IndexMap<String, u16>,
    ) -> Result<(), AssembleError> {
        let resolved_op1 = self.resolve_operand_fully(op1, aliases, labels, 0)?;
        let resolved_op2 = self.resolve_operand_fully(op2, aliases, labels, 0)?;

        match (&resolved_op1, &resolved_op2) {
            (Operand::Register(r1), Operand::Register(r2)) => {
                self.emit_byte(0x27); // DIV Reg Reg
                self.emit_byte(*r1);
                self.emit_byte(*r2);
            }
            (Operand::Register(r1), Operand::Literal(l2)) => {
                self.emit_byte(0x28); // DIV Reg Lit
                self.emit_byte(*r1);
                self.emit_u16(*l2);
            }
            (Operand::Literal(l1), Operand::Register(r2)) => {
                self.emit_byte(0x29); // DIV Lit Reg
                self.emit_u16(*l1);
                self.emit_byte(*r2);
            }
            _ => {
                return Err(AssembleError::GenericError(format!(
                    "Invalid operand combination for DIV: {:?}, {:?} (Original: Op1={:?}, Op2={:?})",
                    resolved_op1, resolved_op2, op1, op2
                )));
            }
        }
        Ok(())
    }

    fn generate_mod(
        &mut self,
        op1: &Operand,
        op2: &Operand,
        aliases: &IndexMap<String, Operand>,
        labels: &IndexMap<String, u16>,
    ) -> Result<(), AssembleError> {
        let resolved_op1 = self.resolve_operand_fully(op1, aliases, labels, 0)?;
        let resolved_op2 = self.resolve_operand_fully(op2, aliases, labels, 0)?;

        match (&resolved_op1, &resolved_op2) {
            (Operand::Register(r1), Operand::Register(r2)) => {
                self.emit_byte(0x2A); // MOD Reg Reg
                self.emit_byte(*r1);
                self.emit_byte(*r2);
            }
            (Operand::Register(r1), Operand::Literal(l2)) => {
                self.emit_byte(0x2B); // MOD Reg Lit
                self.emit_byte(*r1);
                self.emit_u16(*l2);
            }
            (Operand::Literal(l1), Operand::Register(r2)) => {
                self.emit_byte(0x2C); // MOD Lit Reg
                self.emit_u16(*l1);
                self.emit_byte(*r2);
            }
            _ => {
                return Err(AssembleError::GenericError(format!(
                    "Invalid operand combination for MOD: {:?}, {:?} (Original: Op1={:?}, Op2={:?})",
                    resolved_op1, resolved_op2, op1, op2
                )));
            }
        }
        Ok(())
    }

    fn generate_cmp(
        &mut self,
        op1: &Operand,
        op2: &Operand,
        aliases: &IndexMap<String, Operand>,
        labels: &IndexMap<String, u16>,
    ) -> Result<(), AssembleError> {
        let resolved_op1 = self.resolve_operand_fully(op1, aliases, labels, 0)?;
        let resolved_op2 = self.resolve_operand_fully(op2, aliases, labels, 0)?;

        match (&resolved_op1, &resolved_op2) {
            (Operand::Register(r1), Operand::Register(r2)) => {
                self.emit_byte(0x40); // CMP Reg Reg
                self.emit_byte(*r1);
                self.emit_byte(*r2);
            }
            (Operand::Register(r1), Operand::Literal(l2)) => {
                self.emit_byte(0x41); // CMP Reg Lit
                self.emit_byte(*r1);
                self.emit_u16(*l2);
            }
            _ => {
                return Err(AssembleError::GenericError(format!(
                    "Invalid operand combination for CMP: {:?}, {:?} (Original: Op1={:?}, Op2={:?})",
                    resolved_op1, resolved_op2, op1, op2
                )));
            }
        }
        Ok(())
    }

    fn generate_jump(
        &mut self,
        lit_opcode: u8,
        reg_opcode: u8,
        op: &Operand,
        aliases: &IndexMap<String, Operand>,
        labels: &IndexMap<String, u16>,
    ) -> Result<(), AssembleError> {
        let resolved_op = self.resolve_operand_fully(op, aliases, labels, 0)?;

        match resolved_op {
            Operand::Literal(addr) => {
                // This includes resolved LabelRefs and Aliases to Literals/Labels
                self.emit_byte(lit_opcode);
                self.emit_u16(addr);
            }
            Operand::Register(reg_idx) => {
                if reg_idx >= 16 {
                    return Err(AssembleError::GenericError(format!(
                        "Invalid register R{} for jump pointer",
                        reg_idx
                    )));
                }
                self.emit_byte(reg_opcode);
                self.emit_byte(reg_idx);
            }
            Operand::RegisterIndirect(_) => {
                return Err(AssembleError::GenericError(format!(
                   "Register indirect (e.g., R0*) is not a valid jump target. Original operand: {:?}", op
                )));
            }
            _ => {
                // Should not happen if resolve_operand_fully works correctly
                return Err(AssembleError::GenericError(format!(
                    "Invalid resolved operand for jump: {:?} (Original: {:?})",
                    resolved_op, op
                )));
            }
        }
        Ok(())
    }
}
