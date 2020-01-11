use crate::instructions::Instruction;

pub fn read(buf: [u8; 2]) -> Instruction {
    let i = u16::from_be_bytes([buf[0], buf[1]]);
    match i {
        0x00e0 => return Instruction::CLS,
        0x00ee => return Instruction::RET,
        _ => (),
    };
    match i & 0xf000 {
        0x1000 => parse_1(i),
        0x2000 => parse_2(i),
        0x3000 => parse_3(i),
        0x4000 => parse_4(i),
        0x5000 => parse_5(i),
        0x6000 => parse_6(i),
        0x7000 => parse_7(i),
        0x8000 => parse_8(i),
        0x9000 => parse_9(i),
        0xa000 => parse_a(i),
        0xb000 => parse_b(i),
        0xc000 => parse_c(i),
        0xd000 => parse_d(i),
        0xe000 => parse_e(i),
        0xf000 => parse_f(i),
        _ => None,
    }.unwrap_or(Instruction::UNKNOWN(i))
}

pub fn read_all<T: AsRef<[u8]>>(data: T) -> Option<Vec<Instruction>> {
    let buf = data.as_ref();
    let n = buf.len();

    if n == 0 || n%2 != 0 {
        return None
    }

    let mut inst = Vec::new();

    for k in (0..n).step_by(2) {
        inst.push(read([buf[k], buf[k+1]]))
    }

    Some(inst)
}

#[inline(always)]
fn to8(i: u16) -> u8 {
    i as u8
}

fn parse_1(i: u16) -> Option<Instruction> {
    Some(Instruction::JPA(i & 0x0fff))
}

fn parse_2(i: u16) -> Option<Instruction> {
    Some(Instruction::CALL(i & 0x0fff))
}

fn parse_3(i: u16) -> Option<Instruction> {
    Some(Instruction::SEI(to8((i & 0x0f00) >> 8), to8(i & 0xff)))
}

fn parse_4(i: u16) -> Option<Instruction> {
    Some(Instruction::SNEI(to8((i & 0x0f00) >> 8), to8(i & 0xff)))
}

fn parse_5(i: u16) -> Option<Instruction> {
    if i&0x000f != 0 {
        return None
    }
    Some(Instruction::SER(to8((i & 0x0f00) >> 8), to8((i & 0x00f0) >> 4)))
}

fn parse_6(i: u16) -> Option<Instruction> {
    Some(Instruction::LDI(to8((i & 0x0f00) >> 8), to8(i & 0xff)))
}

fn parse_7(i: u16) -> Option<Instruction> {
    Some(Instruction::ADDI(to8((i & 0x0f00) >> 8), to8(i & 0xff)))
}

fn parse_8(i: u16) -> Option<Instruction> {
    match i & 0x000f {
        0x0000 => Some(Instruction::LDR(to8((i & 0x0f00) >> 8), to8((i & 0x00f0) >> 4))),
        0x0001 => Some(Instruction::ORR(to8((i & 0x0f00) >> 8), to8((i & 0x00f0) >> 4))),
        0x0002 => Some(Instruction::ANDR(to8((i & 0x0f00) >> 8), to8((i & 0x00f0) >> 4))),
        0x0003 => Some(Instruction::XORR(to8((i & 0x0f00) >> 8), to8((i & 0x00f0) >> 4))),
        0x0004 => Some(Instruction::ADDR(to8((i & 0x0f00) >> 8), to8((i & 0x00f0) >> 4))),
        0x0005 => Some(Instruction::SUBR(to8((i & 0x0f00) >> 8), to8((i & 0x00f0) >> 4))),
        0x0006 => Some(Instruction::SHRR(to8((i & 0x0f00) >> 8), to8((i & 0x00f0) >> 4))),
        0x0007 => Some(Instruction::SUBNR(to8((i & 0x0f00) >> 8), to8((i & 0x00f0) >> 4))),
        0x000e => Some(Instruction::SHLR(to8((i & 0x0f00) >> 8), to8((i & 0x00f0) >> 4))),
        _ => None,
    }
}

fn parse_9(i: u16) -> Option<Instruction> {
    if i&0x000f != 0 {
        return None
    }
    Some(Instruction::SNER(to8((i & 0x0f00) >> 8), to8((i & 0x00f0) >> 4)))
}

fn parse_a(i: u16) -> Option<Instruction> {
    Some(Instruction::LDA(i & 0x0fff))
}

fn parse_b(i: u16) -> Option<Instruction> {
    Some(Instruction::JPAFAR(i & 0x0fff))
}

fn parse_c(i: u16) -> Option<Instruction> {
    Some(Instruction::RND(to8((i & 0x0f00) >> 8), to8(i & 0xff)))
}

fn parse_d(i: u16) -> Option<Instruction> {
    Some(Instruction::DRW(to8((i & 0x0f00) >> 8), to8((i & 0x00f0) >> 4), to8(i & 0x000f)))
}

fn parse_e(i: u16) -> Option<Instruction> {
    match i & 0x00ff {
        0x009e => Some(Instruction::SKP(to8((i & 0x0f00) >> 8))),
        0x00a1 => Some(Instruction::SKNP(to8((i & 0x0f00) >> 8))),
        _ => None,
    }
}

fn parse_f(i: u16) -> Option<Instruction> {
    match i & 0x00ff {
        0x0007 => Some(Instruction::LDTG(to8((i & 0x0f00) >> 8))),
        0x000a => Some(Instruction::LDK(to8((i & 0x0f00) >> 8))),
        0x0015 => Some(Instruction::LDTS(to8((i & 0x0f00) >> 8))),
        0x0018 => Some(Instruction::LDSS(to8((i & 0x0f00) >> 8))),
        0x001e => Some(Instruction::ADDA(to8((i & 0x0f00) >> 8))),
        0x0029 => Some(Instruction::LDDIG(to8((i & 0x0f00) >> 8))),
        0x0033 => Some(Instruction::LDBCD(to8((i & 0x0f00) >> 8))),
        0x0055 => Some(Instruction::LDREGST(to8((i & 0x0f00) >> 8))),
        0x0065 => Some(Instruction::LDREGRD(to8((i & 0x0f00) >> 8))),
        _ => None,
    }
}
