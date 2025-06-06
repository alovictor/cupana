use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(skip r"[ \t\r\f]+")]
pub enum Token {
    // Comentários
    #[regex(r";[^\n]*", logos::skip)]
    Comment,

    // Números literais
    #[regex(r"\$[0-9]+", |lex| lex.slice()[1..].parse::<u16>().ok())]
    DecimalLiteral(u16),

    #[regex(r"#[0-9a-fA-F]+", |lex| u16::from_str_radix(&lex.slice()[1..], 16).ok())]
    HexLiteral(u16),

    // Registradores
    #[regex(r"[Rr][0-9]+", |lex| {
        let num_str = &lex.slice()[1..];
        num_str.parse::<u8>().ok()
    })]
    Register(u8),

    // Registrador indireto
    #[regex(r"[Rr][0-9]+\*", |lex| {
        let num_str = &lex.slice()[1..lex.slice().len()-1];
        num_str.parse::<u8>().ok()
    })]
    RegisterIndirect(u8),

    // Alias (variáveis)
    #[regex(r"![a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice()[1..].to_string())]
    Alias(String),

    // Labels
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*:", |lex| lex.slice()[..lex.slice().len()-1].to_string())]
    Label(String),

    // String char
    #[regex(r#""[^"]*""#, |lex| lex.slice()[1..lex.slice().len()-1].to_string())]
    CharString(String),

    // Instruções
    #[token("NOP", ignore(case))]
    Nop,
    #[token("HLT", ignore(case))]
    Hlt,
    #[token("MOV", ignore(case))]
    Mov,
    #[token("PHR", ignore(case))]
    Phr,
    #[token("PLR", ignore(case))]
    Plr,
    #[token("ADD", ignore(case))]
    Add,
    #[token("SUB", ignore(case))]
    Sub,
    #[token("MUL", ignore(case))]
    Mul,
    #[token("DIV", ignore(case))]
    Div,
    #[token("MOD", ignore(case))]
    Mod,
    #[token("INC", ignore(case))]
    Inc,
    #[token("DEC", ignore(case))]
    Dec,
    #[token("AND", ignore(case))]
    And,
    #[token("OR", ignore(case))]
    Or,
    #[token("XOR", ignore(case))]
    Xor,
    #[token("NOT", ignore(case))]
    Not,
    #[token("CMP", ignore(case))]
    Cmp,
    #[token("JMP", ignore(case))]
    Jmp,
    #[token("JZ", ignore(case))]
    Jz,
    #[token("JNZ", ignore(case))]
    Jnz,
    #[token("JN", ignore(case))]
    Jn,
    #[token("JNN", ignore(case))]
    Jnn,
    #[token("JC", ignore(case))]
    Jc,
    #[token("JNC", ignore(case))]
    Jnc,
    #[token("JSB", ignore(case))]
    Jsb,
    #[token("RSB", ignore(case))]
    Rsb,
    #[token("CLI", ignore(case))]
    Cli,
    #[token("SEI", ignore(case))]
    Sei,
    #[token("RSI", ignore(case))]
    Rsi,
    

    // Diretivas
    #[regex(r"\.[a-zA-Z]+", |lex| lex.slice()[1..].to_string())]
    Directive(String),

    // Identificadores para labels sem ':'
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Identifier(String),

    // Newline (importante para parsing)
    #[token("\n")]
    Newline,
}

pub struct Lexer<'a> {
    logos_lexer: logos::Lexer<'a, Token>,
    current_token: Option<Token>,
    line: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Self {
            logos_lexer: Token::lexer(input),
            current_token: None,
            line: 0,
        };
        lexer.advance();
        lexer
    }

    pub fn current(&self) -> &Option<Token> {
        &self.current_token
    }

    pub fn line(&self) -> usize {
        self.line
    }

    pub fn advance(&mut self) {
        let token = self.logos_lexer.next();
        match token {
            Some(res) => {
                match res {
                    Ok(tk) => {
                        if tk == Token::Newline {
                            self.line += 1;
                        }
                        self.current_token = Some(tk)
                    },
                    Err(e) => println!("Error: {:?}", e),
                }
            },
            None => self.current_token = None,
        }
    }

    pub fn consume(&mut self, expected: Token) -> Result<(), String> {
        if let Some(ref current) = self.current_token {
            if std::mem::discriminant(current) == std::mem::discriminant(&expected) {
                self.advance();
                Ok(())
            } else {
                Err(format!("Expected {:?}, found {:?} at line {}", expected, current, self.line))
            }
        } else {
            Err(format!("Expected {:?}, found EOF at line {}", expected, self.line))
        }
    }

    pub fn is_at_end(&self) -> bool {
        self.current_token.is_none()
    }
}