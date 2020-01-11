pub mod drivers;

use std::time::Duration;
use std::mem::MaybeUninit;
use std::convert::TryFrom;
use std::thread;

use crate::rand;
use crate::parser;
use crate::instructions::Instruction;
use drivers::*;

pub struct VM {
    reg_snd: u8,
    reg_dt: u8,
    reg_sp: u8,
    reg_i: u16,
    reg_pc: u16,
    registers: MaybeUninit<[u8; 16]>,
    stack: MaybeUninit<[u16; 16]>,
    ram: MaybeUninit<[u8; 4096]>,
}

impl VM {
    pub const fn new() -> VM {
        VM {
            reg_snd: 0,
            reg_dt: 0,
            reg_sp: 0,
            reg_i: 0,
            reg_pc: 0,
            registers: MaybeUninit::uninit(),
            stack: MaybeUninit::uninit(),
            ram: MaybeUninit::uninit(),
        }
    }

    fn registers(&self) -> &[u8; 16] {
        unsafe { &*self.registers.as_ptr() }
    }

    fn registers_mut(&mut self) -> &mut [u8; 16] {
        unsafe { &mut *self.registers.as_mut_ptr() }
    }

    fn stack(&self) -> &[u16; 16] {
        unsafe { &*self.stack.as_ptr() }
    }

    fn stack_mut(&mut self) -> &mut [u16; 16] {
        unsafe { &mut *self.stack.as_mut_ptr() }
    }

    fn ram(&self) -> &[u8; 4096] {
        unsafe { &*self.ram.as_ptr() }
    }

    fn ram_mut(&mut self) -> &mut [u8; 4096] {
        unsafe { &mut *self.ram.as_mut_ptr() }
    }

    pub fn load<T: AsRef<[u8]>>(&mut self, program: T) {
        let prog = program.as_ref();
        self.reg_pc = 0x200;
        (&mut self.ram_mut()[0..80]).copy_from_slice(&FONT[..]);
        (&mut self.ram_mut()[0x200..0x200+prog.len()]).copy_from_slice(prog);
    }

    pub fn run<D, I, S>(&mut self, ctx: &mut Context<D, I, S>)
    where
        D: Display,
        I: Input,
        S: Sound,
    {
        let mut i = 0;
        loop {
            thread::sleep(CPU_DELAY);
            if i == DECREMENT {
                i = 0;
                let dt = self.reg_dt;
                let snd = self.reg_snd;
                self.reg_dt -= DECREMENT as u8;
                self.reg_snd -= DECREMENT as u8;
                if self.reg_dt > dt {
                    self.reg_dt = 0;
                }
                if self.reg_snd > snd {
                    self.reg_snd = 0;
                }
                if self.reg_snd == 0 {
                    ctx.beep_end();
                }
            }
            self.interpret_cycle(ctx);
            i += 1;
        }
    }

