use crate::assembler::definitions::Register;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Mnemonic(String),
    Register(Register),
    DereferencedRegister(Register),
    Literal(u16),
    Comma,
    Colon,
    LabelIdentifier(String),
    DotDirective(String),
    Comment(String),
}

#[derive(Debug, PartialEq, Clone)]
pub enum LexerError {
    InvalidCharacter(char),
    InvalidLiteralFormat(String),
    InvalidRegisterFormat(String),
    UnknownToken(String),
}

fn parse_register(s: &str) -> Result<(Token, &str), LexerError> {
    if !s.starts_with('R') && !s.starts_with('r') {
        return Err(LexerError::InvalidRegisterFormat(s.to_string()));
    }

    let mut chars = s.chars().peekable();
    chars.next(); 

    let mut num_str = String::new();
    while let Some(&c) = chars.peek() {
        if c.is_ascii_digit() {
            num_str.push(chars.next().unwrap());
        } else {
            break;
        }
    }

    if num_str.is_empty() {
        return Err(LexerError::InvalidRegisterFormat(s.to_string()));
    }

    let reg_id: u8 = num_str
        .parse()
        .map_err(|_| LexerError::InvalidRegisterFormat(s.to_string()))?;

    if reg_id > 15 {
        return Err(LexerError::InvalidRegisterFormat(
            format!("Invalid register ID: {}", reg_id),
        ));
    }

    let remaining_s_idx = 1 + num_str.len(); 

    if s.chars().nth(remaining_s_idx) == Some('*') {
        Ok((
            Token::DereferencedRegister(Register(reg_id)),
            &s[remaining_s_idx + 1..],
        ))
    } else {
        Ok((Token::Register(Register(reg_id)), &s[remaining_s_idx..]))
    }
}

fn parse_literal(s: &str) -> Result<(Token, &str), LexerError> {
    if s.starts_with('$') {
        let mut num_str = String::new();
        let mut chars = s.chars().peekable();
        chars.next(); 
        while let Some(&c) = chars.peek() {
            if c.is_ascii_digit() {
                num_str.push(chars.next().unwrap());
            } else {
                break;
            }
        }
        if num_str.is_empty() {
            return Err(LexerError::InvalidLiteralFormat(s.to_string()));
        }
        let val: u16 = num_str
            .parse()
            .map_err(|_| LexerError::InvalidLiteralFormat(s.to_string()))?;
        Ok((Token::Literal(val), &s[1 + num_str.len()..]))
    } else if s.starts_with('#') {
        let mut num_str = String::new();
        let mut chars = s.chars().peekable();
        chars.next(); 
        while let Some(&c) = chars.peek() {
            if c.is_ascii_hexdigit() {
                num_str.push(chars.next().unwrap());
            } else {
                break;
            }
        }
        if num_str.is_empty() {
            return Err(LexerError::InvalidLiteralFormat(s.to_string()));
        }
        let val: u16 = u16::from_str_radix(&num_str, 16)
            .map_err(|_| LexerError::InvalidLiteralFormat(s.to_string()))?;
        Ok((Token::Literal(val), &s[1 + num_str.len()..]))
    } else {
        Err(LexerError::InvalidLiteralFormat(s.to_string()))
    }
}

