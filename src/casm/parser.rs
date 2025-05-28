use crate::casm::lexer::{Lexer, Token};
use crate::error::AssembleError;
use indexmap::IndexMap;

#[derive(Debug, Clone)]
pub enum Operand {
    Register(u8),
    RegisterIndirect(u8),
    Literal(u16),
    Alias(String),
    LabelRef(String),
}

#[derive(Debug, Clone)]
pub enum Instruction {
    Nop,
    Hlt,
    Mov(Operand, Operand),
    Phr(Operand),
    Plr(Operand),
    Add(Operand, Operand),
    Sub(Operand, Operand),
    Mul(Operand, Operand),
    Div(Operand, Operand),
    Mod(Operand, Operand),
    Inc(Operand),
    Dec(Operand),
    And(Operand, Operand),
    Or(Operand, Operand),
    Xor(Operand, Operand),
    Not(Operand),
    Cmp(Operand, Operand),
    Jmp(Operand),
    Jz(Operand),
    Jnz(Operand),
    Jn(Operand),
    Jnn(Operand),
    Jc(Operand),
    Jnc(Operand),
    Jsb(Operand),
    Rsb,
    Cli,
    Sei,
    Rsi,
}

#[derive(Debug, Clone)]
pub enum Statement {
    Instruction(Instruction),
    Label(String),
    AliasDeclaration(String, Operand),
    Directive(String, Operand),
}

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
    pub aliases: IndexMap<String, Operand>,
    pub labels: IndexMap<String, u16>,
}

