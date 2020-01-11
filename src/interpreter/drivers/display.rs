pub const DISPLAY_WIDTH: usize = 64;
pub const DISPLAY_HEIGHT: usize = 32;
pub const DISPLAY_SIZE: usize = DISPLAY_WIDTH * DISPLAY_HEIGHT;

pub trait Display {
    fn draw(&mut self, x: usize, y: usize, sprite: &[u8]) -> bool;
    fn clear(&mut self);
}

pub struct TerminalDisplay {
    buf: String,
    scr: [u8; DISPLAY_SIZE],
}

impl Display for () {
    fn draw(&mut self, _x: usize, _y: usize, _sprite: &[u8]) -> bool {
        false
    }

    fn clear(&mut self) {}
}

impl TerminalDisplay {
    pub const fn new() -> Self {
        let buf = String::new();
        let scr = [0_u8; DISPLAY_SIZE];
        TerminalDisplay { buf, scr }
    }
}

impl Display for TerminalDisplay {
    fn draw(&mut self, x: usize, y: usize, sprite: &[u8]) -> bool {
        let mut collision = false;
        let n = sprite.len();

        for j in 0..n {
            for i in 0..8 {
                let yj = (y + j) % DISPLAY_HEIGHT;
                let xi = (x + i) % DISPLAY_WIDTH;

                if (sprite[j] & (0x80 >> i)) != 0 {
                    if self.scr[yj*DISPLAY_WIDTH + xi] == 1 {
                        collision = true;
                    }
                    self.scr[yj*DISPLAY_WIDTH + xi] ^= 1;
                }
            }
        }


        for y in 0..DISPLAY_HEIGHT {
            self.buf.clear();
            for x in 0..DISPLAY_WIDTH {
                if self.scr[y*DISPLAY_WIDTH + x] == 0 {
                    self.buf.push(' ')
                } else {
                    self.buf.push('█')
                }
            }
            println!("{}{}", termion::cursor::Goto(0, y as u16), &self.buf)
        }

        collision
    }

    fn clear(&mut self) {
        for x in self.scr.iter_mut() {
            *x = 0;
        }
        print!("{}", termion::clear::All);
    }
}
