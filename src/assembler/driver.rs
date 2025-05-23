use crate::assembler::lexer::{tokenize, Token};
use crate::assembler::parser::parse_line;
use crate::assembler::codegen::{generate_instruction_code};
use crate::assembler::definitions::{InstructionType, Operand, ParsedLine};
use crate::assembler::AssemblerError;
use std::collections::HashMap;
use std::fs;
use std::io::{BufWriter, Write};

pub fn calculate_instruction_size(
    instruction: &InstructionType,
    _symbol_table: &HashMap<String, u16>, 
) -> Result<u16, String> { // Returns size in BYTES
    let word_size = match instruction {
        InstructionType::Nop | InstructionType::Hlt | InstructionType::Ret => Ok(1),
        InstructionType::Mov { dest, src } => match (dest, src) {
            (Operand::Register(_), Operand::Register(_)) => Ok(1),
            (Operand::Register(_), Operand::Literal(_)) => Ok(2),
            (Operand::Register(_), Operand::DereferencedRegister(_)) => Ok(1),
            (Operand::DereferencedRegister(_), Operand::Register(_)) => Ok(1),
            (Operand::Literal(_), Operand::Register(_)) => Ok(2), 
            _ => Err(format!("Unsupported MOV for size: {:?}, {:?}", dest, src)),
        },
        InstructionType::Add { dest, src } |
        InstructionType::Sub { dest, src } |
        InstructionType::Mul { dest, src } |
        InstructionType::Div { dest, src } |
        InstructionType::Mod { dest, src } => match (dest, src) {
            (Operand::Register(_), Operand::Register(_)) => Ok(1),
            (Operand::Register(_), Operand::Literal(_)) => Ok(2),
            (Operand::Literal(_), Operand::Register(_)) => Ok(2), 
            _ => Err(format!("Unsupported ALU for size: {:?}, {:?}", dest, src)),
        },
        InstructionType::Inc { .. } | InstructionType::Dec { .. } | InstructionType::Not { .. } => Ok(1),
        InstructionType::And { dest, src } |
        InstructionType::Or { dest, src } |
        InstructionType::Xor { dest, src } => match (dest,src) {
            (Operand::Register(_), Operand::Register(_)) => Ok(1),
            _=> Err(format!("Unsupported LOGIC for size: {:?}, {:?}", dest, src)),
        },
        InstructionType::Cmp { op1, op2 } => match (op1, op2) {
            (Operand::Register(_), Operand::Register(_)) => Ok(1),
            (Operand::Register(_), Operand::Literal(_)) => Ok(2),
            _ => Err(format!("Unsupported CMP for size: {:?}, {:?}", op1, op2)),
        },
        InstructionType::Jmp { target } |
        InstructionType::Jz { target } |
        InstructionType::Jnz { target } |
        InstructionType::Jn { target } |
        InstructionType::Jnn { target } |
        InstructionType::Jc { target } |
        InstructionType::Jnc { target } => match target {
            Operand::Register(_) => Ok(1),
            Operand::Literal(_) | Operand::Label(_) => Ok(2), 
            _ => Err(format!("Unsupported JMP target for size: {:?}", target)),
        },
        InstructionType::Call { target } => match target {
            Operand::Literal(_) | Operand::Label(_) => Ok(2), 
             _ => Err(format!("Unsupported CALL target for size: {:?}", target)),
        },
    };
    word_size.map(|ws| ws * 2) // Convert word count to byte count
}

