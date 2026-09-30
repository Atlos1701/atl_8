pub struct Cpu {
    r: [u8; 8],

    pc: u16,
    sp: u8,

    f: [bool; 8],

    pub ram: [u8; 0xFFFF],
}

pub enum I {
    END,
    MOV(u8),
    PSH,
    POP,
    ADD(u8),
    SUB(u8),
    JMP,
}

#[derive(PartialEq)]
pub enum Status {
    Done,
    Terminate,
    Crash,
}

impl Cpu {
    pub fn new(boot: Vec<u8>) -> Self {

        let mut ret = Cpu {
            r: [0; 8],

            pc: 0x0100,
            sp: 0x00,

            f: [false; 8],

            ram: [0; 0xFFFF],
        };

        ret.ram[0x100..boot.len()+0x100].copy_from_slice(&boot);

        ret
    }
    pub fn cycle(&mut self, debug: bool) -> Status {
        let opcode = self.read();
        self.pc += 1;
        let instruction = self.decode(opcode);

        match instruction {
            I::END => Status::Terminate,
            I::MOV(v) => {
                let arg0 = self.read();
                self.pc += 1;
                let arg1 = self.read();
                self.pc += 1;
                match v {
                    0 => {
                        self.r[arg0 as usize] = self.r[arg1 as usize]
                    }
                    1 => {
                        self.r[arg0 as usize] = self.ram[arg1 as usize]
                    }
                    2 => {
                        self.r[arg0 as usize] = arg1
                    }
                    3 => {
                        self.ram[arg0 as usize] = self.r[arg1 as usize]
                    }
                    4 => {
                        self.ram[arg0 as usize] = self.ram[arg1 as usize]
                    }
                    5 => {
                        self.ram[arg0 as usize] = arg1
                    }
                    6 => {
                        self.ram[(((arg0 as u16) << 8) + arg1 as u16) as usize] = self.r[0]
                    }
                    _ => panic!("OOB variant of MOV")
                }

                if debug {
                    println!("DEBUG: MOV({v}) {arg0} <- {arg1}");
                }

                Status::Done
            }
            I::PSH => {
                let arg0 = self.read();
                self.pc += 1;

                let stack_addr = 0xFEFF + self.sp as u16;

                self.ram[stack_addr as usize] = self.r[arg0 as usize];

                if debug {
                    println!("DEBUG: PSH r{arg0} (={}) sp: {}", self.r[arg0 as usize], self.sp);
                }

                self.sp += 1;

                Status::Done
            }

            I::POP => {
                let arg0 = self.read();
                self.pc += 1;

                self.sp -= 1;

                let stack_addr = 0xFEFF + self.sp as u16;

                self.r[arg0 as usize] = self.ram[stack_addr as usize];

                if debug {
                    println!("DEBUG: POP r{arg0} sp: {} (={})", self.sp, self.ram[stack_addr as usize]);
                }

                Status::Done
            }
            I::ADD(v) => {
                let arg0 = self.read();
                self.pc += 1;
                let arg1 = self.read();
                self.pc += 1;

                let n1 = match v {
                    0 => self.r[arg1 as usize],
                    1 => arg1,
                    _ => panic!("OOB variant of ADD")
                };

                let mut result: usize = (self.r[arg0 as usize] + n1) as usize;

                if result >= 256 {
                    result -= 255;
                    self.f[0] = true;
                }
                self.r[arg0 as usize] = result as u8;

                if debug {
                    println!("DEBUG: ADD({v}) r{arg0} + {arg1} = {}", self.r[arg0 as usize]);
                }

                Status::Done
            }
            I::SUB(v) => {
                let arg0 = self.read();
                self.pc += 1;
                let arg1 = self.read();
                self.pc += 1;

                let n1 = match v {
                    0 => self.r[arg1 as usize],
                    1 => arg1,
                    _ => panic!("OOB variant of ADD")
                };

                let mut result: f64 = (self.r[arg0 as usize] - n1) as f64;

                if result < 0.0 {
                    result += 255f64 + result;
                    self.f[0] = true;
                }
                self.r[arg0 as usize] = result as u8;

                if debug {
                    println!("DEBUG: SUB({v}) r{arg0} - {arg1} = {}", self.r[arg0 as usize]);
                }

                Status::Done
            }
            I::JMP => {
                let arg0 = self.read();
                self.pc += 1;
                let arg1 = self.read();
                self.pc += 1;

                self.pc = ((arg0 as u16) << 8) + arg1 as u16;

                if debug {
                    println!("DEBUG: JMP Jumping to {}", self.pc)
                }

                Status::Done
            }
        }
    }
    fn read(&self) -> u8 {
        self.ram[self.pc as usize]
    }

    fn decode(&self, input: u8) -> I {
        match input {
            0x00 => I::END,
            0x01 => I::MOV(0),
            0x02 => I::MOV(1),
            0x03 => I::MOV(2),
            0x04 => I::MOV(3),
            0x05 => I::MOV(4),
            0x06 => I::MOV(5),
            0x07 => I::MOV(6),
            0x08 => I::PSH,
            0x09 => I::POP,
            0x0A => I::ADD(0),
            0x0B => I::ADD(1),
            0x0C => I::SUB(0),
            0x0D => I::SUB(1),
            0x0E => I::JMP,
            e => todo!("Unimplemented opcode: {e}")
        }
    }
}
