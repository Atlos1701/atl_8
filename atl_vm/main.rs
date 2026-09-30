mod cpu;
mod screen;
use std::env;
use std::fs;
use minifb::{Window, WindowOptions, Key};

const ARGERROR: &str = "usage: atl_vm [BOOT IMG] [DEBUG/N] [SCALE]";

fn main() {
    println!("ATL//VM @atlos06");
    println!("CONFIG INIT");
    let args: Vec<String> = env::args().collect();
    let path = args.get(1).expect(ARGERROR);
    let status = args.get(2).expect(ARGERROR);
    let scale = args.get(3).expect(ARGERROR).parse().expect(ARGERROR);

    let status_bool = match status.trim() {
        "DEBUG" => true,
        "N" => false,
        _ => panic!("Not a debug choice")
    };

    println!("CPU & RAM INIT");

    let mut cpu = cpu::Cpu::new(fs::read(path).expect("couldn't grab file"));

    println!("SCREEN INIT");

    let mut window = Window::new(
        "ATL_VM",
        32*scale,
        16*scale,
        WindowOptions::default(),
    ).expect("couldn't make a window");

    let mut screen = screen::Screen::new(scale);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let state = cpu.cycle(status_bool);

        screen.update(&cpu.ram[0xFE00..0xFEFF]);
        screen.convert();
        screen.scale(scale);
        screen.push(&mut window, scale);

        if state == cpu::Status::Terminate {
            println!("END: Closing");
            break
        }
    }
}
