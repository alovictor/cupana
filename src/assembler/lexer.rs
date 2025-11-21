use regex::Regex;
use std::fs::File;
use std::io::{BufReader, Read};

#[derive(Debug, PartialEq)]
pub enum Token {
    Mnemonic(Opcode),
    Register(RegisterType),
    Literal(LiteralType),
    Label(String),
    Directive(DirectiveType),
    Comment,
    NewLine,
    Comma,
    None,
}

#[derive(Debug, PartialEq)]
pub enum Opcode {
    NOP,
    HLT,
    MOV,
    MOVB,
    PHR,
    PHL,
    ADD,
    ADDB,
    SUB,
    SUBB,
    MUL,
    MULB,
    DIV,
    DIVB,
    MOD,
    MODB,
    INC,
    INCB,
    DEC,
    DECB,
    AND,
    ANDB,
    OR,
    ORB,
    XOR,
    XORB,
    NOT,
    NOTB,
    SHL,
    SHLB,
    SHR,
    SHRB,
    CMP,
    CMPB,
    JMP,
    JZ,
    JNZ,
    JN,
    JNN,
    JO,
    JNO,
    JSB,
    RSB,
    CLI,
    SEI,
    RSI,
    NONE,
}

impl Opcode {
    pub fn to_byte(&self) -> u8 {
        match self {
            Opcode::NOP => 0x00,
            Opcode::HLT => 0x01,
            Opcode::MOV => 0x02,
            Opcode::MOVB => 0x02,
            Opcode::PHR => 0x03,
            Opcode::PHL => 0x04,
            Opcode::ADD => 0x05,
            Opcode::ADDB => 0x05,
            Opcode::SUB => 0x06,
            Opcode::SUBB => 0x06,
            Opcode::MUL => 0x07,
            Opcode::MULB => 0x07,
            Opcode::DIV => 0x08,
            Opcode::DIVB => 0x08,
            Opcode::MOD => 0x09,
            Opcode::MODB => 0x09,
            Opcode::INC => 0x0A,
            Opcode::INCB => 0x0A,
            Opcode::DEC => 0x0B,
            Opcode::DECB => 0x0B,
            Opcode::AND => 0x0C,
            Opcode::ANDB => 0x0C,
            Opcode::OR => 0x0D,
            Opcode::ORB => 0x0D,
            Opcode::XOR => 0x0E,
            Opcode::XORB => 0x0E,
            Opcode::NOT => 0x0F,
            Opcode::NOTB => 0x0F,
            Opcode::SHL => 0x10,
            Opcode::SHLB => 0x10,
            Opcode::SHR => 0x11,
            Opcode::SHRB => 0x11,
            Opcode::CMP => 0x12,
            Opcode::CMPB => 0x12,
            Opcode::JMP => 0x13,
            Opcode::JZ => 0x14,
            Opcode::JNZ => 0x14,
            Opcode::JN => 0x14,
            Opcode::JNN => 0x14,
            Opcode::JO => 0x14,
            Opcode::JNO => 0x14,
            Opcode::JSB => 0x15,
            Opcode::RSB => 0x16,
            Opcode::CLI => 0x17,
            Opcode::SEI => 0x18,
            Opcode::RSI => 0x19,
            _ => unreachable!(),
        }
    }
}

