use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(skip r"[ \t\n\f]+")]
pub enum Token {
    // Comentários
    #[regex(r";[^\n]*", logos::skip)]
    Comment,

    // Números literais
    #[regex(r"\$[0-9]+", |lex| lex.slice()[1..].parse::<u16>())]
    DecimalLiteral(u16),

    #[regex(r"#[0-9a-fA-F]+", |lex| u16::from_str_radix(&lex.slice()[1..], 16))]
    HexLiteral(u16),

    // Registradores
    #[regex(r"[Rr][0-9]+", |lex| {
        let num_str = &lex.slice()[1..];
        num_str.parse::<u8>()
    })]
    Register(u8),

    // Registrador indireto
    #[regex(r"[Rr][0-9]+\*", |lex| {
        let num_str = &lex.slice()[1..lex.slice().len()-1];
        num_str.parse::<u8>()
    })]
    RegisterIndirect(u8),

    // Alias (variáveis)
    #[regex(r"![a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice()[1..].to_string())]
    Alias(String),

    // Labels
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*:", |lex| lex.slice()[..lex.slice().len()-1].to_string())]
    Label(String),

    // Instruções
    #[token("NOP", ignore(case))]
    Nop,
    #[token("HLT", ignore(case))]
    Hlt,
    #[token("MOV", ignore(case))]
    Mov,
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
    #[token("CALL", ignore(case))]
    Call,
    #[token("RET", ignore(case))]
    Ret,

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
            line: 1,
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
        self.current_token = self.logos_lexer.next();
        if let Some(Token::Newline) = self.current_token {
            self.line += 1;
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