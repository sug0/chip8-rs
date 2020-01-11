pub trait Sound {
    fn beep_start(&mut self);
    fn beep_end(&mut self);
}

impl Sound for () {
    fn beep_start(&mut self) {}
    fn beep_end(&mut self) {}
}
