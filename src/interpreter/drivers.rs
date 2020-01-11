pub mod display;
pub mod input;
pub mod sound;

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
