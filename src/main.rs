extern crate coarsetime;
extern crate getopts;
extern crate minifb;

use std::fs::File;
use std::io::{Read, Write};
use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::{env, thread};

use coarsetime::{Duration, Instant, Updater};

use getopts::Options;
use minifb::{Key, Window, WindowOptions};
const WIDTH: usize = 64;
const HEIGHT: usize = 64;
mod vm;
use vm::VM;
mod asm;
use crate::compiler::compile;
use crate::vm::DisplayInfo;
use asm::assemble;
mod compiler;
#[path = "instr.rs"]
pub mod instr;
use instr::Instruction;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    match args[1].as_str() {
        "vm" => {
            opts.optflag("w", "window", "open window");
        }
        "asm" => {}
        "disasm" => {}
        "cc" => {}
        _ => {
            println!("HELP MESSAGE");
            return Ok(());
        }
    }
    let matches = opts.parse(&args[2..])?;
    match args[1].as_str() {
        "vm" => {
            let mut window: Option<Window> = Option::None;

            if matches.opt_present("w") {
                window = Option::Some(Window::new(
                    "Test - ESC to exit",
                    WIDTH,
                    HEIGHT,
                    WindowOptions {
                        scale: minifb::Scale::X8,
                        ..Default::default()
                    },
                )?);
            }
            let mut code: Vec<u8> = Vec::new();
            let mut file = File::open(env::current_dir()?.join("test.bin"))?;
            file.read_to_end(&mut code)?;
            let mut vm = VM::new(
                code,
                vec![0; WIDTH * HEIGHT],
                DisplayInfo {
                    width: WIDTH,
                    height: HEIGHT,
                },
            );

            match window {
                None => loop {
                    vm.step()?;
                    if vm.paused {
                        break;
                    }
                },
                Some(mut window) => {
                    let (sender, reciever) = channel::<Vec<u32>>();
                    let alive = Arc::new(Mutex::<bool>::new(true));
                    thread::spawn(move || {
                        let _updater = Updater::new(2).start().unwrap();
                        let mut last_tick = Instant::now();
                        loop {
                            match vm.step() {
                                Ok(()) => {}
                                Err(err) => {
                                    println!("{:?}", err);
                                    break;
                                }
                            }
                            if vm.paused {
                                println!("paused {:?}", vm);
                                break;
                            }
                            if last_tick.elapsed_since_recent() > Duration::from_millis(16) {
                                let pixels = vm.framebuffer.clone();
                                sender.send(pixels).unwrap();
                                sleep(std::time::Duration::from_millis(16));
                            }
                            last_tick = Instant::recent();
                        }
                        println!("VM crashed");
                    });
                    // Limit to max ~60 fps update rate
                    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
                    while window.is_open()
                        && !window.is_key_down(Key::Escape)
                        && *alive.lock().unwrap()
                    {
                        let fb = reciever.recv()?;
                        window.update_with_buffer(&fb, WIDTH, HEIGHT)?;
                    }
                }
            }
        }
        "asm" => {
            let mut file = File::open(env::current_dir().unwrap().join("test.asm")).unwrap();
            let mut str = String::new();
            file.read_to_string(&mut str).unwrap();

            match File::create(env::current_dir().unwrap().join("../vm/test.bin")) {
                Err(_) => panic!("Could not create file"),
                Ok(mut file) => {
                    file.write_all(&assemble(str)).unwrap();
                }
            }
        }
        "cc" => {
            let mut file = File::open(env::current_dir().unwrap().join("test.c")).unwrap();
            let mut code = String::new();
            file.read_to_string(&mut code).unwrap();
            println!("{}", code);
            match File::create(env::current_dir().unwrap().join("../vm/test.bin")) {
                Err(_) => panic!("Could not create file"),
                Ok(mut file) => {
                    file.write_all(&compile(code)).unwrap();
                }
            }
        }
        "disasm" => {
            println!("Disassembly");
            let mut code: Vec<u8> = Vec::new();
            let mut file = File::open(env::current_dir()?.join("test.bin"))?;
            file.read_to_end(&mut code)?;
            let vm = VM::new(
                code,
                vec![0; WIDTH * HEIGHT],
                DisplayInfo {
                    width: WIDTH,
                    height: HEIGHT,
                },
            );
            println!("{:?}", vm);
        }
        _ => println!("Unrecognized command"),
    }

    Ok(())
}
