use crate::assembler::parser::{Instruction, Operand, Program, Statement, Parser};
use crate::error::AssembleError;
use indexmap::IndexMap;
use std::fs;
use std::path::Path;

pub struct Assembler {
    program: Option<Program>,
    output: Vec<u8>,
    current_address: u16,
    org_address: u16,
}

impl Assembler {
    pub fn new() -> Self {
        Self {
            program: None,
            output: Vec::new(),
            current_address: 0,
            org_address: 0,
        }
    }

    pub fn assemble_file<P: AsRef<Path>>(&mut self, input_path: P) -> Result<Vec<u8>, AssembleError> {
        let content = fs::read_to_string(input_path)?;
        self.assemble_string(&content)
    }

    pub fn assemble_string(&mut self, input: &str) -> Result<Vec<u8>, AssembleError> {
        // Parse
        let mut parser = Parser::new(input);
        let mut program = parser.parse()?;

        // First pass: collect labels and resolve addresses
        self.first_pass(&mut program)?;

        // Second pass: generate code
        self.second_pass(&program)?;

        Ok(self.output.clone())
    }

    fn first_pass(&mut self, program: &mut Program) -> Result<(), AssembleError> {
        let mut address = self.org_address;

        for statement in &program.statements {
            match statement {
                Statement::Label(name) => {
                    program.labels.insert(name.clone(), address);
                }
                Statement::Directive(name, value) => {
                    if name.to_lowercase() == "org" {
                        if let Some(addr) = value {
                            address = *addr;
                            self.org_address = *addr;
                        }
                    }
                }
                Statement::Instruction(instruction) => {
                    address += self.get_instruction_size(instruction, &program.aliases)?;
                }
                Statement::AliasDeclaration(_, _) => {
                    // Aliases don't take space
                }
            }
        }

        self.current_address = self.org_address;
        Ok(())
    }

    fn second_pass(&mut self, program: &Program) -> Result<(), AssembleError> {
        self.output.clear();
        self.current_address = self.org_address;

        // Fill output buffer to org address
        self.output.resize(self.org_address as usize, 0);

        for statement in &program.statements {
            match statement {
                Statement::Instruction(instruction) => {
                    self.generate_instruction(instruction, &program.aliases, &program.labels)?;
                }
                Statement::Directive(name, value) => {
                    if name.to_lowercase() == "org" {
                        if let Some(addr) = value {
                            self.current_address = *addr;
                            if self.output.len() < *addr as usize {
                                self.output.resize(*addr as usize, 0);
                            }
                        }
                    }
                }
                Statement::Label(_) | Statement::AliasDeclaration(_, _) => {
                    // Already handled in first pass
                }
            }
        }

        Ok(())
    }

    fn get_instruction_size(&self, instruction: &Instruction, aliases: &IndexMap<String, Operand>) -> Result<u16, AssembleError> {
        match instruction {
            Instruction::Nop | Instruction::Hlt | Instruction::Ret => Ok(1),
            Instruction::Inc(_) | Instruction::Dec(_) | Instruction::Not(_) => Ok(2),
            Instruction::Mov(dest, src) => {
                let size = 1 + self.get_operand_size(dest, aliases)? + self.get_operand_size(src, aliases)?;
                Ok(size)
            }
            Instruction::Add(_, _) | Instruction::Sub(_, _) | Instruction::Mul(_, _) | 
            Instruction::Div(_, _) | Instruction::Mod(_, _) | Instruction::And(_, _) | 
            Instruction::Or(_, _) | Instruction::Xor(_, _) | Instruction::Cmp(_, _) => {
                Ok(3) // Most are reg-reg (3 bytes) or reg-lit (4 bytes), but we need to check
            }
            Instruction::Jmp(_) | Instruction::Jz(_) | Instruction::Jnz(_) | 
            Instruction::Jn(_) | Instruction::Jnn(_) | Instruction::Jc(_) | 
            Instruction::Jnc(_) | Instruction::Call(_) => Ok(3), // Assuming literal address
        }
    }

    fn get_operand_size(&self, operand: &Operand, aliases: &IndexMap<String, Operand>) -> Result<u16, AssembleError> {
        match operand {
            Operand::Register(_) | Operand::RegisterIndirect(_) => Ok(1),
            Operand::Literal(_) | Operand::LabelRef(_) => Ok(2),
            Operand::Alias(name) => {
                if let Some(resolved) = aliases.get(name) {
                    self.get_operand_size(resolved, aliases)
                } else {
                    Err(AssembleError::GenericError(format!("Unknown alias: {}", name)))
                }
            }
        }
    }

