pub mod lexer;

use lexer::CupanaLexer;

pub struct CupanaAssembler {
    lexer: CupanaLexer,
}

impl CupanaAssembler {
    pub fn new(path: &str) -> Self {
        Self {
            lexer: CupanaLexer::new(path),
        }
    }

    pub fn assemble(&mut self) -> Vec<u8> {
        self.lexer.lex();
        for tok in self.lexer.get_output() {
            println!("{:?}", tok);
        }
        vec![]
    }
}