pub fn assemble_source(source_code: &str) -> Result<Vec<u16>, AssemblerError> {
    let mut symbol_table: HashMap<String, u16> = HashMap::new();
    let mut current_address: u16 = 0;
    let lines: Vec<&str> = source_code.lines().collect();

    for (line_idx, line_content) in lines.iter().enumerate() {
        let line_number = line_idx + 1;
        
        let tokens = tokenize(line_content)
            .map_err(|e| AssemblerError::Lexer { 
                line_number, 
                content: line_content.trim().to_string(), 
                kind: e 
            })?;
        
        if tokens.is_empty() || tokens.iter().all(|t| matches!(t, Token::Comment(_))) {
            continue;
        }

        let parsed_line = parse_line(&tokens)
            .map_err(|e| AssemblerError::Parser { 
                line_number, 
                content: line_content.trim().to_string(), 
                kind: e 
            })?;

        match parsed_line {
            ParsedLine::LabelDefinition(name) => {
                if symbol_table.contains_key(&name) {
                    return Err(AssemblerError::Driver { 
                        line_number: Some(line_number), 
                        message: format!("Duplicate label '{}'", name) 
                    });
                }
                symbol_table.insert(name, current_address);
            }
            ParsedLine::Instruction(instr) => {
                let size = calculate_instruction_size(&instr, &symbol_table)
                    .map_err(|e_msg| AssemblerError::Driver{ 
                        line_number: Some(line_number), 
                        message: format!("Error calculating size: {} for instr {:?}", e_msg, instr) 
                    })?;
                current_address += size;
            }
            ParsedLine::Directive { name, args } => {
                if name.to_uppercase() == ".ORG" {
                    if args.len() != 1 {
                        return Err(AssemblerError::Driver { 
                            line_number: Some(line_number), 
                            message: format!(".ORG directive expects 1 argument, got {}", args.len()) 
                        });
                    }
                    match args.first() {
                        Some(Operand::Literal(val)) => {
                            current_address = *val; // .ORG address is a byte address
                        }
                        _ => return Err(AssemblerError::Driver { 
                            line_number: Some(line_number), 
                            message: ".ORG argument must be a literal value".to_string() 
                        }),
                    }
                }
            }
            ParsedLine::Empty | ParsedLine::Comment => { /* Do nothing */ }
        }
    }

    current_address = 0;
    let mut machine_code: Vec<u16> = Vec::new();

    for (line_idx, line_content) in lines.iter().enumerate() {
        let line_number = line_idx + 1;
        let tokens = tokenize(line_content)
            .map_err(|e| AssemblerError::Lexer { 
                line_number, 
                content: line_content.trim().to_string(), 
                kind: e 
            })?;

        if tokens.is_empty() || tokens.iter().all(|t| matches!(t, Token::Comment(_))) {
            continue;
        }

        let parsed_line = parse_line(&tokens)
            .map_err(|e| AssemblerError::Parser { 
                line_number, 
                content: line_content.trim().to_string(), 
                kind: e 
            })?;

        match parsed_line {
            ParsedLine::Instruction(instr) => {
                let instruction_string_for_error = format!("{:?}", instr); 
                let generated_words = generate_instruction_code(&instr, &symbol_table, current_address)
                    .map_err(|e| AssemblerError::CodeGen { 
                        line_number, 
                        content: instruction_string_for_error, 
                        kind: e 
                    })?;
                
                machine_code.extend(generated_words.iter());
                current_address += generated_words.len() as u16;
            }
            ParsedLine::Directive { name, args } => {
                if name.to_uppercase() == ".ORG" {
                     if args.len() != 1 {
                        return Err(AssemblerError::Driver { 
                            line_number: Some(line_number), 
                            message: format!(".ORG directive expects 1 argument, got {}", args.len()) 
                        });
                    }
                    match args.first() {
                        Some(Operand::Literal(val)) => {
                            if *val < current_address && !machine_code.is_empty() {
                                return Err(AssemblerError::Driver { 
                                    line_number: Some(line_number), 
                                    message: format!(".ORG directive to a previous address {} is not supported when code has been generated at {}", val, current_address) 
                                });
                            }
                            if *val > current_address {
                                machine_code.resize(machine_code.len() + (*val - current_address) as usize, 0x0000);
                            }
                            current_address = *val;
                        }
                        _ => return Err(AssemblerError::Driver { 
                            line_number: Some(line_number), 
                            message: ".ORG argument must be a literal value (Pass 2)".to_string() 
                        }),
                    }
                }
            }
            ParsedLine::LabelDefinition(_, ..) | ParsedLine::Empty | ParsedLine::Comment => { /* Do nothing */ }
        }
    }
    Ok(machine_code)
}

pub fn assemble_file(input_path: &str, output_path: &str) -> Result<(), AssemblerError> {
    let source_code = fs::read_to_string(input_path)
        .map_err(|e| AssemblerError::Io { 
            message: format!("Error reading input file '{}': {}", input_path, e) 
        })?;

    let machine_code = assemble_source(&source_code)?;

    let file = fs::File::create(output_path)
        .map_err(|e| AssemblerError::Io { 
            message: format!("Error creating output file '{}': {}", output_path, e) 
        })?;
    let mut writer = BufWriter::new(file);

    for word in machine_code {
        writer.write_all(&word.to_le_bytes())
            .map_err(|e| AssemblerError::Io { 
                message: format!("Error writing to output file '{}': {}", output_path, e) 
            })?;
    }

    writer.flush()
        .map_err(|e| AssemblerError::Io { 
            message: format!("Error flushing output file '{}': {}", output_path, e) 
        })?;
    Ok(())
}