    fn generate_instruction(&mut self, instruction: &Instruction, aliases: &IndexMap<String, Operand>, labels: &IndexMap<String, u16>) -> Result<(), AssembleError> {
        match instruction {
            Instruction::Nop => {
                self.emit_byte(0x00);
            }
            Instruction::Hlt => {
                self.emit_byte(0x01);
            }
            Instruction::Mov(dest, src) => {
                self.generate_mov(dest, src, aliases, labels)?;
            }
            Instruction::Add(op1, op2) => {
                self.generate_binary_arithmetic(0x20, 0x21, op1, op2, aliases, labels)?;
            }
            Instruction::Sub(op1, op2) => {
                self.generate_sub(op1, op2, aliases, labels)?;
            }
            Instruction::Mul(op1, op2) => {
                self.generate_binary_arithmetic(0x25, 0x26, op1, op2, aliases, labels)?;
            }
            Instruction::Div(op1, op2) => {
                self.generate_div(op1, op2, aliases, labels)?;
            }
            Instruction::Mod(op1, op2) => {
                self.generate_mod(op1, op2, aliases, labels)?;
            }
            Instruction::Inc(op) => {
                self.emit_byte(0x2D);
                self.emit_operand_reg(op, aliases)?;
            }
            Instruction::Dec(op) => {
                self.emit_byte(0x2E);
                self.emit_operand_reg(op, aliases)?;
            }
            Instruction::And(op1, op2) => {
                self.emit_byte(0x30);
                self.emit_operand_reg(op1, aliases)?;
                self.emit_operand_reg(op2, aliases)?;
            }
            Instruction::Or(op1, op2) => {
                self.emit_byte(0x31);
                self.emit_operand_reg(op1, aliases)?;
                self.emit_operand_reg(op2, aliases)?;
            }
            Instruction::Xor(op1, op2) => {
                self.emit_byte(0x32);
                self.emit_operand_reg(op1, aliases)?;
                self.emit_operand_reg(op2, aliases)?;
            }
            Instruction::Not(op) => {
                self.emit_byte(0x33);
                self.emit_operand_reg(op, aliases)?;
            }
            Instruction::Cmp(op1, op2) => {
                self.generate_cmp(op1, op2, aliases, labels)?;
            }
            Instruction::Jmp(op) => {
                self.generate_jump(0x50, 0x51, op, aliases, labels)?;
            }
            Instruction::Jz(op) => {
                self.generate_jump(0x52, 0x53, op, aliases, labels)?;
            }
            Instruction::Jnz(op) => {
                self.generate_jump(0x54, 0x55, op, aliases, labels)?;
            }
            Instruction::Jn(op) => {
                self.generate_jump(0x56, 0x57, op, aliases, labels)?;
            }
            Instruction::Jnn(op) => {
                self.generate_jump(0x58, 0x59, op, aliases, labels)?;
            }
            Instruction::Jc(op) => {
                self.generate_jump(0x5A, 0x5B, op, aliases, labels)?;
            }
            Instruction::Jnc(op) => {
                self.generate_jump(0x5C, 0x5D, op, aliases, labels)?;
            }
            Instruction::Call(op) => {
                self.emit_byte(0x60);
                self.emit_operand_literal(op, aliases, labels)?;
            }
            Instruction::Ret => {
                self.emit_byte(0x61);
            }
        }
        Ok(())
    }

    fn generate_mov(&mut self, dest: &Operand, src: &Operand, aliases: &IndexMap<String, Operand>, labels: &IndexMap<String, u16>) -> Result<(), AssembleError> {
        match (dest, src) {
            (Operand::Register(_), Operand::Register(_)) => {
                self.emit_byte(0x10);
                self.emit_operand_reg(dest, aliases)?;
                self.emit_operand_reg(src, aliases)?;
            }
            (Operand::Register(_), Operand::Literal(_)) | (Operand::Register(_), Operand::LabelRef(_)) | (Operand::Register(_), Operand::Alias(_)) => {
                self.emit_byte(0x11);
                self.emit_operand_reg(dest, aliases)?;
                self.emit_operand_literal(src, aliases, labels)?;
            }
            (Operand::Register(_), Operand::RegisterIndirect(_)) => {
                self.emit_byte(0x13);
                self.emit_operand_reg(dest, aliases)?;
                self.emit_operand_reg(src, aliases)?;
            }
            (Operand::RegisterIndirect(_), Operand::Register(_)) => {
                self.emit_byte(0x15);
                self.emit_operand_reg(dest, aliases)?;
                self.emit_operand_reg(src, aliases)?;
            }
            _ => return Err(AssembleError::GenericError("Invalid MOV operand combination".to_string())),
        }
        Ok(())
    }

    fn generate_binary_arithmetic(&mut self, reg_reg_opcode: u8, reg_lit_opcode: u