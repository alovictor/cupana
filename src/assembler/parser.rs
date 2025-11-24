use crate::assembler::lexer::{DirectiveType, MnemonicType, RegisterType};

use super::lexer::Token;

#[derive(Debug, PartialEq, Clone)]
pub enum Instruction {
    Operation(OperationType),
    ConditionalJump(ConditionalJumpType),
    Diretive(DirectiveType, Token),
    LabelDef(String),
    Label(String),
}

#[derive(Debug, PartialEq, Clone)]
pub enum OperationType {
    NoArgs(MnemonicType),
    OneArg(OneArgType),
    TwoArgs(TwoArgsType),
}

#[derive(Debug, PartialEq, Clone)]
pub enum ConditionalJumpType {
    Zero(Token),
    NotZero(Token),
    Negative(Token),
    NotNegative(Token),
    Overflow(Token),
    NotOverflow(Token),
}

#[derive(Debug, PartialEq, Clone)]
pub enum OneArgType {
    Reg(MnemonicType, Token),
    RegB(MnemonicType, Token),
    RegIndirect(MnemonicType, Token),
    Lit(MnemonicType, Token),
    Label(MnemonicType, Token),
}

#[derive(Debug, PartialEq, Clone)]
pub enum TwoArgsType {
    RegReg(MnemonicType, Token, Token),
    RegLit(MnemonicType, Token, Token),
    RegIReg(MnemonicType, Token, Token),
    RegRegI(MnemonicType, Token, Token),
    RegBRegB(MnemonicType, Token, Token),
    RegLitB(MnemonicType, Token, Token),
    RegIRegB(MnemonicType, Token, Token),
}

pub struct CupanaParser {
    input: Vec<Token>,
    idx: usize,
    output: Vec<Instruction>,
}

impl CupanaParser {
    pub fn new(input: Vec<Token>) -> Self {
        Self {
            input: input,
            idx: 0,
            output: vec![],
        }
    }

    fn get_output(&self) -> &Vec<Instruction> {
        &self.output
    }

    fn fetch(&mut self) -> Token {
        let token = self.input[self.idx].clone();
        self.idx += 1;
        token
    }

    pub fn parse(&mut self) -> &Vec<Instruction> {
        while self.idx < self.input.len() {
            let token = self.fetch();
            match token {
                Token::LabelDef(name) => self.output.push(Instruction::LabelDef(name)),
                Token::Directive(dir_type) => {
                    let tk = self.fetch();
                    self.output.push(Instruction::Diretive(dir_type, tk));
                }
                Token::Mnemonic(mnem) => {
                    if is_no_args(&mnem) {
                        self.output
                            .push(Instruction::Operation(OperationType::NoArgs(mnem)));
                    } else if is_one_arg(&mnem) {
                        let inst = self.parse_one_arg(mnem);
                        self.output
                            .push(Instruction::Operation(OperationType::OneArg(inst)));
                    } else {
                        let inst = self.parse_two_args(mnem);
                        self.output
                            .push(Instruction::Operation(OperationType::TwoArgs(inst)));
                    }
                }
                _ => unreachable!(),
            }
        }
        return self.get_output();
    }

    fn parse_one_arg(&mut self, mnem: MnemonicType) -> OneArgType {
        let token = self.fetch();
        match token {
            Token::Register(RegisterType::Direct(_)) => match is_byte_op(&mnem) {
                true => OneArgType::RegB(mnem, token),
                false => OneArgType::Reg(mnem, token),
            },
            Token::Register(RegisterType::Indirect(_)) => OneArgType::RegIndirect(mnem, token),
            Token::Literal(_) => OneArgType::Lit(mnem, token),
            Token::Label(_) => OneArgType::Label(mnem, token),
            _ => unreachable!(),
        }
    }

    fn parse_two_args(&mut self, mnem: MnemonicType) -> TwoArgsType {
        let token_dest = self.fetch();
        let token_orig = self.fetch();
        match token_dest {
            Token::Register(RegisterType::Direct(_)) => match token_orig {
                Token::Register(RegisterType::Direct(_)) => match is_byte_op(&mnem) {
                    true => TwoArgsType::RegBRegB(mnem, token_dest, token_orig),
                    false => TwoArgsType::RegReg(mnem, token_dest, token_orig),
                },
                Token::Register(RegisterType::Indirect(_)) => {
                    TwoArgsType::RegRegI(mnem, token_dest, token_orig)
                }
                Token::Literal(_) => match is_byte_op(&mnem) {
                    true => TwoArgsType::RegLitB(mnem, token_dest, token_orig),
                    false => TwoArgsType::RegLit(mnem, token_dest, token_orig),
                },
                _ => {
                    println!("{:?} {:?}", token_dest, token_orig);
                    unreachable!()
                }
            },
            Token::Register(RegisterType::Indirect(_)) => match token_orig {
                Token::Register(RegisterType::Direct(_)) => match is_byte_op(&mnem) {
                    true => TwoArgsType::RegIRegB(mnem, token_dest, token_orig),
                    false => TwoArgsType::RegIReg(mnem, token_dest, token_orig),
                },
                _ => unreachable!(),
            },
            _ => {
                println!("{:?} {:?}", token_dest, token_orig);
                unreachable!()
            }
        }
    }
}

pub fn is_no_args(mnem: &MnemonicType) -> bool {
    matches!(
        mnem,
        MnemonicType::NOP
            | MnemonicType::HLT
            | MnemonicType::CLI
            | MnemonicType::SEI
            | MnemonicType::RSI
            | MnemonicType::RSB
    )
}

pub fn is_one_arg(mnem: &MnemonicType) -> bool {
    // INC, DEC, NOT, JMP, JZ, JSB, PHR, PLR
    matches!(
        mnem,
        MnemonicType::INC
            | MnemonicType::INCB
            | MnemonicType::DEC
            | MnemonicType::DECB
            | MnemonicType::NOT
            | MnemonicType::NOTB
            | MnemonicType::PHR
            | MnemonicType::PHL
            | MnemonicType::JMP
            | MnemonicType::JSB
            | MnemonicType::JZ
            | MnemonicType::JNZ
            | MnemonicType::JN
            | MnemonicType::JNN
            | MnemonicType::JO
            | MnemonicType::JNO
    )
}

pub fn is_conditional_jump(mnem: &MnemonicType) -> bool {
    matches!(
        mnem,
        MnemonicType::JZ
            | MnemonicType::JNZ
            | MnemonicType::JN
            | MnemonicType::JNN
            | MnemonicType::JO
            | MnemonicType::JNO
    )
}

pub fn is_byte_op(mnem: &MnemonicType) -> bool {
    // MOVB, ADDB, SUBB, MULB, DIVB, MODB, INCB, DECB, ANDB, ORB, XORB, NOTB, SHLB, SHRB, CMPB
    matches!(
        mnem,
        MnemonicType::MOVB
            | MnemonicType::ADDB
            | MnemonicType::SUBB
            | MnemonicType::MULB
            | MnemonicType::DIVB
            | MnemonicType::MODB
            | MnemonicType::INCB
            | MnemonicType::DECB
            | MnemonicType::ANDB
            | MnemonicType::ORB
            | MnemonicType::XORB
            | MnemonicType::NOTB
            | MnemonicType::SHLB
            | MnemonicType::SHRB
            | MnemonicType::CMPB
    )
}
