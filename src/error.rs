use std::fmt;
use std::io;
use std::num::ParseIntError;

// Erro principal do emulador
#[derive(Debug)]
pub enum CError {
    VM(VMError),
    IoError(io::Error),
    Assemble(AssembleError),
    Memory(MemoryError),
}

// Implementação de Display para CError
impl fmt::Display for CError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CError::VM(error) => write!(f, "Erro na VM: {}", error),
            CError::IoError(error) => write!(f, "I/O error {}", error),
            CError::Assemble(error) => write!(f, "Erro no Assembler: {}", error),
            CError::Memory(error) => write!(f, "Erro de memória: {}", error),
        }
    }
}

// Conversão automática de VMError para CError
impl From<VMError> for CError {
    fn from(error: VMError) -> Self {
        CError::VM(error)
    }
}

impl From<AssembleError> for CError {
    fn from(error: AssembleError) -> Self {
        CError::Assemble(error)
    }
}

impl From<io::Error> for CError {
    fn from(error: io::Error) -> Self {
        CError::IoError(error)
    }
}

impl From<MemoryError> for CError {
    fn from(error: MemoryError) -> Self {
        CError::Memory(error)
    }
}


#[derive(Debug)]
pub enum VMError {
    IoError(io::Error),
    MemoryError(MemoryError),
    InvalidOpcode(u8),
    StackOverflow,
    StackUnderflow,
    InvalidRegister(u8),
    DivideByZero,
}

impl fmt::Display for VMError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VMError::IoError(error) => write!(f, "Erro de I/O: {}", error),
            VMError::InvalidOpcode(code) => write!(f, "Opcode inválido: {}", code),
            VMError::MemoryError(error) => write!(f, "Erro de memória: {}", error),
            VMError::StackOverflow => write!(f, "Stack Overflow"),
            VMError::StackUnderflow => write!(f, "Stack Underflow"),
            VMError::InvalidRegister(reg) => write!(f, "Invalid Register: {}", reg),
            VMError::DivideByZero => write!(f, "Division by Zero"),
        }
    }
}

impl From<io::Error> for VMError {
    fn from(error: io::Error) -> Self {
        VMError::IoError(error)
    }
}

impl From<MemoryError> for VMError {
    fn from(error: MemoryError) -> Self {
        VMError::MemoryError(error)
    }
}

#[derive(Debug)]
pub enum AssembleError {
    IoError(io::Error),
    InvalidInstruction(String),
    InvalidOpcode(String),
    ParseIntError(ParseIntError),
    ParseError(String),
    GenericError(String),
}

impl fmt::Display for AssembleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AssembleError::IoError(error) => write!(f, "Erro de I/O: {}", error),
            AssembleError::InvalidInstruction(inst) => write!(f, "Instrução inválida: {}", inst),
            AssembleError::InvalidOpcode(code) => write!(f, "Opcode inválido: {}", code),
            AssembleError::ParseIntError(error) => write!(f, "Erro ao converter inteiro: {}", error),
            AssembleError::GenericError(error) => write!(f, "Erro genérico: {}", error),
            AssembleError::ParseError(error) => write!(f, "Erro ao fazer parse: {}", error),
        }
    }
}

impl From<io::Error> for AssembleError {
    fn from(error: io::Error) -> Self {
        AssembleError::IoError(error)
    }
}

impl From<ParseIntError> for AssembleError {
    fn from(error: ParseIntError) -> Self {
        AssembleError::ParseIntError(error)
    }
}

#[derive(Debug)]
pub enum MemoryError {
    InvalidRamAddress(u16),
    InvalidRomSize(usize),
    WriteNotPermitted(u16),
}

impl fmt::Display for MemoryError {
    fn fmt(&self, f: &mut fmt::	Formatter<'_>) -> fmt::Result {
        match self {
            MemoryError::InvalidRamAddress(addr) => write!(f, "Endereço de RAN inválido: {}", addr),
            MemoryError::InvalidRomSize(size) => write!(f, "Tamanho da ROM inválido: {}", size),
            MemoryError::WriteNotPermitted(addr) => write!(f, "Escrita não permitida no endereço: {}", addr),
        }
    }
}