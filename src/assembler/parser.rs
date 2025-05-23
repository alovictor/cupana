use crate::assembler::definitions::{InstructionType, Operand, ParsedLine, Register};
use crate::assembler::lexer::Token;
use std::iter::Peekable;
use std::slice::Iter;

#[derive(Debug, PartialEq, Clone)]
pub enum ParserError {
    UnexpectedToken(Token),
    MissingOperand,
    InvalidOperandType(Token),
    UnknownInstruction(String),
    TrailingTokens(Vec<Token>),
    EmptyInput,
    InvalidDirective(String),
    InvalidLabel(String),
    MissingComma(Token),
    ExpectedMnemonic,
    ExpectedRegister,
    ExpectedLiteral,
    ExpectedLabelIdentifier,
    ExpectedColon,
    ExpectedSpecificToken(String), 
}

fn parse_operand(token: &Token) -> Result<Operand, ParserError> {
    match token {
        Token::Register(r) => Ok(Operand::Register(*r)),
        Token::DereferencedRegister(r) => Ok(Operand::DereferencedRegister(*r)),
        Token::Literal(val) => Ok(Operand::Literal(*val)),
        _ => Err(ParserError::InvalidOperandType(token.clone())),
    }
}

fn expect_token<'a, I>( 
    tokens_iter: &mut Peekable<I>,
    expected: &Token, 
    error: ParserError,
) -> Result<(), ParserError>
where
    I: Iterator<Item = &'a Token>,
{
    match tokens_iter.next() {
        Some(token) if token == expected => Ok(()),
        Some(_token) => Err(error.clone()), 
        None => Err(error.clone()), 
    }
}

fn parse_instruction(mnemonic: &str, tokens_iter: &mut Peekable<Iter<Token>>) -> Result<InstructionType, ParserError> {
    match mnemonic.to_uppercase().as_str() {
        "NOP" => Ok(InstructionType::Nop),
        "HLT" => Ok(InstructionType::Hlt),
        "RET" => Ok(InstructionType::Ret),
        "MOV" | "ADD" | "SUB" | "MUL" | "DIV" | "MOD" | "AND" | "OR" | "XOR" | "CMP" => {
            let op1_token = tokens_iter.next().ok_or(ParserError::MissingOperand)?;
            let op1 = parse_operand(op1_token)?;

            expect_token(tokens_iter, &Token::Comma, ParserError::MissingComma(op1_token.clone()))?;

            let op2_token = tokens_iter.next().ok_or(ParserError::MissingOperand)?;
            let op2 = parse_operand(op2_token)?;

            match mnemonic.to_uppercase().as_str() {
                "MOV" => Ok(InstructionType::Mov { dest: op1, src: op2 }),
                "ADD" => Ok(InstructionType::Add { dest: op1, src: op2 }),
                "SUB" => Ok(InstructionType::Sub { dest: op1, src: op2 }),
                "MUL" => Ok(InstructionType::Mul { dest: op1, src: op2 }),
                "DIV" => Ok(InstructionType::Div { dest: op1, src: op2 }),
                "MOD" => Ok(InstructionType::Mod { dest: op1, src: op2 }),
                "AND" => Ok(InstructionType::And { dest: op1, src: op2 }),
                "OR" => Ok(InstructionType::Or { dest: op1, src: op2 }),
                "XOR" => Ok(InstructionType::Xor { dest: op1, src: op2 }),
                "CMP" => Ok(InstructionType::Cmp { op1, op2 }),
                _ => unreachable!(),
            }
        }
        "INC" | "DEC" | "NOT" => {
            let op_token = tokens_iter.next().ok_or(ParserError::MissingOperand)?;
            let op = parse_operand(op_token)?;
            match op {
                Operand::Register(_) | Operand::DereferencedRegister(_) => {}
                _ => return Err(ParserError::InvalidOperandType(op_token.clone())),
            }
            match mnemonic.to_uppercase().as_str() {
                "INC" => Ok(InstructionType::Inc { reg: op }),
                "DEC" => Ok(InstructionType::Dec { reg: op }),
                "NOT" => Ok(InstructionType::Not { reg: op }),
                _ => unreachable!(),
            }
        }
        "JMP" | "JZ" | "JNZ" | "JN" | "JNN" | "JC" | "JNC" => {
            let target_token = tokens_iter.next().ok_or(ParserError::MissingOperand)?;
            let target = match target_token {
                Token::LabelIdentifier(name) => Operand::Label(name.clone()),
                Token::Mnemonic(name) => Operand::Label(name.clone()), // Treat Mnemonic as Label here
                Token::Register(_) | Token::Literal(_) => parse_operand(target_token)?,
                _ => return Err(ParserError::InvalidOperandType(target_token.clone())),
            };
            // Validation of target operand type is implicitly handled by the match and parse_operand
            match mnemonic.to_uppercase().as_str() {
                "JMP" => Ok(InstructionType::Jmp { target }),
                "JZ" => Ok(InstructionType::Jz { target }),
                "JNZ" => Ok(InstructionType::Jnz { target }),
                "JN" => Ok(InstructionType::Jn { target }),
                "JNN" => Ok(InstructionType::Jnn { target }),
                "JC" => Ok(InstructionType::Jc { target }),
                "JNC" => Ok(InstructionType::Jnc { target }),
                _ => unreachable!(),
            }
        }
        "CALL" => {
            let target_token = tokens_iter.next().ok_or(ParserError::MissingOperand)?;
            let target = match target_token {
                Token::LabelIdentifier(name) => Operand::Label(name.clone()),
                Token::Mnemonic(name) => Operand::Label(name.clone()), // Treat Mnemonic as Label here
                Token::Literal(_) => parse_operand(target_token)?,
                _ => return Err(ParserError::InvalidOperandType(target_token.clone())),
            };
            // Validation of target operand type is implicitly handled by the match and parse_operand
            Ok(InstructionType::Call { target })
        }
        _ => Err(ParserError::UnknownInstruction(mnemonic.to_string())),
    }
}

