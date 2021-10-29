use crate::Instruction;
pub fn assemble(str: String) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::new();

    for i in str.lines() {
        if i.trim().is_empty() || i.starts_with(";") {
            continue;
        }
        let mut iter = i.split_whitespace();
        let op = iter.next().unwrap();
        let arg = iter.next();
        match op {
            "nop" => out.push(Instruction::Nop as u8),
            "hlt" => out.push(Instruction::Halt as u8),
            "push" => {
                let unwrapped_arg = arg.unwrap();
                if unwrapped_arg.starts_with("0x") {
                    let arg =
                        i64::from_str_radix(unwrapped_arg.trim_start_matches("0x"), 16).unwrap();
                    out.push(Instruction::Push as u8);
                    out.append(&mut arg.to_le_bytes().to_vec());
                } else {
                    let arg = i64::from_str_radix(unwrapped_arg, 10).unwrap();
                    out.push(Instruction::Push as u8);
                    out.append(&mut arg.to_le_bytes().to_vec());
                }
            }
            "pop" => out.push(Instruction::Pop as u8),
            "add" => out.push(Instruction::Add as u8),
            "mul" => out.push(Instruction::Mul as u8),
            "sub" => out.push(Instruction::Sub as u8),
            "div" => out.push(Instruction::Div as u8),
            "jmp" => out.push(Instruction::Jump as u8),
            "loadu8" => out.push(Instruction::LoadU8 as u8),
            "storeu8" => out.push(Instruction::StoreU8 as u8),
            "swap" => out.push(Instruction::Swap as u8),
            "dup" => out.push(Instruction::Dupe as u8),
            "dupp" => {
                let unwrapped_arg = arg.unwrap();
                if unwrapped_arg.starts_with("0x") {
                    let arg =
                        i64::from_str_radix(unwrapped_arg.trim_start_matches("0x"), 16).unwrap();
                    out.push(Instruction::DupeAt as u8);
                    out.append(&mut arg.to_le_bytes().to_vec());
                } else {
                    let arg = i64::from_str_radix(unwrapped_arg, 10).unwrap();
                    out.push(Instruction::DupeAt as u8);
                    out.append(&mut arg.to_le_bytes().to_vec());
                }
            }
            "int" => {
                let unwrapped_arg = arg.unwrap();
                if unwrapped_arg.starts_with("0x") {
                    let arg =
                        u8::from_str_radix(unwrapped_arg.trim_start_matches("0x"), 16).unwrap();
                    out.push(Instruction::Interrupt as u8);
                    out.push(arg);
                } else {
                    let arg = u8::from_str_radix(unwrapped_arg, 10).unwrap();
                    out.push(Instruction::Interrupt as u8);
                    out.push(arg);
                }
            }
            "call" => out.push(Instruction::Call as u8),
            "ret" => out.push(Instruction::Ret as u8),
            _ => panic!("Unknown instruction"),
        }
    }

    out
}
