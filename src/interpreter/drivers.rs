pub use display::Display;
pub use input::Input;
pub use sound::Sound;

pub struct Context<D, I, S> {
    display: D,
    input: I,
    sound: S,
}

impl<D, I, S> Context<D, I, S>
where
    D: Display,
    I: Input,
    S: Sound,
{
    pub fn new(display: D, input: I, sound: S) -> Self {
        Context { display, input, sound }
    }
}

impl<D: Display, I, S> Display for Context<D, I, S> {
    fn draw(&mut self, x: usize, y: usize, sprite: &[u8]) -> bool {
        self.display.draw(x, y, sprite)
    }

    fn clear(&mut self) {
        self.display.clear()
    }
}

impl<D, I: Input, S> Input for Context<D, I, S> {
    fn poll_keyboard(&mut self) -> input::KeySet {
        self.input.poll_keyboard()
    }

    fn wait_key(&mut self) -> input::Key {
        self.input.wait_key()
    }
}

impl<D, I, S: Sound> Sound for Context<D, I, S> {
    fn beep_start(&mut self) {
        self.sound.beep_start()
    }

    fn beep_end(&mut self) {
        self.sound.beep_end()
    }
}

pub mod display {
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
}

pub mod input {
    use std::ops;
    use std::mem;
    use std::convert::TryFrom;

    #[repr(u8)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
    pub enum Key {
        Zero,
        One,
        Two,
        Three,
        Four,
        Five,
        Six,
        Seven,
        Eight,
        Nine,
        A,
        B,
        C,
        D,
        E,
        F,
    }

    impl TryFrom<u8> for Key {
        type Error = &'static str;

        fn try_from(k: u8) -> Result<Key, Self::Error> {
            if k < 16 {
                unsafe { Ok(mem::transmute(k)) }
            } else {
                Err("key out of range")
            }
        }
    }

    #[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
    pub struct KeySet([bool; 16]);

    impl ops::Index<Key> for KeySet {
        type Output = bool;

        fn index(&self, k: Key) -> &bool {
            &self.0[k as usize]
        }
    }

    pub trait Input {
        fn poll_keyboard(&mut self) -> KeySet;
        fn wait_key(&mut self) -> Key;
    }

    impl Input for () {
        fn poll_keyboard(&mut self) -> KeySet {
            KeySet([false; 16])
        }

        fn wait_key(&mut self) -> Key {
            loop {}
        }
    }
}

pub mod sound {
    pub trait Sound {
        fn beep_start(&mut self);
        fn beep_end(&mut self);
    }

    impl Sound for () {
        fn beep_start(&mut self) {}
        fn beep_end(&mut self) {}
    }
}
