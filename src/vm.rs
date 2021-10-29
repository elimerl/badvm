use num_enum::TryFromPrimitiveError;
use std::{convert::TryInto, error::Error, fmt, mem::size_of};
type VMResult<T> = std::result::Result<T, VMError>;
use super::Instruction;
use ansi_term::Colour::*;
// Define our error types. These may be customized for our error handling cases.
// Now we will be able to write our own errors, defer to an underlying error
// implementation, or do something in between.
#[derive(Debug, Clone)]
pub struct VMError {
    pub message: String,
    pub address: usize,
}

impl fmt::Display for VMError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "VMError {} at {:04x}", self.message, self.address)
    }
}
impl Error for VMError {
    fn description(&self) -> &str {
        "VMError"
    }
}
pub struct VM {
    pub memory: Vec<u8>,
    pub framebuffer: Vec<u32>,
    pub stack: Vec<i64>,
    pub paused: bool,
    pub display: DisplayInfo,
    pub call_stack: Vec<StackFrame>,
    pc: usize,
}
pub struct StackFrame {
    pub return_addr: usize,
}
impl StackFrame {
    pub fn new(return_addr: usize) -> StackFrame {
        StackFrame { return_addr }
    }
}
pub struct DisplayInfo {
    pub width: usize,
    pub height: usize,
}

impl VM {
    pub fn new(code: Vec<u8>, framebuffer: Vec<u32>, display: DisplayInfo) -> VM {
        let len = code.len();
        let mem = [code, Vec::with_capacity(65355 - len)].concat();
        VM {
            memory: mem,
            framebuffer,
            stack: Vec::new(),
            paused: false,
            display,
            call_stack: Vec::new(),
            pc: 0,
        }
    }
    pub fn step(self: &mut VM) -> VMResult<()> {
        let vm = self;
        vm.execute()?;
        Ok(())
    }
    pub fn stop(self: &mut VM) {
        self.paused = true;
    }

