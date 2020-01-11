mod instructions;
mod interpreter;
mod parser;
mod rand;

use std::io::{self, Read, BufReader};
use interpreter::{
    VM,
    drivers::Context,
    drivers::display::TerminalDisplay,
};

fn main() {
    let data = {
        let stdin = io::stdin();
        let mut stdin_handle = stdin.lock();
        let mut handle = BufReader::new(&mut stdin_handle);

        let mut data = Vec::new();
        handle.read_to_end(&mut data)
            .expect("failed to read stdin data");

        data
    };

    let mut disp = TerminalDisplay::new();
    let mut ctx = Context::new(disp, (), ());
    let mut vm = VM::new();

    vm.load(data);
    vm.run(&mut ctx);
}
