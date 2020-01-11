pub type Address = u16;
pub type Immediate = u8;
pub type Register = u8;

// order as in 3.1 of:
// http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#3.1
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum Instruction {
    UNKNOWN(u16),
    CLS,
    RET,
    JPA(Address),
    CALL(Address),
    SEI(Register, Immediate),
    SNEI(Register, Immediate),
    SER(Register, Register),
    LDI(Register, Immediate),
    ADDI(Register, Immediate),
    LDR(Register, Register),
    ORR(Register, Register),
    ANDR(Register, Register),
    XORR(Register, Register),
    ADDR(Register, Register),
    SUBR(Register, Register),
    SHRR(Register, Register),
    SUBNR(Register, Register),
    SHLR(Register, Register),
    SNER(Register, Register),
    LDA(Address),
    JPAFAR(Address),
    RND(Register, Immediate),
    DRW(Register, Register, Immediate),
    SKP(Register),
    SKNP(Register),
    LDTG(Register),
    LDK(Register),
    LDTS(Register),
    LDSS(Register),
    ADDA(Register),
    LDDIG(Immediate),
    LDBCD(Register),
    LDREGST(Register),
    LDREGRD(Register),
}