    fn execute(self: &mut VM) -> VMResult<()> {
        let mut vm = self;
        if vm.pc >= vm.memory.len() {
            return Err(VMError {
                message: "PC out of bounds".to_string(),
                address: vm.pc,
            });
        }
        let instr_result: Result<Instruction, TryFromPrimitiveError<Instruction>> =
            vm.memory[vm.pc].try_into();
        match instr_result {
            Ok(instr) => {
                match instr {
                    Instruction::Nop => {}
                    Instruction::Halt => {
                        vm.stop();
                        return Ok(());
                    }
                    Instruction::Push => {
                        vm.stack.push(i64::from_le_bytes(
                            vm.memory[vm.pc + 1..vm.pc + 9].try_into().unwrap(),
                        ));
                        vm.pc += size_of::<i64>();
                    }
                    Instruction::Add => {
                        let val1 = vm.stack.pop().unwrap();
                        let val2 = vm.stack.pop().unwrap();
                        vm.stack.push(val1 + val2);
                    }
                    Instruction::Mul => {
                        let val2 = vm.stack.pop().unwrap();
                        let val1 = vm.stack.pop().unwrap();

                        vm.stack.push(val1 * val2);
                    }
                    Instruction::Sub => {
                        let val2 = vm.stack.pop().unwrap();
                        let val1 = vm.stack.pop().unwrap();

                        vm.stack.push(val2 - val1);
                    }
                    Instruction::Div => {
                        let val2 = vm.stack.pop().unwrap();
                        let val1 = vm.stack.pop().unwrap();
                        vm.stack.push(val2 / val1);
                    }
                    Instruction::Jump => {
                        let dest = vm.stack.pop().unwrap();
                        vm.pc = (dest - 1).clamp(0, i64::MAX) as usize;
                    }
                    Instruction::Pop => {
                        vm.stack.pop();
                    }
                    Instruction::LoadU8 => {
                        let addr = vm.stack.pop().unwrap();
                        let val = vm.memory[addr as usize];
                        vm.stack.push(val as i64);
                    }
                    Instruction::StoreU8 => {
                        let addr = vm.stack.pop().unwrap();
                        let val = vm.stack.pop().unwrap();
                        vm.set_memory(addr as usize, val as u8);
                    }

                    Instruction::Swap => {
                        let val2 = vm.stack.pop().unwrap();
                        let val1 = vm.stack.pop().unwrap();
                        vm.stack.push(val2);
                        vm.stack.push(val1);
                    }
                    Instruction::Dupe => {
                        vm.stack.push(vm.stack[vm.stack.len() - 1]);
                    }
                    Instruction::DupeAt => {
                        let val = vm.stack[((vm.stack.len() as i64)
                            - i64::from_le_bytes(
                                vm.memory[vm.pc + 1..vm.pc + 9].try_into().unwrap(),
                            )) as usize];
                        vm.stack.push(val);
                    }
                    Instruction::Interrupt => {
                        let interrupt =
                            u8::from_le_bytes(vm.memory[vm.pc + 1..vm.pc + 2].try_into().unwrap());
                        vm.pc += size_of::<u8>();
                        match interrupt {
                            0x0 => {
                                vm.framebuffer[vm.stack.pop().unwrap() as usize] =
                                    vm.stack.pop().unwrap() as u32;
                            }
                            0x1 => {
                                let color = vm.stack.pop().expect("color not on stack");
                                let y = vm.stack.pop().expect("y position not on stack") as usize;
                                let x = vm.stack.pop().expect("x position not on stack") as usize;
                                vm.framebuffer[(y * vm.display.width + x) as usize] = color as u32;
                            }
                            _ => {
                                return Err(VMError {
                                    message: "Unknown interrupt".to_string(),
                                    address: vm.pc,
                                });
                            }
                        }
                    }
                    Instruction::Call => {
                        let dest = vm.stack.pop().expect("Address on the stack to call");
                        vm.call_stack.push(StackFrame::new(vm.pc));
                        vm.pc = (dest).clamp(0, i64::MAX).try_into().unwrap();
                    }
                    Instruction::Ret => {
                        vm.call_stack
                            .pop()
                            .expect("Cannot return when call stack is empty");
                    }
                }
                if instr != Instruction::Jump && instr != Instruction::Call {
                    vm.pc += 1;
                }
                Ok(())
            }
            Err(_) => Err(VMError {
                message: "Invalid instruction".to_string(),
                address: vm.pc,
            }),
        }
    }
    pub fn set_memory(self: &mut VM, addr: usize, val: u8) {
        if addr >= 0x8000 && addr < 0x9000 {
            self.framebuffer[addr - 0x8000] =
                val as u32 + ((val as u32) << 8) + ((val as u32) << 16) + ((val as u32) << 24);
        } else {
            self.memory[addr] = val;
        }
    }
}
impl fmt::Display for VM {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "VM")
    }
}
impl fmt::Debug for VM {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "VM")?;
        writeln!(f, "  pc: {}", self.pc)?;
        writeln!(f, "  stack: {:?}", self.stack)?;
        writeln!(f, "  paused: {}", self.paused)?;
        writeln!(f, "Debugger")?;
        for i in 0..self.memory.len() {
            if self.pc == i {
                write!(f, "{}  →  ", Red.paint(format!("0x{:04x}", i)))?;
            } else {
                write!(f, "{}     ", Blue.paint(format!("0x{:04x}", i)))?;
            }
            let v = self.memory[i];
            let instruction: Result<Instruction, TryFromPrimitiveError<Instruction>> = v.try_into();
            match instruction {
                Err(_) => {
                    writeln!(f, "{:02x}", v)?;
                }
                Ok(x) => {
                    let comment = get_instr_comment(&x);
                    writeln!(
                        f,
                        "{:?} {}",
                        x,
                        Green.paint("# ".to_string() + &comment).to_string()
                    )?;
                }
            }
        }

        Ok(())
    }
}
fn get_instr_comment(instr: &Instruction) -> String {
    match instr {
        Instruction::Push => "Push constant onto stack",
        Instruction::Add => "Add two numbers popped off the stack",
        Instruction::Mul => "Multiply two numbers popped off the stack",
        Instruction::Sub => "Subtract two numbers popped off the stack",
        Instruction::Div => "Divide two numbers popped off the stack",
        Instruction::Halt => "Halt execution",
        _ => "No comment",
    }
    .to_string()
}
