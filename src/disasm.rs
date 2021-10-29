use super::Instruction;

fn disassemble(code: &[u8]) {
    for i in code.iter() {
        println!("{:02x}", i);
    }
}
