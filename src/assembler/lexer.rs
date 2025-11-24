use regex::Regex;
use std::fs::File;
use std::io::{BufReader, Read};

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Mnemonic(MnemonicType),
    Register(RegisterType),
    Literal(u16),
    LabelDef(String),
    Label(String),
    Directive(DirectiveType),
    Comment,
    NewLine,
    Comma,
    None,
}

#[derive(Debug, PartialEq, Clone)]
pub enum MnemonicType {
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

impl MnemonicType {
    pub fn to_byte(&self) -> u8 {
        match self {
            MnemonicType::NOP => 0x00,
            MnemonicType::HLT => 0x01,
            MnemonicType::MOV => 0x02,
            MnemonicType::MOVB => 0x02,
            MnemonicType::PHR => 0x03,
            MnemonicType::PHL => 0x04,
            MnemonicType::ADD => 0x05,
            MnemonicType::ADDB => 0x05,
            MnemonicType::SUB => 0x06,
            MnemonicType::SUBB => 0x06,
            MnemonicType::MUL => 0x07,
            MnemonicType::MULB => 0x07,
            MnemonicType::DIV => 0x08,
            MnemonicType::DIVB => 0x08,
            MnemonicType::MOD => 0x09,
            MnemonicType::MODB => 0x09,
            MnemonicType::INC => 0x0A,
            MnemonicType::INCB => 0x0A,
            MnemonicType::DEC => 0x0B,
            MnemonicType::DECB => 0x0B,
            MnemonicType::AND => 0x0C,
            MnemonicType::ANDB => 0x0C,
            MnemonicType::OR => 0x0D,
            MnemonicType::ORB => 0x0D,
            MnemonicType::XOR => 0x0E,
            MnemonicType::XORB => 0x0E,
            MnemonicType::NOT => 0x0F,
            MnemonicType::NOTB => 0x0F,
            MnemonicType::SHL => 0x10,
            MnemonicType::SHLB => 0x10,
            MnemonicType::SHR => 0x11,
            MnemonicType::SHRB => 0x11,
            MnemonicType::CMP => 0x12,
            MnemonicType::CMPB => 0x12,
            MnemonicType::JMP => 0x13,
            MnemonicType::JZ => 0x14,
            MnemonicType::JNZ => 0x14,
            MnemonicType::JN => 0x14,
            MnemonicType::JNN => 0x14,
            MnemonicType::JO => 0x14,
            MnemonicType::JNO => 0x14,
            MnemonicType::JSB => 0x15,
            MnemonicType::RSB => 0x16,
            MnemonicType::CLI => 0x17,
            MnemonicType::SEI => 0x18,
            MnemonicType::RSI => 0x19,
            _ => unreachable!(),
        }
    }
}

impl From<&str> for MnemonicType {
    fn from(value: &str) -> MnemonicType {
        match value.to_uppercase().as_str() {
            "NOP" => MnemonicType::NOP,
            "HLT" => MnemonicType::HLT,
            "MOV" => MnemonicType::MOV,
            "MOVB" => MnemonicType::MOVB,
            "PHR" => MnemonicType::PHR,
            "PHL" => MnemonicType::PHL,
            "ADD" => MnemonicType::ADD,
            "ADDB" => MnemonicType::ADDB,
            "SUB" => MnemonicType::SUB,
            "SUBB" => MnemonicType::SUBB,
            "MUL" => MnemonicType::MUL,
            "MULB" => MnemonicType::MULB,
            "DIV" => MnemonicType::DIV,
            "DIVB" => MnemonicType::DIVB,
            "MOD" => MnemonicType::MOD,
            "MODB" => MnemonicType::MODB,
            "INC" => MnemonicType::INC,
            "INCB" => MnemonicType::INCB,
            "DEC" => MnemonicType::DEC,
            "DECB" => MnemonicType::DECB,
            "AND" => MnemonicType::AND,
            "ANDB" => MnemonicType::ANDB,
            "OR" => MnemonicType::OR,
            "ORB" => MnemonicType::ORB,
            "XOR" => MnemonicType::XOR,
            "XORB" => MnemonicType::XORB,
            "NOT" => MnemonicType::NOT,
            "NOTB" => MnemonicType::NOTB,
            "SHL" => MnemonicType::SHL,
            "SHLB" => MnemonicType::SHLB,
            "SHR" => MnemonicType::SHR,
            "SHRB" => MnemonicType::SHRB,
            "CMP" => MnemonicType::CMP,
            "CMPB" => MnemonicType::CMPB,
            "JMP" => MnemonicType::JMP,
            "JZ" => MnemonicType::JZ,
            "JNZ" => MnemonicType::JNZ,
            "JN" => MnemonicType::JN,
            "JNN" => MnemonicType::JNN,
            "JO" => MnemonicType::JO,
            "JNO" => MnemonicType::JNO,
            "JSB" => MnemonicType::JSB,
            "RSB" => MnemonicType::RSB,
            "CLI" => MnemonicType::CLI,
            "SEI" => MnemonicType::SEI,
            "RSI" => MnemonicType::RSI,
            _ => MnemonicType::NONE,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum DirectiveType {
    ORG,
    INCLUDE,
    GLOBAL,
    SHORT,
    BYTE,
    ASCII,
}

#[derive(Debug, PartialEq, Clone)]
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

    pub fn get_output(&self) -> &Vec<Token> {
        &self.output
    }

    pub fn lex(&mut self) -> &Vec<Token> {
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
        let labeldef = Regex::new(r"[a-z]*:").unwrap();
        let reg = Regex::new(r"r[0-9]{1,2}\*?").unwrap();
        let lit_bin = Regex::new(r"0b[0-1]").unwrap();
        let lit_hex = Regex::new(r"0x[0-9a-f]").unwrap();
        let lit_dec = Regex::new(r"[0-9]+$").unwrap();
        let label = Regex::new(r"[a-z]*").unwrap();

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
                false => match labeldef.is_match(sym) {
                    true => match sym.strip_suffix(":") {
                        Some(string) => token = Token::LabelDef(string.to_string()),
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
                                    Ok(value) => token = Token::Literal(value),
                                    Err(e) => println!("Erro no Bin: {}", e),
                                }
                            }
                            false => match lit_hex.is_match(sym) {
                                true => {
                                    let value = sym.strip_prefix("0x").unwrap();
                                    match u16::from_str_radix(value, 16) {
                                        Ok(value) => token = Token::Literal(value),
                                        Err(e) => println!("Erro no Hex: {:?}", e),
                                    }
                                }
                                false => {
                                    if MnemonicType::from(sym) != MnemonicType::NONE {
                                        token =
                                            Token::from(Token::Mnemonic(MnemonicType::from(sym)))
                                    } else {
                                        match lit_dec.is_match(sym) {
                                            true => match u16::from_str_radix(sym, 10) {
                                                Ok(value) => token = Token::Literal(value),
                                                Err(e) => println!("Erro no Dec: {} {:?}", sym, e),
                                            },
                                            false => match label.is_match(sym) {
                                                true => token = Token::Label(sym.to_string()),
                                                false => token = Token::None,
                                            },
                                        }
                                    }
                                }
                            },
                        },
                    },
                },
            }
            if token != Token::None {
                self.output.push(token);
            }
        }
        return self.get_output();
    }
}