pub struct Parser<'a> {
    lexer: Lexer<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            lexer: Lexer::new(input),
        }
    }

    pub fn parse(&mut self) -> Result<Program, AssembleError> {
        let mut statements = Vec::new();
        let mut aliases = IndexMap::new();

        // Skip initial newlines
        while let Some(Token::Newline) = self.lexer.current() {
            self.lexer.advance();
        }

        while !self.lexer.is_at_end() {
            match self.lexer.current() {
                Some(Token::Newline) => {
                    self.lexer.advance();
                    continue;
                }
                Some(Token::Label(name)) => {
                    let label_name = name.clone();
                    self.lexer.advance();
                    statements.push(Statement::Label(label_name));
                }
                Some(Token::Alias(name)) => {
                    let alias_name = name.clone();
                    self.lexer.advance();
                    let operand = self.parse_operand()?;
                    aliases.insert(alias_name.clone(), operand.clone());
                    statements.push(Statement::AliasDeclaration(alias_name, operand));
                }
                Some(Token::Directive(name)) => {
                    let directive_name = name.clone();
                    self.lexer.advance();
                    let value = self.parse_operand()?;
                    statements.push(Statement::Directive(directive_name, value));
                }
                _ => {
                    let instruction = self.parse_instruction()?;
                    statements.push(Statement::Instruction(instruction));
                }
            }
        }

        Ok(Program {
            statements,
            aliases,
            labels: IndexMap::new(), // Will be populated during first pass
        })
    }

    fn parse_instruction(&mut self) -> Result<Instruction, AssembleError> {
        match self.lexer.current() {
            Some(Token::Nop) => {
                self.lexer.advance();
                Ok(Instruction::Nop)
            }
            Some(Token::Hlt) => {
                self.lexer.advance();
                Ok(Instruction::Hlt)
            }
            Some(Token::Mov) => {
                self.lexer.advance();
                let dest = self.parse_operand()?;
                let src = self.parse_operand()?;
                Ok(Instruction::Mov(dest, src))
            }
            Some(Token::Phr) => {
                self.lexer.advance();
                let src = self.parse_operand()?;
                Ok(Instruction::Phr(src))
            }
            Some(Token::Plr) => {
                self.lexer.advance();
                let src = self.parse_operand()?;
                Ok(Instruction::Phr(src))
            }
            Some(Token::Add) => {
                self.lexer.advance();
                let op1 = self.parse_operand()?;
                let op2 = self.parse_operand()?;
                Ok(Instruction::Add(op1, op2))
            }
            Some(Token::Sub) => {
                self.lexer.advance();
                let op1 = self.parse_operand()?;
                let op2 = self.parse_operand()?;
                Ok(Instruction::Sub(op1, op2))
            }
            Some(Token::Mul) => {
                self.lexer.advance();
                let op1 = self.parse_operand()?;
                let op2 = self.parse_operand()?;
                Ok(Instruction::Mul(op1, op2))
            }
            Some(Token::Div) => {
                self.lexer.advance();
                let op1 = self.parse_operand()?;
                let op2 = self.parse_operand()?;
                Ok(Instruction::Div(op1, op2))
            }
            Some(Token::Mod) => {
                self.lexer.advance();
                let op1 = self.parse_operand()?;
                let op2 = self.parse_operand()?;
                Ok(Instruction::Mod(op1, op2))
            }
            Some(Token::Inc) => {
                self.lexer.advance();
                let op = self.parse_operand()?;
                Ok(Instruction::Inc(op))
            }
            Some(Token::Dec) => {
                self.lexer.advance();
                let op = self.parse_operand()?;
                Ok(Instruction::Dec(op))
            }
            Some(Token::And) => {
                self.lexer.advance();
                let op1 = self.parse_operand()?;
                let op2 = self.parse_operand()?;
                Ok(Instruction::And(op1, op2))
            }
            Some(Token::Or) => {
                self.lexer.advance();
                let op1 = self.parse_operand()?;
                let op2 = self.parse_operand()?;
                Ok(Instruction::Or(op1, op2))
            }
            Some(Token::Xor) => {
                self.lexer.advance();
                let op1 = self.parse_operand()?;
                let op2 = self.parse_operand()?;
                Ok(Instruction::Xor(op1, op2))
            }
            Some(Token::Not) => {
                self.lexer.advance();
                let op = self.parse_operand()?;
                Ok(Instruction::Not(op))
            }
            Some(Token::Cmp) => {
                self.lexer.advance();
                let op1 = self.parse_operand()?;
                let op2 = self.parse_operand()?;
                Ok(Instruction::Cmp(op1, op2))
            }
            Some(Token::Jmp) => {
                self.lexer.advance();
                let op = self.parse_operand()?;
                Ok(Instruction::Jmp(op))
            }
            Some(Token::Jz) => {
                self.lexer.advance();
                let op = self.parse_operand()?;
                Ok(Instruction::Jz(op))
            }
            Some(Token::Jnz) => {
                self.lexer.advance();
                let op = self.parse_operand()?;
                Ok(Instruction::Jnz(op))
            }
            Some(Token::Jn) => {
                self.lexer.advance();
                let op = self.parse_operand()?;
                Ok(Instruction::Jn(op))
            }
            Some(Token::Jnn) => {
                self.lexer.advance();
                let op = self.parse_operand()?;
                Ok(Instruction::Jnn(op))
            }
            Some(Token::Jc) => {
                self.lexer.advance();
                let op = self.parse_operand()?;
                Ok(Instruction::Jc(op))
            }
            Some(Token::Jnc) => {
                self.lexer.advance();
                let op = self.parse_operand()?;
                Ok(Instruction::Jnc(op))
            }
            Some(Token::Jsb) => {
                self.lexer.advance();
                let op = self.parse_operand()?;
                Ok(Instruction::Jsb(op))
            }
            Some(Token::Rsb) => {
                self.lexer.advance();
                Ok(Instruction::Rsb)
            }
            Some(Token::Cli) => {
                self.lexer.advance();
                Ok(Instruction::Cli)
            }
            Some(Token::Sei) => {
                self.lexer.advance();
                Ok(Instruction::Sei)
            }
            Some(Token::Rsi) => {
                self.lexer.advance();
                Ok(Instruction::Rsi)
            }
            other => Err(AssembleError::InvalidInstruction(
                format!("Unexpected token: {:?} at line {}", other, self.lexer.line())
            )),
        }
    }

    fn parse_operand(&mut self) -> Result<Operand, AssembleError> {
        match self.lexer.current() {
            Some(Token::Register(reg)) => {
                let r = *reg;
                self.lexer.advance();
                Ok(Operand::Register(r))
            }
            Some(Token::RegisterIndirect(reg)) => {
                let r = *reg;
                self.lexer.advance();
                Ok(Operand::RegisterIndirect(r))
            }
            Some(Token::DecimalLiteral(val)) => {
                let v = *val;
                self.lexer.advance();
                Ok(Operand::Literal(v))
            }
            Some(Token::HexLiteral(val)) => {
                let v = *val;
                self.lexer.advance();
                Ok(Operand::Literal(v))
            }
            Some(Token::Alias(name)) => {
                let n = name.clone();
                self.lexer.advance();
                Ok(Operand::Alias(n))
            }
            Some(Token::Identifier(name)) => {
                let n = name.clone();
                self.lexer.advance();
                Ok(Operand::LabelRef(n))
            }
            other => Err(AssembleError::ParseError(
                format!("Expected operand, found {:?} at line {}", other, self.lexer.line())
            )),
        }
    }
}