pub fn tokenize(line: &str) -> Result<Vec<Token>, LexerError> {
    let mut tokens = Vec::new();
    let mut current_pos = 0;
    let line_len = line.len();

    while current_pos < line_len {
        let mut current_char = line.chars().nth(current_pos).unwrap();

        while current_char.is_whitespace() {
            current_pos += 1;
            if current_pos >= line_len {
                return Ok(tokens);
            }
            current_char = line.chars().nth(current_pos).unwrap();
        }

        if current_char == ';' {
            let comment_str = &line[current_pos + 1..];
            tokens.push(Token::Comment(comment_str.trim_end().to_string()));
            return Ok(tokens); 
        }

        let remaining_line = &line[current_pos..];

        if current_char.to_ascii_uppercase() == 'R' {
             match parse_register(remaining_line) {
                Ok((token, rest)) => {
                    tokens.push(token);
                    current_pos = line_len - rest.len();
                    continue;
                }
                Err(LexerError::InvalidRegisterFormat(_)) => {}
                Err(e) => return Err(e),
            }
        }

        if current_char == '$' || current_char == '#' {
            match parse_literal(remaining_line) {
                Ok((token, rest)) => {
                    tokens.push(token);
                    current_pos = line_len - rest.len();
                    continue;
                }
                Err(LexerError::InvalidLiteralFormat(_)) => {}
                Err(e) => return Err(e),
            }
        }

        if current_char == ',' {
            tokens.push(Token::Comma);
            current_pos += 1;
            continue;
        }
        if current_char == ':' {
            if let Some(Token::LabelIdentifier(_)) = tokens.last() {
                 tokens.push(Token::Colon);
                 current_pos += 1;
                 continue;
            }
        }

        if current_char == '.' {
            let mut directive = String::from(".");
            let mut next_char_pos = current_pos + 1;
            while next_char_pos < line_len {
                let c = line.chars().nth(next_char_pos).unwrap();
                if c.is_alphanumeric() {
                    directive.push(c);
                    next_char_pos += 1;
                } else {
                    break;
                }
            }
            if directive.len() > 1 { 
                tokens.push(Token::DotDirective(directive));
                current_pos = next_char_pos;
                continue;
            } else {
                return Err(LexerError::UnknownToken(remaining_line.split_whitespace().next().unwrap_or("").to_string()));
            }
        }

        let mut identifier = String::new();
        let mut temp_pos = current_pos;
        while temp_pos < line_len {
            let c = line.chars().nth(temp_pos).unwrap();
            if c.is_alphanumeric() || c == '_' {
                identifier.push(c);
                temp_pos += 1;
            } else {
                break;
            }
        }

        if !identifier.is_empty() {
            if line.chars().nth(temp_pos) == Some(':') {
                tokens.push(Token::LabelIdentifier(identifier.to_uppercase()));
            } else {
                tokens.push(Token::Mnemonic(identifier.to_uppercase()));
            }
            current_pos = temp_pos;
            continue;
        }

        return Err(LexerError::UnknownToken(remaining_line.split_whitespace().next().unwrap_or("").to_string()));
    }
    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assembler::definitions::Register;

    #[test]
    fn test_tokenize_simple_instructions() {
        assert_eq!(tokenize("NOP"), Ok(vec![Token::Mnemonic("NOP".to_string())]));
        assert_eq!(tokenize("HLT"), Ok(vec![Token::Mnemonic("HLT".to_string())]));
        assert_eq!(tokenize("  RET  "), Ok(vec![Token::Mnemonic("RET".to_string())]));
    }

    #[test]
    fn test_tokenize_registers() {
        assert_eq!(tokenize("R0"), Ok(vec![Token::Register(Register(0))]));
        assert_eq!(tokenize("r15"), Ok(vec![Token::Register(Register(15))]));
        assert_eq!(tokenize("R7*"), Ok(vec![Token::DereferencedRegister(Register(7))]));
        assert_eq!(tokenize("  R3*  "), Ok(vec![Token::DereferencedRegister(Register(3))]));
    }

    #[test]
    fn test_tokenize_literals() {
        assert_eq!(tokenize("$123"), Ok(vec![Token::Literal(123)]));
        assert_eq!(tokenize("#FF"), Ok(vec![Token::Literal(0xFF)]));
        assert_eq!(tokenize("#0abC"), Ok(vec![Token::Literal(0x0ABC)]));
        assert_eq!(tokenize("  $0  "), Ok(vec![Token::Literal(0)]));
    }

    #[test]
    fn test_tokenize_instruction_with_registers() {
        assert_eq!(
            tokenize("MOV R1, R0"),
            Ok(vec![
                Token::Mnemonic("MOV".to_string()),
                Token::Register(Register(1)),
                Token::Comma,
                Token::Register(Register(0))
            ])
        );
    }

    #[test]
    fn test_tokenize_label_definition() {
        assert_eq!(
            tokenize("MY_LOOP: NOP"),
            Ok(vec![
                Token::LabelIdentifier("MY_LOOP".to_string()),
                Token::Colon,
                Token::Mnemonic("NOP".to_string())
            ])
        );
    }

    #[test]
    fn test_tokenize_directive() {
        assert_eq!(
            tokenize(".ORG #100"),
            Ok(vec![
                Token::DotDirective(".ORG".to_string()),
                Token::Literal(0x100)
            ])
        );
    }

    #[test]
    fn test_tokenize_line_with_comment() {
        assert_eq!(
            tokenize("MOV R0, $0 ; load zero"),
            Ok(vec![
                Token::Mnemonic("MOV".to_string()),
                Token::Register(Register(0)),
                Token::Comma,
                Token::Literal(0),
                Token::Comment("load zero".to_string())
            ])
        );
    }
    
    #[test]
    fn test_tokenize_error_invalid_register_format() {
        assert!(matches!(tokenize("R16"), Err(LexerError::InvalidRegisterFormat(_))));
    }

    #[test]
    fn test_tokenize_error_invalid_literal_format() {
        assert!(matches!(tokenize("$FF"), Err(LexerError::InvalidLiteralFormat(_))));
    }
}