    fn interpret_cycle<D, I, S>(&mut self, ctx: &mut Context<D, I, S>)
    where
        D: Display,
        I: Input,
        S: Sound,
    {
        use Instruction::*;
        let inst = {
            let pc = self.reg_pc as usize;
            let ram = self.ram();
            [ram[pc], ram[pc+1]]
        };
        match parser::read(inst) {
            UNKNOWN(_) => self.reg_pc += 2,
            CLS => {
                ctx.clear();
                self.reg_pc += 2;
            },
            RET => {
                self.reg_sp -= 1;
                let sp = self.reg_sp as usize;
                self.reg_pc = self.stack()[sp];
            },
            JPA(addr) => self.reg_pc = addr,
            CALL(addr) => {
                let sp = self.reg_sp as usize;
                self.stack_mut()[sp] = self.reg_pc + 2;
                self.reg_sp += 1;
                self.reg_pc = addr;
            },
            SEI(reg, val) => {
                let reg = self.registers()[reg as usize];
                self.reg_pc += if reg == val {
                    4
                } else {
                    2
                };
            },
            SNEI(reg, val) => {
                let reg = self.registers()[reg as usize];
                self.reg_pc += if reg == val {
                    2
                } else {
                    4
                };
            },
            SER(x, y) => {
                let x = self.registers()[x as usize];
                let y = self.registers()[y as usize];
                self.reg_pc += if x == y {
                    4
                } else {
                    2
                };
            },
            LDI(reg, val) => {
                self.registers_mut()[reg as usize] = val;
                self.reg_pc += 2;
            },
            ADDI(reg, val) => {
                self.registers_mut()[reg as usize] += val;
                self.reg_pc += 2;
            },
            LDR(x, y) => {
                let y = self.registers()[y as usize];
                self.registers_mut()[x as usize] = y;
                self.reg_pc += 2;
            },
            ORR(x, y) => {
                let y = self.registers()[y as usize];
                self.registers_mut()[x as usize] |= y;
                self.reg_pc += 2;
            },
            ANDR(x, y) => {
                let y = self.registers()[y as usize];
                self.registers_mut()[x as usize] &= y;
                self.reg_pc += 2;
            },
            XORR(x, y) => {
                let y = self.registers()[y as usize];
                self.registers_mut()[x as usize] ^= y;
                self.reg_pc += 2;
            },
            ADDR(xx, y) => {
                let x = self.registers()[xx as usize] as u16;
                let y = self.registers()[y as usize] as u16;

                let z = x + y;
                let carry = ((z >> 8) != 0) as u8;
                let z = (z & 0xff) as u8;

                self.registers_mut()[xx as usize] = z;
                self.registers_mut()[0xf] = carry;

                self.reg_pc += 2;
            },
            SUBR(xx, y) => {
                let x = self.registers()[xx as usize];
                let y = self.registers()[y as usize];

                self.registers_mut()[0xf] = if x > y {
                    1
                } else {
                    0
                };

                self.registers_mut()[xx as usize] -= y;
                self.reg_pc += 2;
            },
            SHRR(xx, _) => {
                let x = self.registers()[xx as usize];
                self.registers_mut()[0xf] = x & 1;
                self.registers_mut()[xx as usize] = x >> 1;
                self.reg_pc += 2;
            },
            SUBNR(xx, y) => {
                let x = self.registers()[xx as usize];
                let y = self.registers()[y as usize];

                self.registers_mut()[0xf] = if y > x {
                    1
                } else {
                    0
                };

                self.registers_mut()[xx as usize] = y - x;
                self.reg_pc += 2;
            },
            SHLR(xx, _) => {
                let x = self.registers()[xx as usize];
                self.registers_mut()[0xf] = x & 0x80;
                self.registers_mut()[xx as usize] = x << 1;
                self.reg_pc += 2;
            },
            SNER(x, y) => {
                let x = self.registers()[x as usize];
                let y = self.registers()[y as usize];
                self.reg_pc += if x == y {
                    2
                } else {
                    4
                };
            },
            LDA(addr) => {
                self.reg_i = addr;
                self.reg_pc += 2;
            },
            JPAFAR(addr) => self.reg_pc = self.registers()[0] as u16 + addr,
            RND(reg, val) => {
                self.registers_mut()[reg as usize] = rand::byte() & val;
                self.reg_pc += 2;
            },
            DRW(x, y, n) => {
                let x = self.registers()[x as usize] as usize;
                let y = self.registers()[y as usize] as usize;

                // read N bytes from memory starting at I
                let i = self.reg_i as usize;
                let n = n as usize;
                let sprite = &self.ram()[i..i+n];

                // draw the sprite onto the screen, and check collisions
                self.registers_mut()[0xf] = ctx.draw(x, y, sprite) as u8;
                self.reg_pc += 2;
            },
            SKP(reg) => {
                let k = self.registers()[reg as usize];
                let k = input::Key::try_from(k).unwrap();
                let keys = ctx.poll_keyboard();
                self.reg_pc += if keys[k] {
                    4
                } else {
                    2
                };
            },
            SKNP(reg) => {
                let k = self.registers()[reg as usize];
                let k = input::Key::try_from(k).unwrap();
                let keys = ctx.poll_keyboard();
                self.reg_pc += if keys[k] {
                    2
                } else {
                    4
                };
            },
            LDTG(reg) => {
                self.registers_mut()[reg as usize] = self.reg_dt;
                self.reg_pc += 2;
            },
            LDK(reg) => {
                self.registers_mut()[reg as usize] = ctx.wait_key() as u8;
                self.reg_pc += 2;
            },
            LDTS(reg) => {
                self.reg_dt = self.registers()[reg as usize];
                self.reg_pc += 2;
            },
            LDSS(reg) => {
                ctx.beep_start();
                self.reg_snd = self.registers()[reg as usize];
                self.reg_pc += 2;
            },
            ADDA(reg) => {
                self.reg_i += self.registers()[reg as usize] as u16;
                self.reg_pc += 2;
            },
            LDDIG(dig) => {
                self.reg_i = (dig as u16) * 5_u16;
                self.reg_pc += 2;
            },
            LDBCD(_register) => {
                unimplemented!()
            },
            LDREGST(x) => {
                let off = self.reg_i as usize;
                let x = x as usize;
                for i in 0..x {
                    self.ram_mut()[off+i] = self.registers()[i];
                }
            },
            LDREGRD(x) => {
                let off = self.reg_i as usize;
                let x = x as usize;
                for i in 0..x {
                    self.registers_mut()[i] = self.ram_mut()[off+i];
                }
            },
        }
    }
}

static FONT: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, 0x20, 0x60, 0x20, 0x20, 0x70,
    0xF0, 0x10, 0xF0, 0x80, 0xF0, 0xF0, 0x10, 0xF0, 0x10, 0xF0,
    0x90, 0x90, 0xF0, 0x10, 0x10, 0xF0, 0x80, 0xF0, 0x10, 0xF0,
    0xF0, 0x80, 0xF0, 0x90, 0xF0, 0xF0, 0x10, 0x20, 0x40, 0x40,
    0xF0, 0x90, 0xF0, 0x90, 0xF0, 0xF0, 0x90, 0xF0, 0x10, 0xF0,
    0xF0, 0x90, 0xF0, 0x90, 0x90, 0xE0, 0x90, 0xE0, 0x90, 0xE0,
    0xF0, 0x80, 0x80, 0x80, 0xF0, 0xE0, 0x90, 0x90, 0x90, 0xE0,
    0xF0, 0x80, 0xF0, 0x80, 0xF0, 0xF0, 0x80, 0xF0, 0x80, 0x80,
];

pub const CPU_FREQ: u64 = 500;
pub const DELAY_TICK_FREQ: u64 = 60;
pub const CPU_DELAY: Duration = Duration::from_millis(1000 / CPU_FREQ);

const DECREMENT: i32 = (CPU_FREQ / DELAY_TICK_FREQ) as i32;
