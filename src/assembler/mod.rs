pub mod lexer;
pub mod parser;

use std::collections::HashMap;

use lexer::{CupanaLexer, Token};
use parser::{CupanaParser, Instruction, OneArgType, OperationType, TwoArgsType};

use crate::assembler::lexer::{DirectiveType, MnemonicType, RegisterType};
use crate::assembler::parser::{is_conditional_jump, ConditionalJumpType};
use crate::memory::ROM_SIZE;

pub struct CupanaAssembler {
    instructions: Vec<Instruction>,
    labels: HashMap<String, u16>,
    curr_addr: u16,
    output: [u8; ROM_SIZE],
}

impl CupanaAssembler {
    pub fn new(path: &str) -> Self {
        let mut lexer = CupanaLexer::new(path);
        let tokens = lexer.lex().to_vec();
        // dbg!(&tokens);
        let mut parser = CupanaParser::new(tokens);
        let instructions = parser.parse().to_vec();

        Self {
            instructions,
            labels: HashMap::new(),
            curr_addr: 0,
            output: [0; ROM_SIZE],
        }
    }

    pub fn assemble(&mut self) -> &[u8] {
        self.resolve_labels();
        self.gen_bytes()
    }

    fn push_u8(&mut self, value: u8) {
        if self.curr_addr >= ROM_SIZE as u16 {
            println!("Tamanho da rom excedido");
            return;
        }
        self.output[self.curr_addr as usize] = value;
        self.curr_addr += 1;
    }

    fn push_u16(&mut self, value: u16) {
        let high = (value & 0xff) as u8;
        let low = (value >> 8) as u8;
        self.push_u8(high);
        self.push_u8(low);
    }

    fn get_output(&self) -> &[u8] {
        &self.output
    }

    fn resolve_labels(&mut self) {
        let mut iter = self.instructions.iter();

        while let Some(inst) = iter.next() {
            match inst {
                Instruction::LabelDef(name) => {
                    self.labels.insert(name.to_string(), self.curr_addr);
                }
                Instruction::Operation(op) => {
                    self.curr_addr += self.instruction_size(op);
                }
                Instruction::Diretive(dir_type, tok) => match dir_type {
                    DirectiveType::SHORT => self.curr_addr += 2,
                    DirectiveType::BYTE => self.curr_addr += 1,
                    DirectiveType::ORG => match tok {
                        Token::Literal(value) => self.curr_addr = *value,
                        _ => unreachable!(),
                    },
                    _ => unreachable!(),
                },
                _ => {}
            }
        }

        self.curr_addr = 0;
    }

