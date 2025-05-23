use crate::assembler::lexer::LexerError;
use crate::assembler::parser::ParserError;
use crate::assembler::codegen::CodeGenError;
use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum AssemblerError {
    Lexer {
        line_number: usize,
        content: String,
        kind: LexerError,
    },
    Parser {
        line_number: usize,
        content: String,
        kind: ParserError,
    },
    CodeGen {
        line_number: usize,
        content: String, 
        kind: CodeGenError,
    },
    Driver {
        line_number: Option<usize>,
        message: String,
    },
    Io {
        message: String,
    }
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LexerError::InvalidCharacter(c) => write!(f, "Invalid character: '{}'", c),
            LexerError::InvalidLiteralFormat(s) => write!(f, "Invalid literal format: '{}'", s),
            LexerError::InvalidRegisterFormat(s) => write!(f, "Invalid register format: '{}'", s),
            LexerError::UnknownToken(s) => write!(f, "Unknown token starting with: '{}'", s),
        }
    }
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParserError::UnexpectedToken(token) => write!(f, "Unexpected token: {:?}", token),
            ParserError::MissingOperand => write!(f, "Missing operand"),
            ParserError::InvalidOperandType(token) => write!(f, "Invalid operand type: {:?}", token),
            ParserError::UnknownInstruction(name) => write!(f, "Unknown instruction: '{}'", name),
            ParserError::TrailingTokens(tokens) => write!(f, "Trailing tokens: {:?}", tokens),
            ParserError::EmptyInput => write!(f, "Empty input or no significant tokens found"),
            ParserError::InvalidDirective(name) => write!(f, "Invalid directive: '{}'", name),
            ParserError::InvalidLabel(name) => write!(f, "Invalid label: '{}'", name),
            ParserError::MissingComma(token) => write!(f, "Missing comma after token: {:?}", token),
            ParserError::ExpectedMnemonic => write!(f, "Expected a mnemonic (instruction name)"),
            ParserError::ExpectedRegister => write!(f, "Expected a register"),
            ParserError::ExpectedLiteral => write!(f, "Expected a literal value"),
            ParserError::ExpectedLabelIdentifier => write!(f, "Expected a label identifier"),
            ParserError::ExpectedColon => write!(f, "Expected a colon ':' after label identifier"),
            ParserError::ExpectedSpecificToken(s) => write!(f, "Expected specific token: {}", s),
        }
    }
}

impl fmt::Display for CodeGenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CodeGenError::UnsupportedInstruction(s) => write!(f, "Unsupported instruction: {}", s),
            CodeGenError::InvalidOperandForInstruction(s) => write!(f, "Invalid operand for instruction: {}", s),
            CodeGenError::MissingLabel(name) => write!(f, "Missing label: '{}'", name),
            CodeGenError::OperandIsNotRegister(op) => write!(f, "Operand is not a register: {:?}", op),
            CodeGenError::OperandIsNotLiteral(op) => write!(f, "Operand is not a literal: {:?}", op),
            CodeGenError::OperandIsNotDereferencedRegister(op) => write!(f, "Operand is not a dereferenced register: {:?}", op),
            CodeGenError::OperandCombinationNotSupported(instr) => write!(f, "Operand combination not supported for instruction: {:?}", instr),
            CodeGenError::RegisterIdOutOfRange(id) => write!(f, "Register ID out of range: {}", id),
            CodeGenError::LiteralOutOfRange(val) => write!(f, "Literal value out of range: {}", val),
        }
    }
}

impl fmt::Display for AssemblerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AssemblerError::Lexer { line_number, content, kind } => {
                write!(f, "Lexer Error on line {}: '{}' - Details: {}", line_number, content, kind)
            }
            AssemblerError::Parser { line_number, content, kind } => {
                write!(f, "Parser Error on line {}: '{}' - Details: {}", line_number, content, kind)
            }
            AssemblerError::CodeGen { line_number, content, kind } => {
                write!(f, "CodeGen Error on line {}: Processing '{}' - Details: {}", line_number, content, kind)
            }
            AssemblerError::Driver { line_number, message } => {
                if let Some(ln) = line_number {
                    write!(f, "Assembler Driver Error on line {}: {}", ln, message)
                } else {
                    write!(f, "Assembler Driver Error: {}", message)
                }
            }
            AssemblerError::Io { message } => {
                write!(f, "I/O Error: {}", message)
            }
        }
    }
}