impl From<&str> for Opcode {
    fn from(value: &str) -> Opcode {
        match value.to_uppercase().as_str() {
            "NOP" => Opcode::NOP,
            "HLT" => Opcode::HLT,
            "MOV" => Opcode::MOV,
            "MOVB" => Opcode::MOVB,
            "PHR" => Opcode::PHR,
            "PHL" => Opcode::PHL,
            "ADD" => Opcode::ADD,
            "ADDB" => Opcode::ADDB,
            "SUB" => Opcode::SUB,
            "SUBB" => Opcode::SUBB,
            "MUL" => Opcode::MUL,
            "MULB" => Opcode::MULB,
            "DIV" => Opcode::DIV,
            "DIVB" => Opcode::DIVB,
            "MOD" => Opcode::MOD,
            "MODB" => Opcode::MODB,
            "INC" => Opcode::INC,
            "INCB" => Opcode::INCB,
            "DEC" => Opcode::DEC,
            "DECB" => Opcode::DECB,
            "AND" => Opcode::AND,
            "ANDB" => Opcode::ANDB,
            "OR" => Opcode::OR,
            "ORB" => Opcode::ORB,
            "XOR" => Opcode::XOR,
            "XORB" => Opcode::XORB,
            "NOT" => Opcode::NOT,
            "NOTB" => Opcode::NOTB,
            "SHL" => Opcode::SHL,
            "SHLB" => Opcode::SHLB,
            "SHR" => Opcode::SHR,
            "SHRB" => Opcode::SHRB,
            "CMP" => Opcode::CMP,
            "CMPB" => Opcode::CMPB,
            "JMP" => Opcode::JMP,
            "JZ" => Opcode::JZ,
            "JNZ" => Opcode::JNZ,
            "JN" => Opcode::JN,
            "JNN" => Opcode::JNN,
            "JO" => Opcode::JO,
            "JNO" => Opcode::JNO,
            "JSB" => Opcode::JSB,
            "RSB" => Opcode::RSB,
            "CLI" => Opcode::CLI,
            "SEI" => Opcode::SEI,
            "RSI" => Opcode::RSI,
            _ => Opcode::NONE,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum DirectiveType {
    ORG,
    INCLUDE,
    GLOBAL,
    SHORT,
    BYTE,
    ASCII,
}

#[derive(Debug, PartialEq)]
pub enum LiteralType {
    Dec(u16),
    Hex(u16),
    Bin(u16),
}

#[derive(Debug, PartialEq)]
pub enum RegisterType {
    Direct(u8),
    Indirect(u8),
}

pub struct CupanaLexer {
    buffer: Vec<u8>,
    idx: usize,
    line: usize,
    col: usize,
    total_lines: usize,
    total_tokens: usize,
    output: Vec<Token>,
}

impl CupanaLexer {
    pub fn new(path: &str) -> Self {
        let file = File::open(path).expect("Erro ao abrir o arquivo");
        let reader = BufReader::new(file);
        let mut buffer: Vec<u8> = vec![];
        let mut total_lines = 0;

        for byte in reader.bytes() {
            let byte = byte.expect("Erro ao ler linha");
            buffer.push(byte);
            if byte == '\n' as u8 {
                total_lines += 1;
            }
        }
        println!("Total de linhas: {}", total_lines);
        println!("Tamanho do arquivo: {} bytes", buffer.len());
        Self {
            buffer,
            idx: 0,
            line: 0,
            col: 0,
            total_lines,
            total_tokens: 0,
            output: vec![],
        }
    }

    fn fetch(&mut self) -> u8 {
        let value = self.buffer[self.idx];
        self.idx += 1;
        self.col += 1;
        value
    }

    pub fn get_output(&self) -> &[Token] {
        self.output.as_slice()
    }

    pub fn lex(&mut self) {
        let mut symbols: Vec<String> = vec![];
        while !(self.idx >= self.buffer.len()) {
            let byte = self.fetch();
            if byte == '\n' as u8 {
                self.line += 1;
                self.col = 0;
                continue;
            }
            if byte == ' ' as u8 || byte == '\t' as u8 {
                continue;
            }

            let mut symbol = vec![byte];
            loop {
                if self.idx == self.buffer.len() - 1 {
                    break;
                }
                let peeked_byte = self.fetch();

                if peeked_byte == ' ' as u8 || peeked_byte == '\n' as u8 {
                    break;
                }
                symbol.push(peeked_byte);
            }
            let mut string_symbol = String::new();
            symbol
                .as_slice()
                .read_to_string(&mut string_symbol)
                .expect("[LEXER] Não foi possivel escrever os bytes na string");
            symbols.push(string_symbol.to_lowercase());
        }

        let org = Regex::new(r"\.[a-z]*").unwrap();
        let label = Regex::new(r"[a-z]*:").unwrap();
        let reg = Regex::new(r"r[0-9]{1,2}\*?").unwrap();
        let lit_bin = Regex::new(r"0b[0-1]").unwrap();
        let lit_hex = Regex::new(r"0x[0-9a-f]").unwrap();
        let mnem = Regex::new(r"[a-z]{2,4}").unwrap();
        let lit_dec = Regex::new(r"[0-9]*").unwrap();

        for symbol in symbols {
            let mut token = Token::None;
            let mut sym = symbol.as_str();

            match org.is_match(sym) {
                true => match sym.strip_prefix(".") {
                    Some(string) => match string {
                        "org" => token = Token::Directive(DirectiveType::ORG),
                        "include" => token = Token::Directive(DirectiveType::INCLUDE),
                        "global" => token = Token::Directive(DirectiveType::GLOBAL),
                        "short" => token = Token::Directive(DirectiveType::SHORT),
                        "byte" => token = Token::Directive(DirectiveType::BYTE),
                        "ascii" => token = Token::Directive(DirectiveType::ASCII),
                        _ => {}
                    },
                    None => {}
                },
                false => match label.is_match(sym) {
                    true => match sym.strip_suffix(":") {
                        Some(string) => token = Token::Label(string.to_string()),
                        None => {}
                    },
                    false => match reg.is_match(sym) {
                        true => match sym.ends_with("*") {
                            true => {
                                sym = sym.trim_matches('*');
                                sym = sym.trim_matches('r');
                                sym = sym.trim_matches(',');

                                token = Token::Register(RegisterType::Indirect(
                                    u8::from_str_radix(sym, 10)
                                        .expect("Erro ao gerar u8 de string"),
                                ));
                            }
                            false => {
                                sym = sym.trim_matches('r');
                                sym = sym.trim_matches(',');
                                token = Token::Register(RegisterType::Direct(
                                    u8::from_str_radix(sym, 10)
                                        .expect("Erro ao gerar u8 de string"),
                                ));
                            }
                        },
                        false => match lit_bin.is_match(sym) {
                            true => {
                                let value = sym.strip_prefix("0b").unwrap();
                                match u16::from_str_radix(value, 2) {
                                    Ok(value) => token = Token::Literal(LiteralType::Bin(value)),
                                    Err(e) => println!("Erro no Bin: {}", e),
                                }
                            }
                            false => match lit_hex.is_match(sym) {
                                true => {
                                    let value = sym.strip_prefix("0x").unwrap();
                                    match u16::from_str_radix(value, 16) {
                                        Ok(value) => {
                                            token = Token::Literal(LiteralType::Hex(value))
                                        }
                                        Err(e) => println!("Erro no Hex: {:?}", e),
                                    }
                                }
                                false => match mnem.is_match(sym) {
                                    true => token = Token::from(Token::Mnemonic(Opcode::from(sym))),
                                    false => match lit_dec.is_match(sym) {
                                        true => match u16::from_str_radix(sym, 10) {
                                            Ok(value) => {
                                                token = Token::Literal(LiteralType::Dec(value))
                                            }
                                            Err(e) => println!("Erro no Dec: {} {:?}", sym, e),
                                        },
                                        false => token = Token::None,
                                    },
                                },
                            },
                        },
                    },
                },
            }
            if token != Token::None {
                self.output.push(token);
            }
        }
    }
}