    fn gen_bytes(&mut self) -> &[u8] {
        for inst in self.instructions.clone() {
            println!("{:?}", inst);
            match inst {
                Instruction::Operation(op) => match op {
                    OperationType::NoArgs(mnem) => self.push_u8(mnem.to_byte() << 3),
                    OperationType::OneArg(op_type) => match op_type {
                        OneArgType::Reg(mnem, Token::Register(RegisterType::Direct(reg))) => {
                            self.push_u8(mnem.to_byte() << 3);
                            self.handle_conditional(mnem);
                            self.push_u8(reg);
                        }
                        OneArgType::RegB(mnem, Token::Register(RegisterType::Direct(reg))) => {
                            self.push_u8((mnem.to_byte() << 3) | 0x04);
                            self.push_u8(reg);
                        }
                        OneArgType::RegIndirect(
                            mnem,
                            Token::Register(RegisterType::Indirect(reg)),
                        ) => {
                            self.push_u8((mnem.to_byte() << 3) | 0x02);
                            self.push_u8(reg);
                        }
                        OneArgType::Lit(mnem, Token::Literal(value)) => {
                            self.push_u8((mnem.to_byte() << 3) | 0x01);
                            self.handle_conditional(mnem);
                            self.push_u16(value);
                        }
                        OneArgType::Label(mnem, Token::Label(name)) => {
                            let label_value = self.labels.get(&name).copied();
                            if let Some(value) = label_value {
                                self.push_u8((mnem.to_byte() << 3) | 0x01);
                                self.handle_conditional(mnem);
                                self.push_u16(value);
                            }
                        }
                        _ => unreachable!(),
                    },
                    OperationType::TwoArgs(op_type) => match op_type {
                        TwoArgsType::RegReg(
                            mnem,
                            Token::Register(RegisterType::Direct(dest)),
                            Token::Register(RegisterType::Direct(orig)),
                        ) => {
                            println!("dest {} orig {} byte {}", dest, orig, (dest << 4) & orig);
                            self.push_u8(mnem.to_byte() << 3);
                            self.push_u8((dest << 4) | orig);
                        }
                        TwoArgsType::RegLit(
                            mnem,
                            Token::Register(RegisterType::Direct(dest)),
                            Token::Literal(value),
                        ) => {
                            self.push_u8((mnem.to_byte() << 3) | 0b001);
                            self.push_u8(dest);
                            self.push_u16(value);
                        }
                        TwoArgsType::RegIReg(
                            mnem,
                            Token::Register(RegisterType::Direct(dest)),
                            Token::Register(RegisterType::Direct(orig)),
                        ) => {
                            self.push_u8((mnem.to_byte() << 3) | 0b010);
                            self.push_u8((dest << 4) | orig);
                        }
                        TwoArgsType::RegRegI(
                            mnem,
                            Token::Register(RegisterType::Direct(dest)),
                            Token::Register(RegisterType::Direct(orig)),
                        ) => {
                            self.push_u8((mnem.to_byte() << 3) | 0b011);
                            self.push_u8((dest << 4) | orig);
                        }
                        TwoArgsType::RegBRegB(
                            mnem,
                            Token::Register(RegisterType::Direct(dest)),
                            Token::Register(RegisterType::Direct(orig)),
                        ) => {
                            self.push_u8((mnem.to_byte() << 3) | 0b100);
                            self.push_u8((dest << 4) | orig);
                        }
                        TwoArgsType::RegLitB(
                            mnem,
                            Token::Register(RegisterType::Direct(dest)),
                            Token::Literal(value),
                        ) => {
                            self.push_u8((mnem.to_byte() << 3) | 0b101);
                            self.push_u8(dest);
                            self.push_u8(value as u8);
                        }
                        TwoArgsType::RegIRegB(
                            mnem,
                            Token::Register(RegisterType::Direct(dest)),
                            Token::Register(RegisterType::Direct(orig)),
                        ) => {
                            self.push_u8((mnem.to_byte() << 3) | 0b110);
                            self.push_u8((dest << 4) | orig);
                        }
                        _ => unreachable!(),
                    },
                },
                Instruction::Diretive(dir_type, Token::Literal(value)) => match dir_type {
                    DirectiveType::ORG => {
                        self.curr_addr = value;
                    }
                    DirectiveType::SHORT => {
                        self.push_u16(value);
                    }
                    DirectiveType::BYTE => {
                        self.push_u8(value as u8);
                    }

                    _ => unreachable!(),
                },
                Instruction::LabelDef(_) => {}
                _ => unreachable!(),
            }
        }

        self.get_output()
    }

    fn instruction_size(&self, op: &OperationType) -> u16 {
        match op {
            OperationType::NoArgs(_) => 1, // 8 bits
            OperationType::OneArg(op_type) => match op_type {
                OneArgType::Reg(_, _) | OneArgType::RegIndirect(_, _) | OneArgType::RegB(_, _) => 2, // Opcode + Reg
                OneArgType::Lit(_, _) | OneArgType::Label(_, _) => 3, // Opcode (1 byte, JMP/Call) + 2 bytes lit
            },
            OperationType::TwoArgs(op_type) => match op_type {
                TwoArgsType::RegReg(_, _, _)
                | TwoArgsType::RegIReg(_, _, _)
                | TwoArgsType::RegRegI(_, _, _)
                | TwoArgsType::RegBRegB(_, _, _)
                | TwoArgsType::RegIRegB(_, _, _) => 2,
                TwoArgsType::RegLitB(_, _, _) => 3,
                TwoArgsType::RegLit(_, _, _) => 4,
            },
        }
    }

    fn handle_conditional(&mut self, mnem: MnemonicType) {
        if is_conditional_jump(&mnem) {
            match mnem {
                MnemonicType::JZ => self.push_u8(0),
                MnemonicType::JNZ => self.push_u8(1),
                MnemonicType::JN => self.push_u8(2),
                MnemonicType::JNN => self.push_u8(3),
                MnemonicType::JO => self.push_u8(4),
                MnemonicType::JNO => self.push_u8(5),
                _ => unreachable!(),
            }
        }
    }
}
