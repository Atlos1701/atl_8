use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path_s = args.get(1).expect("usage: atl_comp [SOURCE FILE] [TARGET FILE]");
    let path_t = args.get(2).expect("usage: atl_comp [SOURCE FILE] [TARGET FILE]");

    let source_str = fs::read_to_string(format!("{path_s}")).expect("couldn't load from file {path_s}");

    let source_array: Vec<&str> = source_str.split('\n').collect();

    let mut target_buffer: Vec<u8> = Vec::new();

    let mut label_names: Vec<&str> = Vec::new();
    let mut label_addr: Vec<usize> = Vec::new();

    let mut jmp_names: Vec<&str> = Vec::new();
    let mut jmp_addr: Vec<usize> = Vec::new();

    for line in 0..source_array.len() {
        let mut line_array: Vec<&str> = source_array[line].split(' ').collect();

        for i in 0..line_array.len() {
            line_array[i] = line_array[i].trim()
        }
    }

    for line in 0..source_array.len() {
        let mut line_array: Vec<&str> = source_array[line].split(' ').collect();

        for i in 0..line_array.len() {
            line_array[i] = line_array[i].trim()
        }

        let opcode: Option<Vec<u8>> = match line_array.as_slice() {
            ["END"] => Some(vec!(0x00)),
            ["MOV", t1, arg0, t2, arg1] => {
                match [*t1, *t2] {
                    ["r", "r"] => {
                        Some(vec!(0x01, parser(arg0), parser(arg1)))
                    }
                    ["r", "m"] => {
                        Some(vec!(0x02, parser(arg0), parser(arg1)))
                    }
                    ["r", "n"] => {
                        Some(vec!(0x03, parser(arg0), parser(arg1)))
                    }
                    ["m", "r"] => {
                        Some(vec!(0x04, parser(arg0), parser(arg1)))
                    }
                    ["m", "m"] => {
                        Some(vec!(0x05, parser(arg0), parser(arg1)))
                    }
                    ["m", "n"] => {
                        Some(vec!(0x06, parser(arg0), parser(arg1)))
                    }
                    ["l", "l"] => {
                        Some(vec!(0x07, parser(arg0), parser(arg1)))
                    }
                    _ => {
                        panic!("MOV at line {line}+1: type match does not exist")
                    }
                }
            }
            ["PSH", arg0] => {
                Some(vec!(0x08, parser(arg0)))
            }
            ["POP", arg0] => {
                Some(vec!(0x09, parser(arg0)))
            }
            ["ADD", arg0, t1, arg1] => {
                match *t1 {
                    "r" => {
                        Some(vec!(0x0A, parser(arg0), parser(arg1)))
                    }
                    "n" => {
                        Some(vec!(0x0B, parser(arg0), parser(arg1)))
                    }
                    _ => {
                        panic!("ADD at line {line}+1: type match does not exist")
                    }
                }

            }
            ["SUB", arg0, t1, arg1] => {
                match *t1 {
                    "r" => {
                        Some(vec!(0x0C, parser(arg0), parser(arg1)))
                    }
                    "n" => {
                        Some(vec!(0x0D, parser(arg0), parser(arg1)))
                    }
                    _ => {
                        panic!("SUB at line {line}+1: type match does not exist")
                    }
                }
            }
            ["JMP", arg0, arg1] => {
                Some(vec!(0x0E, parser(arg0), parser(arg1)))
            }

            ["JMP", arg0] => {
                jmp_names.push(arg0);
                jmp_addr.push(target_buffer.len());
                None
            }
            [label, ":"] => {
                label_names.push(label);
                label_addr.push(target_buffer.len()+0x100);
                None
            }

            _ => None
        };

        if let Some(mut op) = opcode {
            target_buffer.append(&mut op)
        }
    }

    for j in 0..jmp_names.len() {
        for l in 0..label_names.len() {
            if jmp_names[j] == label_names[l] {
                let a = label_addr[l];
                let lo: u8 = (a & 0b0000_0000_1111_1111) as u8;
                let hi: u8 = ((a & 0b1111_1111_0000_0000) >> 8) as u8;
                println!("JMP {} set to go to label at {a}", jmp_names[j]);
                target_buffer.insert(jmp_addr[j], 0x0E);
                target_buffer.insert(jmp_addr[j] + 1, hi);
                target_buffer.insert(jmp_addr[j] + 2, lo);
            }
        }
    }

    fs::write(format!("{path_t}"), target_buffer).expect("couldn't write to file");
}

fn parser(input: &str) -> u8 {
    let ret = match input.parse() {
        Ok(n) => n,
        Err(e) => {println!("{input} is not a number: {e}"); panic!("")}
    };

    ret
}