pub fn parse_line(tokens: &[Token]) -> Result<ParsedLine, ParserError> {
    let filtered_tokens: Vec<Token> = tokens
        .iter()
        .filter(|t| !matches!(t, Token::Comment(_)))
        .cloned()
        .collect();

    if filtered_tokens.is_empty() {
        return Ok(ParsedLine::Empty);
    }
    
    let mut it = filtered_tokens.iter().peekable(); 

    if let Some(Token::LabelIdentifier(name)) = it.peek().cloned() {
        // Look ahead without consuming to check for colon
        if filtered_tokens.get(1).map_or(false, |t_ref| matches!(t_ref, Token::Colon)) {
            it.next(); // Consume LabelIdentifier
            it.next(); // Consume Colon
            if it.peek().is_none() { // If nothing follows "LABEL:", it's a pure label definition
                return Ok(ParsedLine::LabelDefinition(name.clone()));
            }
            // If something follows, `it` is now positioned after "LABEL:", ready to parse instruction/directive
        }
        // If not "LABEL:" or if "LABEL" is not followed by ":", `it` is not advanced here.
        // The next match block will try to parse from the current position of `it`.
        // This means if it was "LABEL" (no colon), it will be treated as a mnemonic/identifier.
        // If it was "LABEL:" followed by something, `it` is correctly positioned past the colon.
    }


    match it.peek().cloned() {
        Some(&Token::DotDirective(ref directive_name)) => {
            it.next(); 
            let mut args = Vec::new();
            let mut expect_operand = true;
            loop {
                match it.peek() {
                    Some(&&Token::Comma) => {
                        if expect_operand {
                            return Err(ParserError::UnexpectedToken(Token::Comma));
                        }
                        it.next(); 
                        expect_operand = true;
                    }
                    Some(token_ref_ref) => { 
                        let token_ref = *token_ref_ref;
                        if !expect_operand {
                            return Err(ParserError::MissingComma(token_ref.clone()));
                        }
                        args.push(parse_operand(token_ref)?);
                        it.next(); 
                        expect_operand = false;
                    }
                    None => break, 
                }
            }
            if expect_operand && !args.is_empty() {
                return Err(ParserError::MissingOperand);
            }
            if it.peek().is_some() {
                return Err(ParserError::TrailingTokens(it.cloned().collect()));
            }
            Ok(ParsedLine::Directive { name: directive_name.clone(), args })
        }
        Some(&Token::Mnemonic(ref mnemonic_str)) => {
            let mnemonic_clone = mnemonic_str.clone();
            it.next(); 
            let instruction = parse_instruction(&mnemonic_clone, &mut it)?;
            if it.peek().is_some() {
                let trailing: Vec<Token> = it.cloned().collect();
                return Err(ParserError::TrailingTokens(trailing));
            }
            Ok(ParsedLine::Instruction(instruction))
        }
        Some(Token::LabelIdentifier(name_str)) => {
            // This is reached if LabelIdentifier was not followed by a colon (e.g. "MYLABEL NOP")
            // OR if the label parsing logic decided it wasn't a label for some reason and `it` wasn't advanced.
            // This should be treated as a mnemonic.
            let mnemonic_clone = name_str.clone();
            it.next(); 
            let instruction = parse_instruction(&mnemonic_clone, &mut it)?;
            if it.peek().is_some() {
                let trailing: Vec<Token> = it.cloned().collect();
                return Err(ParserError::TrailingTokens(trailing));
            }
            Ok(ParsedLine::Instruction(instruction))
        }
        Some(token) => {
             Err(ParserError::UnexpectedToken(token.clone()))
        }
        None => {
            // This implies that all tokens were consumed by the label parsing logic,
            // but it didn't result in a standalone `ParsedLine::LabelDefinition` (which would have returned early).
            // This typically means the line was just "LABEL:" and was handled, or the line is now empty.
            // If `filtered_tokens` was not empty, but `it` is now empty, it means everything was consumed.
            // If `_label` was Some (from a previous design), it would imply "LABEL:" was found.
            // Given the current logic, if we reach here and `it` is empty, it means the line was fully processed
            // as a label, or it was empty to begin with.
            Ok(ParsedLine::Empty) 
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assembler::definitions::{Register, InstructionType, Operand, ParsedLine};
    use crate::assembler::lexer::Token;

    fn make_tokens(line: &str) -> Vec<Token> {
        vec![]
    }

    #[test]
    fn test_parse_empty_line() {
        assert_eq!(parse_line(&[]), Ok(ParsedLine::Empty));
        assert_eq!(parse_line(&[Token::Comment("test".to_string())]), Ok(ParsedLine::Empty));
    }

    #[test]
    fn test_parse_label_definition() {
        let tokens = vec![
            Token::LabelIdentifier("MY_LABEL".to_string()),
            Token::Colon,
        ];
        assert_eq!(parse_line(&tokens), Ok(ParsedLine::LabelDefinition("MY_LABEL".to_string())));
    }

    #[test]
    fn test_parse_label_definition_with_comment() {
        let tokens = vec![
            Token::LabelIdentifier("MY_LABEL".to_string()),
            Token::Colon,
            Token::Comment("this is a label".to_string()),
        ];
        assert_eq!(parse_line(&tokens), Ok(ParsedLine::LabelDefinition("MY_LABEL".to_string())));
    }
    
    #[test]
    fn test_parse_error_label_trailing_tokens() {
        let tokens_invalid_after_label = vec![
            Token::LabelIdentifier("MY_LABEL".to_string()),
            Token::Colon,
            Token::Register(Register(1)), 
        ];
        assert!(matches!(parse_line(&tokens_invalid_after_label), Err(ParserError::UnexpectedToken(Token::Register(Register(1))))));
    }

    #[test]
    fn test_parse_nop() {
        assert_eq!(parse_line(&[Token::Mnemonic("NOP".to_string())]), Ok(ParsedLine::Instruction(InstructionType::Nop)));
    }
    
    // ... (rest of the tests from previous correct version) ...
    #[test]
    fn test_parse_hlt() {
        let tokens = vec![Token::Mnemonic("HLT".to_string())];
        assert_eq!(parse_line(&tokens), Ok(ParsedLine::Instruction(InstructionType::Hlt)));
    }
    
    #[test]
    fn test_parse_ret() {
        let tokens = vec![Token::Mnemonic("RET".to_string())];
        assert_eq!(parse_line(&tokens), Ok(ParsedLine::Instruction(InstructionType::Ret)));
    }

    #[test]
    fn test_parse_mov_reg_reg() {
        let tokens = vec![
            Token::Mnemonic("MOV".to_string()),
            Token::Register(Register(1)),
            Token::Comma,
            Token::Register(Register(2)),
        ];
        assert_eq!(
            parse_line(&tokens),
            Ok(ParsedLine::Instruction(InstructionType::Mov {
                dest: Operand::Register(Register(1)),
                src: Operand::Register(Register(2)),
            }))
        );
    }

    #[test]
    fn test_parse_mov_reg_lit() {
        let tokens = vec![
            Token::Mnemonic("MOV".to_string()),
            Token::Register(Register(0)),
            Token::Comma,
            Token::Literal(123),
        ];
        assert_eq!(
            parse_line(&tokens),
            Ok(ParsedLine::Instruction(InstructionType::Mov {
                dest: Operand::Register(Register(0)),
                src: Operand::Literal(123),
            }))
        );
    }
    
    #[test]
    fn test_parse_mov_reg_deref() {
        let tokens = vec![
            Token::Mnemonic("MOV".to_string()),
            Token::Register(Register(3)),
            Token::Comma,
            Token::DereferencedRegister(Register(4)),
        ];
        assert_eq!(
            parse_line(&tokens),
            Ok(ParsedLine::Instruction(InstructionType::Mov {
                dest: Operand::Register(Register(3)),
                src: Operand::DereferencedRegister(Register(4)),
            }))
        );
    }

     #[test]
    fn test_parse_mov_deref_lit() { 
        let tokens = vec![
            Token::Mnemonic("MOV".to_string()),
            Token::DereferencedRegister(Register(1)),
            Token::Comma,
            Token::Literal(0xAB),
        ];
        assert_eq!(
            parse_line(&tokens),
            Ok(ParsedLine::Instruction(InstructionType::Mov {
                dest: Operand::DereferencedRegister(Register(1)),
                src: Operand::Literal(0xAB),
            }))
        );
    }

    #[test]
    fn test_parse_add_reg_reg() {
        let tokens = vec![
            Token::Mnemonic("ADD".to_string()),
            Token::Register(Register(0)),
            Token::Comma,
            Token::Register(Register(1)),
        ];
        assert_eq!(
            parse_line(&tokens),
            Ok(ParsedLine::Instruction(InstructionType::Add {
                dest: Operand::Register(Register(0)),
                src: Operand::Register(Register(1)),
            }))
        );
    }
    
    #[test]
    fn test_parse_add_reg_lit() {
        let tokens = vec![
            Token::Mnemonic("ADD".to_string()),
            Token::Register(Register(0)),
            Token::Comma,
            Token::Literal(10),
        ];
        assert_eq!(
            parse_line(&tokens),
            Ok(ParsedLine::Instruction(InstructionType::Add {
                dest: Operand::Register(Register(0)),
                src: Operand::Literal(10),
            }))
        );
    }

    #[test]
    fn test_parse_inc_reg() {
        let tokens = vec![
            Token::Mnemonic("INC".to_string()),
            Token::Register(Register(0)),
        ];
        assert_eq!(
            parse_line(&tokens),
            Ok(ParsedLine::Instruction(InstructionType::Inc {
                reg: Operand::Register(Register(0)),
            }))
        );
    }

    #[test]
    fn test_parse_dec_reg() {
        let tokens = vec![
            Token::Mnemonic("DEC".to_string()),
            Token::Register(Register(1)),
        ];
        assert_eq!(
            parse_line(&tokens),
            Ok(ParsedLine::Instruction(InstructionType::Dec {
                reg: Operand::Register(Register(1)),
            }))
        );
    }
    
    #[test]
    fn test_parse_jmp_reg() {
         let tokens = vec![
            Token::Mnemonic("JMP".to_string()),
            Token::Register(Register(2)),
        ];
        assert_eq!(
            parse_line(&tokens),
            Ok(ParsedLine::Instruction(InstructionType::Jmp {
                target: Operand::Register(Register(2)),
            }))
        );
    }

    #[test]
    fn test_parse_jmp_lit() {
        let tokens = vec![
            Token::Mnemonic("JMP".to_string()),
            Token::Literal(0xC000),
        ];
        assert_eq!(
            parse_line(&tokens),
            Ok(ParsedLine::Instruction(InstructionType::Jmp {
                target: Operand::Literal(0xC000),
            }))
        );
    }
    
    #[test]
    fn test_parse_call_lit() {
        let tokens = vec![
            Token::Mnemonic("CALL".to_string()),
            Token::Literal(0xD000),
        ];
        assert_eq!(
            parse_line(&tokens),
            Ok(ParsedLine::Instruction(InstructionType::Call {
                target: Operand::Literal(0xD000),
            }))
        );
    }

    #[test]
    fn test_parse_directive_org() {
        let tokens = vec![
            Token::DotDirective(".ORG".to_string()),
            Token::Literal(0x1000),
        ];
        assert_eq!(
            parse_line(&tokens),
            Ok(ParsedLine::Directive {
                name: ".ORG".to_string(),
                args: vec![Operand::Literal(0x1000)],
            })
        );
    }
    
    #[test]
    fn test_parse_directive_db_multiple_args() {
        let tokens = vec![
            Token::DotDirective(".DB".to_string()),
            Token::Literal(10),
            Token::Comma,
            Token::Literal(20),
            Token::Comma,
            Token::Register(Register(1)), 
        ];
         assert_eq!(
            parse_line(&tokens),
            Ok(ParsedLine::Directive {
                name: ".DB".to_string(),
                args: vec![Operand::Literal(10), Operand::Literal(20), Operand::Register(Register(1))],
            })
        );
    }

    #[test]
    fn test_parse_error_unknown_mnemonic() {
        let tokens = vec![Token::Mnemonic("FOO".to_string()), Token::Register(Register(0))];
        match parse_line(&tokens) {
            Ok(ParsedLine::Instruction(InstructionType::UnknownInstruction { name })) => {
                assert_eq!(name, "FOO");
            }
            _ => panic!("Expected UnknownInstruction error"),
        }
    }
    
    #[test]
    fn test_parse_error_label_identifier_as_mnemonic() {
        let tokens = vec![Token::LabelIdentifier("MYLABEL".to_string()), Token::Register(Register(0))];
         match parse_line(&tokens) {
            Ok(ParsedLine::Instruction(InstructionType::UnknownInstruction { name })) => {
                assert_eq!(name, "MYLABEL");
            }
            _ => panic!("Expected UnknownInstruction for label-like mnemonic"),
        }
    }

    #[test]
    fn test_parse_error_missing_operands_mov() {
        let tokens = vec![Token::Mnemonic("MOV".to_string()), Token::Register(Register(0))];
        assert!(matches!(parse_line(&tokens), Err(ParserError::MissingComma(_)) | Err(ParserError::MissingOperand)));
    }
    
    #[test]
    fn test_parse_error_missing_operands_add() {
        let tokens = vec![Token::Mnemonic("ADD".to_string())];
        assert!(matches!(parse_line(&tokens), Err(ParserError::MissingOperand)));
    }

    #[test]
    fn test_parse_error_too_many_operands_hlt() {
        let tokens = vec![Token::Mnemonic("HLT".to_string()), Token::Register(Register(0))];
        assert!(matches!(parse_line(&tokens), Err(ParserError::TrailingTokens(_))));
    }
    
    #[test]
    fn test_parse_error_too_many_operands_mov() {
         let tokens = vec![
            Token::Mnemonic("MOV".to_string()),
            Token::Register(Register(0)),
            Token::Comma,
            Token::Register(Register(1)),
            Token::Comma, 
            Token::Register(Register(2)), 
        ];
        assert!(matches!(parse_line(&tokens), Err(ParserError::TrailingTokens(_))));
    }

    #[test]
    fn test_parse_error_inc_literal() {
        let tokens = vec![Token::Mnemonic("INC".to_string()), Token::Literal(10)];
        assert!(matches!(parse_line(&tokens), Err(ParserError::InvalidOperandType(Token::Literal(10)))));
    }
    
    #[test]
    fn test_parse_error_call_register() { 
        let tokens = vec![Token::Mnemonic("CALL".to_string()), Token::Register(Register(0))];
        assert!(matches!(parse_line(&tokens), Err(ParserError::InvalidOperandType(Token::Register(Register(0))))));
    }

    #[test]
    fn test_parse_error_missing_comma_mov() {
        let tokens = vec![
            Token::Mnemonic("MOV".to_string()),
            Token::Register(Register(0)),
            Token::Register(Register(1)),
        ];
        assert!(matches!(parse_line(&tokens), Err(ParserError::MissingComma(Token::Register(Register(0))))));
    }

    #[test]
    fn test_parse_error_unexpected_comma_hlt() {
        let tokens = vec![Token::Mnemonic("HLT".to_string()), Token::Comma];
        assert!(matches!(parse_line(&tokens), Err(ParserError::TrailingTokens(_))));
    }
    
    #[test]
    fn test_parse_error_invalid_label_format_colon_first() {
        let tokens = vec![Token::Colon, Token::LabelIdentifier("MY_LABEL".to_string())];
        assert!(matches!(parse_line(&tokens), Err(ParserError::UnexpectedToken(Token::Colon))));
    }

    #[test]
    fn test_parse_error_directive_missing_arg() {
        let tokens = vec![Token::DotDirective(".ORG".to_string())]; 
        assert!(matches!(parse_line(&tokens), Err(ParserError::MissingOperand)));
    }

    #[test]
    fn test_parse_error_directive_invalid_arg_type() {
        let tokens = vec![
            Token::DotDirective(".ORG".to_string()),
            Token::Register(Register(1)), 
        ];
        assert_eq!(
            parse_line(&tokens),
            Ok(ParsedLine::Directive {
                name: ".ORG".to_string(),
                args: vec![Operand::Register(Register(1))],
            })
        );
    }
    
    #[test]
    fn test_parse_error_directive_trailing_comma() {
        let tokens = vec![
            Token::DotDirective(".DB".to_string()),
            Token::Literal(10),
            Token::Comma,
        ];
        assert!(matches!(parse_line(&tokens), Err(ParserError::MissingOperand)));
    }

    #[test]
    fn test_parse_line_label_then_instruction() {
        let tokens = vec![
            Token::LabelIdentifier("START".to_string()),
            Token::Colon,
            Token::Mnemonic("MOV".to_string()),
            Token::Register(Register(1)),
            Token::Comma,
            Token::Register(Register(2)),
        ];
        assert_eq!(
            parse_line(&tokens),
            Ok(ParsedLine::Instruction(InstructionType::Mov {
                dest: Operand::Register(Register(1)),
                src: Operand::Register(Register(2)),
            }))
        );
    }
    
    #[test]
    fn test_parse_line_label_then_directive() {
        let tokens = vec![
            Token::LabelIdentifier("DATA_AREA".to_string()),
            Token::Colon,
            Token::DotDirective(".ORG".to_string()),
            Token::Literal(0x2000),
        ];
         assert_eq!(
            parse_line(&tokens),
            Ok(ParsedLine::Directive {
                name: ".ORG".to_string(),
                args: vec![Operand::Literal(0x2000)],
            })
        );
    }

    #[test]
    fn test_parse_error_label_then_invalid() {
        let tokens = vec![
            Token::LabelIdentifier("BAD".to_string()),
            Token::Colon,
            Token::Comma, 
        ];
        assert!(matches!(parse_line(&tokens), Err(ParserError::UnexpectedToken(Token::Comma))));
    }

    #[test]
    fn test_parse_instruction_with_leading_whitespace_tokens_is_handled_by_lexer() {
        let tokens = vec![
            Token::Comment("leading comment".to_string()),
            Token::Mnemonic("NOP".to_string()),
        ];
         assert_eq!(parse_line(&tokens), Ok(ParsedLine::Instruction(InstructionType::Nop)));
    }
}
