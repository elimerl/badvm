use crate::Instruction;

use super::parser::ASTNode;

pub fn emit(root_node: ASTNode) -> Vec<u8> {
    let mut instrs = Vec::new();
    root_node.emit(&mut instrs);
    instrs.push(Instruction::Push as u8);
    instrs.append(&mut (1 as i64).to_le_bytes().to_vec());
    // instrs.push(Instruction::Push as u8);
    // instrs.append(&mut (1 as i64).to_le_bytes().to_vec());
    // instrs.push(Instruction::Push as u8);
    // instrs.append(&mut (0xbf9d3d as i64).to_le_bytes().to_vec());
    // instrs.push(Instruction::Interrupt as u8);
    // instrs.append(&mut (1 as i64).to_le_bytes().to_vec());
    // instrs.push(Instruction::Push as u8);
    // instrs.append(&mut (0 as i64).to_le_bytes().to_vec());
    // instrs.push(Instruction::Jump as u8);
    instrs
}
