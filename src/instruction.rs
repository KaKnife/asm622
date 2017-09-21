use line::Line;
use std::fmt::Display;
use self::Mnemonic::*;
use self::OpType::*;

#[derive(Debug, Clone,PartialEq, Eq)]
enum OpType {
    A,
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    Data(u8),
    Data16(u16),
    Addr(u8),
    Addr16(u16),
    AtR0,
    AtR1,
    C,
    Label(String),
    Dptr,
    AtDptr,
    AB,
    AtADptr,
    AtAPc,
}

#[derive(Debug, Clone)]
enum Mnemonic {
    Jmp,
    Setb,
    Da,
    Xchd,
    Push,
    Pop,
    Swap,
    Mul,
    Cpl,
    Db,
    Cseg,
    Dseg,
    Org,
    Nop,
    Ajmp,
    Ljmp,
    Rr,
    Inc,
    Jbc,
    Acall,
    Lcall,
    Rrc,
    Dec,
    Jb,
    Ret,
    Rl,
    Add,
    Jnb,
    Reti,
    Rlc,
    Addc,
    Jc,
    Orl,
    Jnc,
    Anl,
    Jz,
    Xrl,
    Jnz,
    Mov,
    Subb,
    Cjne,
    Xch,
    Djnz,
    Movx,
    Clr,
    Sjmp,
    Ds,
    Movc,
}

#[derive(Clone, Debug)]
pub struct Instruction {
    offset:u16 ,
    num: u8,
    pub label: Option<String>,
    mnemonic: Option<Mnemonic>,
    op1: Option<OpType>,
    op2: Option<OpType>,
}

impl Instruction {
    fn short_jmp(&self, addr: u16) -> u8 {
        // println!("{}", addr );
        // println!("{}", self.offset);
        // println!("{}", self.len());
        ((addr as u16+0x100-(self.offset as u16+self.len() as u16))%0x100)as u8
    }

    pub fn to_hex(&self) -> Result<Vec<u8>, String> {
        match self.validate() {
            Ok(()) => {},
            Err(e) => return Err(e),
        }
        match (&self.mnemonic, & self.op1, & self.op2) {
            (&Some(Nop),&None,&None) => Ok(vec![0x00]),
            (&Some(Ajmp),&Some(Addr16(d)),&None) => Ok(vec![0x01,d as u8]),
            (&Some(Ljmp),&Some(Addr16(d)),&None) => Ok(vec![0x02,(d/0x100)as u8,(d%0x100) as u8]),
            (&Some(Rr),&Some(A),&None) => Ok(vec![0x03]),
            (&Some(Inc),&Some(A),&None) => Ok(vec![0x04]),
            (&Some(Inc),&Some(Data(d)),&None) => Ok(vec![0x05,d]),
            (&Some(Inc),&Some(AtR0),&None) => Ok(vec![0x06]),
            (&Some(Inc),&Some(AtR1),&None) => Ok(vec![0x07]),
            (&Some(Inc),&Some(R0),&None) => Ok(vec![0x08]),
            (&Some(Inc),&Some(R1),&None) => Ok(vec![0x09]),
            (&Some(Inc),&Some(R2),&None) => Ok(vec![0x0A]),
            (&Some(Inc),&Some(R3),&None) => Ok(vec![0x0B]),
            (&Some(Inc),&Some(R4),&None) => Ok(vec![0x0C]),
            (&Some(Inc),&Some(R5),&None) => Ok(vec![0x0D]),
            (&Some(Inc),&Some(R6),&None) => Ok(vec![0x0E]),
            (&Some(Inc),&Some(R7),&None) => Ok(vec![0x0F]),

            (&Some(Jbc),&Some(Addr(b)),&Some(Addr16(d))) => Ok(vec![0x10,b,self.short_jmp(d as u16)]),
            (&Some(Acall),&Some(Addr16(d)),&None) => Ok(vec![0x11,d as u8]),
            (&Some(Lcall),&Some(Addr16(d)),&None) => Ok(vec![0x12,(d/0x100)as u8,(d%0x100) as u8]),
            (&Some(Rrc),&Some(A),&None) => Ok(vec![0x13]),
            (&Some(Dec),&Some(A),&None) => Ok(vec![0x14]),
            (&Some(Dec),&Some(Addr(d)),&None) => Ok(vec![0x15,d]),
            (&Some(Dec),&Some(AtR0),&None) => Ok(vec![0x16]),
            (&Some(Dec),&Some(AtR1),&None) => Ok(vec![0x17]),
            (&Some(Dec),&Some(R0),&None) => Ok(vec![0x18]),
            (&Some(Dec),&Some(R1),&None) => Ok(vec![0x19]),
            (&Some(Dec),&Some(R2),&None) => Ok(vec![0x1A]),
            (&Some(Dec),&Some(R3),&None) => Ok(vec![0x1B]),
            (&Some(Dec),&Some(R4),&None) => Ok(vec![0x1C]),
            (&Some(Dec),&Some(R5),&None) => Ok(vec![0x1D]),
            (&Some(Dec),&Some(R6),&None) => Ok(vec![0x1E]),
            (&Some(Dec),&Some(R7),&None) => Ok(vec![0x1F]),

            (&Some(Jb),&Some(Addr(a)),&Some(Addr16(d))) => Ok(vec![0x20,a,self.short_jmp(d as u16)]),
            //TODo: code 21
            (&Some(Ret),&None,&None) => Ok(vec![0x22]),
            (&Some(Rl),&Some(A),&None) => Ok(vec![0x23]),
            (&Some(Add),&Some(A),&Some(Data(d))) => Ok(vec![0x24,d]),
            (&Some(Add),&Some(A),&Some(Addr(d))) => Ok(vec![0x25,d]),
            (&Some(Add),&Some(A),&Some(AtR0)) => Ok(vec![0x26]),
            (&Some(Add),&Some(A),&Some(AtR1)) => Ok(vec![0x27]),
            (&Some(Add),&Some(A),&Some(R0)) => Ok(vec![0x28]),
            (&Some(Add),&Some(A),&Some(R1)) => Ok(vec![0x29]),
            (&Some(Add),&Some(A),&Some(R2)) => Ok(vec![0x2A]),
            (&Some(Add),&Some(A),&Some(R3)) => Ok(vec![0x2B]),
            (&Some(Add),&Some(A),&Some(R4)) => Ok(vec![0x2C]),
            (&Some(Add),&Some(A),&Some(R5)) => Ok(vec![0x2D]),
            (&Some(Add),&Some(A),&Some(R6)) => Ok(vec![0x2E]),
            (&Some(Add),&Some(A),&Some(R7)) => Ok(vec![0x2F]),

            (&Some(Jnb),&Some(Addr(a)),&Some(Addr16(d))) => Ok(vec![0x30,a,self.short_jmp(d as u16)]),
            //TODo: code 31
            (&Some(Reti),&None,&None) => Ok(vec![0x32]),
            (&Some(Rlc),&None,&None) => Ok(vec![0x33]),
            (&Some(Addc),&Some(A),&Some(Data(d))) => Ok(vec![0x34,d]),
            (&Some(Addc),&Some(A),&Some(Addr(d))) => Ok(vec![0x35,d]),
            (&Some(Addc),&Some(A),&Some(AtR0)) => Ok(vec![0x36]),
            (&Some(Addc),&Some(A),&Some(AtR1)) => Ok(vec![0x37]),
            (&Some(Addc),&Some(A),&Some(R0)) => Ok(vec![0x38]),
            (&Some(Addc),&Some(A),&Some(R1)) => Ok(vec![0x39]),
            (&Some(Addc),&Some(A),&Some(R2)) => Ok(vec![0x3A]),
            (&Some(Addc),&Some(A),&Some(R3)) => Ok(vec![0x3B]),
            (&Some(Addc),&Some(A),&Some(R4)) => Ok(vec![0x3C]),
            (&Some(Addc),&Some(A),&Some(R5)) => Ok(vec![0x3D]),
            (&Some(Addc),&Some(A),&Some(R6)) => Ok(vec![0x3E]),
            (&Some(Addc),&Some(A),&Some(R7)) => Ok(vec![0x3F]),

            (&Some(Jc),&Some(Addr16(d)),&None) => Ok(vec![0x40,self.short_jmp(d as u16)]),
            //TODo: code 41
            (&Some(Orl),&Some(Addr(a)),&Some(A)) => Ok(vec![0x42,a]),
            (&Some(Orl),&Some(Addr(a)),&Some(Data(d))) => Ok(vec![0x43,a,d]),
            (&Some(Orl),&Some(A),&Some(Data(d))) => Ok(vec![0x44,d]),
            (&Some(Orl),&Some(A),&Some(Addr(d))) => Ok(vec![0x45,d]),
            (&Some(Orl),&Some(A),&Some(AtR0)) => Ok(vec![0x46]),
            (&Some(Orl),&Some(A),&Some(AtR1)) => Ok(vec![0x47]),
            (&Some(Orl),&Some(A),&Some(R0)) => Ok(vec![0x48]),
            (&Some(Orl),&Some(A),&Some(R1)) => Ok(vec![0x49]),
            (&Some(Orl),&Some(A),&Some(R2)) => Ok(vec![0x4A]),
            (&Some(Orl),&Some(A),&Some(R3)) => Ok(vec![0x4B]),
            (&Some(Orl),&Some(A),&Some(R4)) => Ok(vec![0x4C]),
            (&Some(Orl),&Some(A),&Some(R5)) => Ok(vec![0x4D]),
            (&Some(Orl),&Some(A),&Some(R6)) => Ok(vec![0x4E]),
            (&Some(Orl),&Some(A),&Some(R7)) => Ok(vec![0x4F]),

            (&Some(Jnc),&Some(Addr16(d)),&None) => Ok(vec![0x50,self.short_jmp(d as u16)]),
            //TODo: code 51
            (&Some(Anl),&Some(Addr(a)),&Some(A)) => Ok(vec![0x52,a]),
            (&Some(Anl),&Some(Addr(a)),&Some(Data(d))) => Ok(vec![0x53,a,d]),
            (&Some(Anl),&Some(A),&Some(Data(d))) => Ok(vec![0x54,d]),
            (&Some(Anl),&Some(A),&Some(Addr(d))) => Ok(vec![0x55,d]),
            (&Some(Anl),&Some(A),&Some(AtR0)) => Ok(vec![0x56]),
            (&Some(Anl),&Some(A),&Some(AtR1)) => Ok(vec![0x57]),
            (&Some(Anl),&Some(A),&Some(R0)) => Ok(vec![0x58]),
            (&Some(Anl),&Some(A),&Some(R1)) => Ok(vec![0x59]),
            (&Some(Anl),&Some(A),&Some(R2)) => Ok(vec![0x5A]),
            (&Some(Anl),&Some(A),&Some(R3)) => Ok(vec![0x5B]),
            (&Some(Anl),&Some(A),&Some(R4)) => Ok(vec![0x5C]),
            (&Some(Anl),&Some(A),&Some(R5)) => Ok(vec![0x5D]),
            (&Some(Anl),&Some(A),&Some(R6)) => Ok(vec![0x5E]),
            (&Some(Anl),&Some(A),&Some(R7)) => Ok(vec![0x5F]),

            (&Some(Jz),&Some(Addr16(d)),&None) => Ok(vec![0x60,self.short_jmp(d as u16)]),
            //TODo: code 61
            (&Some(Xrl),&Some(Addr(d)),&Some(A)) => Ok(vec![0x62,d]),
            (&Some(Xrl),&Some(Addr(a)),&Some(Data(d))) => Ok(vec![0x63,a,d]),
            (&Some(Xrl),&Some(A),&Some(Data(d))) => Ok(vec![0x64,d]),
            (&Some(Xrl),&Some(A),&Some(Addr(d))) => Ok(vec![0x65,d]),
            (&Some(Xrl),&Some(A),&Some(AtR0)) => Ok(vec![0x66]),
            (&Some(Xrl),&Some(A),&Some(AtR1)) => Ok(vec![0x67]),
            (&Some(Xrl),&Some(A),&Some(R0)) => Ok(vec![0x68]),
            (&Some(Xrl),&Some(A),&Some(R1)) => Ok(vec![0x69]),
            (&Some(Xrl),&Some(A),&Some(R2)) => Ok(vec![0x6A]),
            (&Some(Xrl),&Some(A),&Some(R3)) => Ok(vec![0x6B]),
            (&Some(Xrl),&Some(A),&Some(R4)) => Ok(vec![0x6C]),
            (&Some(Xrl),&Some(A),&Some(R5)) => Ok(vec![0x6D]),
            (&Some(Xrl),&Some(A),&Some(R6)) => Ok(vec![0x6E]),
            (&Some(Xrl),&Some(A),&Some(R7)) => Ok(vec![0x6F]),

            (&Some(Jnz),&Some(Addr16(d)),&None) => Ok(vec![0x70,self.short_jmp(d as u16)]),
            //TODo: code 71
            (&Some(Orl),&Some(C),&Some(Addr(d))) => Ok(vec![0x72,d]),
            (&Some(Jmp),&Some(AtADptr),&None) => Ok(vec![0x73]),
            (&Some(Mov),&Some(A),&Some(Data(d))) => Ok(vec![0x74,d]),
            (&Some(Mov),&Some(Addr(a)),&Some(Data(d))) => Ok(vec![0x75,a,d]),
            (&Some(Mov),&Some(AtR0),&Some(Data(d))) => Ok(vec![0x76,d]),
            (&Some(Mov),&Some(AtR1),&Some(Data(d))) => Ok(vec![0x77,d]),
            (&Some(Mov),&Some(R0),&Some(Data(d))) => Ok(vec![0x78,d]),
            (&Some(Mov),&Some(R1),&Some(Data(d))) => Ok(vec![0x79,d]),
            (&Some(Mov),&Some(R2),&Some(Data(d))) => Ok(vec![0x7A,d]),
            (&Some(Mov),&Some(R3),&Some(Data(d))) => Ok(vec![0x7B,d]),
            (&Some(Mov),&Some(R4),&Some(Data(d))) => Ok(vec![0x7C,d]),
            (&Some(Mov),&Some(R5),&Some(Data(d))) => Ok(vec![0x7D,d]),
            (&Some(Mov),&Some(R6),&Some(Data(d))) => Ok(vec![0x7E,d]),
            (&Some(Mov),&Some(R7),&Some(Data(d))) => Ok(vec![0x7F,d]),

            (&Some(Sjmp),&Some(Addr(d)),&None) => Ok(vec![0x80,self.short_jmp(d as u16)]),
            //TODo: code 81
            (&Some(Anl),&Some(C),&Some(Addr(d))) => Ok(vec![0x82,d]),
            (&Some(Movc),&Some(A),&Some(AtAPc)) => Ok(vec![0x83]),
            (&Some(Mov),&Some(Data(d)),&Some(A)) => Ok(vec![0x84,d]),
            (&Some(Mov),&Some(Data(d)),&Some(Addr(a))) => Ok(vec![0x85,a,d]),
            (&Some(Mov),&Some(Data(d)),&Some(AtR0)) => Ok(vec![0x86,d]),
            (&Some(Mov),&Some(Data(d)),&Some(AtR1)) => Ok(vec![0x87,d]),
            (&Some(Mov),&Some(Data(d)),&Some(R0)) => Ok(vec![0x88,d]),
            (&Some(Mov),&Some(Data(d)),&Some(R1)) => Ok(vec![0x89,d]),
            (&Some(Mov),&Some(Data(d)),&Some(R2)) => Ok(vec![0x8A,d]),
            (&Some(Mov),&Some(Data(d)),&Some(R3)) => Ok(vec![0x8B,d]),
            (&Some(Mov),&Some(Data(d)),&Some(R4)) => Ok(vec![0x8C,d]),
            (&Some(Mov),&Some(Data(d)),&Some(R5)) => Ok(vec![0x8D,d]),
            (&Some(Mov),&Some(Data(d)),&Some(R6)) => Ok(vec![0x8E,d]),
            (&Some(Mov),&Some(Data(d)),&Some(R7)) => Ok(vec![0x8F,d]),

            (&Some(Mov),&Some(Dptr),&Some(Data16(d))) => Ok(vec![0x90,(d/0x100)as u8,(d%0x100) as u8]),
            //TODo: code 91
            (&Some(Mov),&Some(Addr(d)),&Some(C)) => Ok(vec![0x92,d]),
            (&Some(Movc),&Some(A),&Some(AtADptr)) => Ok(vec![0x93]),
            (&Some(Subb),&Some(A),&Some(Data(d))) => Ok(vec![0x94,d]),
            (&Some(Subb),&Some(A),&Some(Addr(d))) => Ok(vec![0x95,d]),
            (&Some(Subb),&Some(A),&Some(AtR0)) => Ok(vec![0x96]),
            (&Some(Subb),&Some(A),&Some(AtR1)) => Ok(vec![0x97]),
            (&Some(Subb),&Some(A),&Some(R0)) => Ok(vec![0x98]),
            (&Some(Subb),&Some(A),&Some(R1)) => Ok(vec![0x99]),
            (&Some(Subb),&Some(A),&Some(R2)) => Ok(vec![0x9A]),
            (&Some(Subb),&Some(A),&Some(R3)) => Ok(vec![0x9B]),
            (&Some(Subb),&Some(A),&Some(R4)) => Ok(vec![0x9C]),
            (&Some(Subb),&Some(A),&Some(R5)) => Ok(vec![0x9D]),
            (&Some(Subb),&Some(A),&Some(R6)) => Ok(vec![0x9E]),
            (&Some(Subb),&Some(A),&Some(R7)) => Ok(vec![0x9F]),

            //TODo: code A0
            // (&Some(Ajmp),&Some(Addr16(d)),&None) => Ok(vec![0xD1,d as u8]),
            (&Some(Mov),&Some(C),&Some(Addr(d))) => Ok(vec![0xA2,d]),
            (&Some(Inc),&Some(Dptr),&None) => Ok(vec![0xA3]),
            (&Some(Mul),&Some(AB),&None) => Ok(vec![0xA4]),
            (&Some(Mov),&Some(Addr(a)),&Some(Addr(d))) => Ok(vec![0xA5,a,d]),
            (&Some(Mov),&Some(AtR0),&Some(Addr(d))) => Ok(vec![0xA6,d]),
            (&Some(Mov),&Some(AtR1),&Some(Addr(d))) => Ok(vec![0xA7,d]),
            (&Some(Mov),&Some(R0),&Some(Addr(d))) => Ok(vec![0xA8,d]),
            (&Some(Mov),&Some(R1),&Some(Addr(d))) => Ok(vec![0xA9,d]),
            (&Some(Mov),&Some(R2),&Some(Addr(d))) => Ok(vec![0xAA,d]),
            (&Some(Mov),&Some(R3),&Some(Addr(d))) => Ok(vec![0xAB,d]),
            (&Some(Mov),&Some(R4),&Some(Addr(d))) => Ok(vec![0xAC,d]),
            (&Some(Mov),&Some(R5),&Some(Addr(d))) => Ok(vec![0xAD,d]),
            (&Some(Mov),&Some(R6),&Some(Addr(d))) => Ok(vec![0xAE,d]),
            (&Some(Mov),&Some(R7),&Some(Addr(d))) => Ok(vec![0xAF,d]),

            ///TODo: code B0
            // (&Some(Acall),&Some(Addr16(d)),&None) => Ok(vec![0xD1,d as u8]),
            (&Some(Cpl),&Some(Addr(d)),&None) => Ok(vec![0xB2,d]),
            (&Some(Cpl),&Some(C),&None) => Ok(vec![0xB3]),
            //TODO: codes B4-BF

            (&Some(Push),&Some(Addr(d)),&None) => Ok(vec![0xC0,d]),
            //TODo: code C1
            (&Some(Clr),&Some(Addr(d)),&None) => Ok(vec![0xC2,d]),
            (&Some(Clr),&Some(C),&None) => Ok(vec![0xC3]),
            (&Some(Swap),&Some(A),&None) => Ok(vec![0xC4]),
            (&Some(Xch),&Some(A),&Some(Addr(d))) => Ok(vec![0xC5,d]),
            (&Some(Xch),&Some(A),&Some(AtR0)) => Ok(vec![0xC6]),
            (&Some(Xch),&Some(A),&Some(AtR1)) => Ok(vec![0xC7]),
            (&Some(Xch),&Some(A),&Some(R0)) => Ok(vec![0xC8]),
            (&Some(Xch),&Some(A),&Some(R1)) => Ok(vec![0xC9]),
            (&Some(Xch),&Some(A),&Some(R2)) => Ok(vec![0xCA]),
            (&Some(Xch),&Some(A),&Some(R3)) => Ok(vec![0xCB]),
            (&Some(Xch),&Some(A),&Some(R4)) => Ok(vec![0xCC]),
            (&Some(Xch),&Some(A),&Some(R5)) => Ok(vec![0xCD]),
            (&Some(Xch),&Some(A),&Some(R6)) => Ok(vec![0xCE]),
            (&Some(Xch),&Some(A),&Some(R7)) => Ok(vec![0xCF]),

            (&Some(Pop),&Some(Addr(d)),&None) => Ok(vec![0xD0,d]),
            // (&Some(Acall),&Some(Addr16(d)),&None) => Ok(vec![0xD1,d as u8]),
            (&Some(Setb),&Some(Addr(d)),&None) => Ok(vec![0xD2,d]),
            (&Some(Setb),&Some(C),&None) => Ok(vec![0xD3]),
            (&Some(Da),&Some(A),&None) => Ok(vec![0xD4]),
            (&Some(Djnz),&Some(Addr(d)),&Some(Addr16(a))) => Ok(vec![0xD5,d,self.short_jmp(a )]),
            (&Some(Xchd),&Some(A),&Some(AtR0)) => Ok(vec![0xD6]),
            (&Some(Xchd),&Some(A),&Some(AtR1)) => Ok(vec![0xD7]),
            (&Some(Djnz),&Some(R0),&Some(Addr16(d))) => Ok(vec![0xD8,self.short_jmp(d as u16)]),
            (&Some(Djnz),&Some(R1),&Some(Addr16(d))) => Ok(vec![0xD9,self.short_jmp(d  as u16)]),
            (&Some(Djnz),&Some(R2),&Some(Addr16(d))) => Ok(vec![0xDA,self.short_jmp(d  as u16)]),
            (&Some(Djnz),&Some(R3),&Some(Addr16(d))) => Ok(vec![0xDB,self.short_jmp(d  as u16)]),
            (&Some(Djnz),&Some(R4),&Some(Addr16(d))) => Ok(vec![0xDC,self.short_jmp(d as u16)]),
            (&Some(Djnz),&Some(R5),&Some(Addr16(d))) => Ok(vec![0xDD,self.short_jmp(d as u16)]),
            (&Some(Djnz),&Some(R6),&Some(Addr16(d))) => Ok(vec![0xDE,self.short_jmp(d as u16)]),
            (&Some(Djnz),&Some(R7),&Some(Addr16(d))) => Ok(vec![0xDF,self.short_jmp(d as u16)]),

            (&Some(Movx),&Some(A),&Some(AtDptr)) => Ok(vec![0xE0]),
            // (&Some(Acall),&Some(Addr16(d)),&None) => Ok(vec![0xE1,d as u8]),
            (&Some(Movx),&Some(A),&Some(AtR0)) => Ok(vec![0xE2]),
            (&Some(Movx),&Some(A),&Some(AtR1)) => Ok(vec![0xE3]),
            (&Some(Clr),&Some(A),&None) => Ok(vec![0xE4]),
            (&Some(Mov),&Some(A),&Some(Addr(d))) => Ok(vec![0xE5,d]),
            (&Some(Mov),&Some(A),&Some(AtR0)) => Ok(vec![0xE6]),
            (&Some(Mov),&Some(A),&Some(AtR1)) => Ok(vec![0xE7]),
            (&Some(Mov),&Some(A),&Some(R0)) => Ok(vec![0xE8]),
            (&Some(Mov),&Some(A),&Some(R1)) => Ok(vec![0xE9]),
            (&Some(Mov),&Some(A),&Some(R2)) => Ok(vec![0xEA]),
            (&Some(Mov),&Some(A),&Some(R3)) => Ok(vec![0xEB]),
            (&Some(Mov),&Some(A),&Some(R4)) => Ok(vec![0xEC]),
            (&Some(Mov),&Some(A),&Some(R5)) => Ok(vec![0xED]),
            (&Some(Mov),&Some(A),&Some(R6)) => Ok(vec![0xEE]),
            (&Some(Mov),&Some(A),&Some(R7)) => Ok(vec![0xEF]),

            (&Some(Movx),&Some(AtDptr),&Some(A)) => Ok(vec![0xF0]),
            // (&Some(Acall),&Some(Addr16(d)),&None) => Ok(vec![0xF1,d as u8]),
            (&Some(Movx),&Some(AtR0),&Some(A)) => Ok(vec![0xF2]),
            (&Some(Movx),&Some(AtR1),&Some(A)) => Ok(vec![0xF3]),
            (&Some(Cpl),&Some(A),&None) => Ok(vec![0xF4]),
            (&Some(Mov),&Some(Addr(d)),&Some(A)) => Ok(vec![0xF5,d]),
            (&Some(Mov),&Some(AtR0),&Some(A)) => Ok(vec![0xF6]),
            (&Some(Mov),&Some(AtR1),&Some(A)) => Ok(vec![0xF7]),
            (&Some(Mov),&Some(R0),&Some(A)) => Ok(vec![0xF8]),
            (&Some(Mov),&Some(R1),&Some(A)) => Ok(vec![0xF9]),
            (&Some(Mov),&Some(R2),&Some(A)) => Ok(vec![0xFA]),
            (&Some(Mov),&Some(R3),&Some(A)) => Ok(vec![0xFB]),
            (&Some(Mov),&Some(R4),&Some(A)) => Ok(vec![0xFC]),
            (&Some(Mov),&Some(R5),&Some(A)) => Ok(vec![0xFD]),
            (&Some(Mov),&Some(R6),&Some(A)) => Ok(vec![0xFE]),
            (&Some(Mov),&Some(R7),&Some(A)) => Ok(vec![0xFF]),

            (&Some(Org),_,_) => Ok(vec![]),
            (&Some(Cseg),_,_) => Ok(vec![]),
            (&Some(Dseg),_,_) => Ok(vec![]),
            (&Some(Db), &Some(Label(ref l)), &None) => {
                let mut v = l.clone().into_bytes();
                v.remove(l.len()-1);
                v.remove(0);
                v.push(0);
                Ok(v)
            },
            (&Some(Ds),&Some(Addr(d)),&None) => Ok(vec![0x10;d as usize]),
            (&None,_,_) => Ok(vec![]),
            (a@_,b@_,c@_) => Err(format!("Unimplemented: {:?},{:?},{:?}", a,b,c)),
        }
    }

    pub fn from_line(line:Line, offset: u16) -> Result<Self, String> {
        let mne;
        let op1;
        let op2;

        if line.mnu.is_some() {
            mne = match line.mnu.unwrap().to_lowercase().as_ref() {
                "db" => Some(Db),
                "mov" => Some(Mnemonic::Mov),
                "sjmp" => Some(Mnemonic::Sjmp),
                "add" => Some(Mnemonic::Add),
                "org" => Some(Mnemonic::Org),
                "cseg" => Some(Cseg),
                "dseg" => Some(Dseg),
                "nop" => Some(Nop),
                "ajmp" => Some(Ajmp),
                "ljmp" => Some(Ljmp),
                "rr" => Some(Rr),
                "inc" => Some(Inc),
                "jbc" => Some(Jbc),
                "acall" => Some(Acall),
                "lcall" => Some(Lcall),
                "rrc" => Some(Rrc),
                "dec" => Some(Dec),
                "jb" => Some(Jb),
                "ret" => Some(Ret),
                "rl" => Some(Rl),
                "jnb" => Some(Jnb),
                "reti" => Some(Reti),
                "clr" => Some(Clr),
                "movx" => Some(Movx),
                "djnz" => Some(Djnz),
                "xch" => Some(Xch),
                "cjne" => Some(Cjne),
                "subb" => Some(Subb),
                "jnz" => Some(Jnz),
                "xrl" => Some(Xrl),
                "jz" => Some(Jz),
                "jnc" => Some(Jnc),
                "orl" => Some(Orl),
                "jc" => Some(Jc),
                "addc" => Some(Addc),
                "rlc" => Some(Rlc),
                "anl" => Some(Anl),
                "cpl" => Some(Cpl),
                "ds" => Some(Ds),
                "swap" => Some(Swap),
                "push" => Some(Push),
                "pop" => Some(Pop),
                "mul" => Some(Mul),
                "xchd" => Some(Xchd),
                "da" => Some(Da),
                "setb" => Some(Setb),
                "movc" => Some(Movc),
                "jmp" => Some(Jmp),
                "end" => None,
                m @ _ => return Err(format!("unknown mnemonic: line {}: {}",line.num,m)),
            };
        }
        else {
            mne = None;
        }

        if line.op1.is_some() {
            op1 = match line.op1.unwrap().to_lowercase().as_ref() {
                "@r0" => Some(OpType::AtR0),
                "@r1" => Some(OpType::AtR1),
                "r1" => Some(OpType::R1),
                "r2" => Some(OpType::R2),
                "r3" => Some(OpType::R3),
                "r4" => Some(OpType::R4),
                "r5" => Some(OpType::R5),
                "r6" => Some(OpType::R6),
                "r7" => Some(OpType::R7),
                "r0" => Some(OpType::R0),
                "c" => Some(OpType::C),
                "a" => Some(OpType::A),
                "ab" => Some(OpType::AB),
                "@dptr" => Some(AtDptr),
                "@a+dptr" => Some(AtADptr),
                "@a+pc" => Some(AtAPc),
                "dptr" => Some(Dptr),
                "b" =>Some(Label("B".to_string())),
                op @ _ => {
                    match other_op(op.to_string()){
                        Ok(op) => Some(op),
                        Err(e) => return Err(e),
                    }
                },
            };
        }
        else {
            op1 = None;
        }

        if line.op2.is_some() {
            op2 = match line.op2.unwrap().to_lowercase().as_ref() {
                "@r0" => Some(OpType::AtR0),
                "@r1" => Some(OpType::AtR1),
                "r1" => Some(OpType::R1),
                "r2" => Some(OpType::R2),
                "r3" => Some(OpType::R3),
                "r4" => Some(OpType::R4),
                "r5" => Some(OpType::R5),
                "r6" => Some(OpType::R6),
                "r7" => Some(OpType::R7),
                "r0" => Some(OpType::R0),
                "c" => Some(OpType::C),
                "a" => Some(OpType::A),
                "b" =>Some(Label("B".to_string())),
                "dptr" => Some(Dptr),
                "@a+pc" => Some(AtAPc),
                "@a+dptr" => Some(AtADptr),
                "@dptr" => Some(AtDptr),
                op @ _ => {
                    match other_op(op.to_string()){
                        Ok(op) => Some(op),
                        Err(e) => return Err(e),
                    }
                },
            };
        }
        else {
            op2 = None;
        }

        Ok(Instruction{offset:offset, num: line.num, label:line.label, mnemonic: mne, op1: op1, op2: op2,})
    }

    pub fn offset(&self) ->u16 {
        self.offset
    }

    pub fn len(&self) ->i32 {
        let mut len = 0;
        if self.mnemonic.is_some() {
            match self.mnemonic.clone().unwrap(){
                Org |Cseg | Dseg=> return 0,
                Db => {
                    match self.op1.clone().unwrap() {
                        Label(l)=> return l.len() as i32,
                        _ => {},
                    };
                },
                Ds => {
                    match self.op1.clone().unwrap() {
                        Addr(l)=> return l as i32,
                        _ => {},
                    };
                },
                Lcall | Ljmp => return 3,
                _ =>{},
            }
            len +=1;
            if self.op1.is_some() {
                match self.op1.clone().unwrap() {
                    Data(_)| Addr(_)| Addr16(_)=> len+=1,
                    Label(_) => len+=1,
                    Data16(_) => len+=2,
                    _ => {},
                };
            }
            if self.op2.is_some() {
                match self.op2.clone().unwrap() {
                    Data(_)| Addr(_) | Addr16(_)=> len+=1,
                    Label(_) => len+=1,
                    Data16(_) => len+=2,
                    _ => {},
                };
            }
        }

        len

    }

    pub fn fix_label(&mut self, table: &Vec<(String, u16)>) -> Result<(), String>{
        if self.mnemonic.is_none() {
            return Ok(())
        }
        match self.mnemonic.clone().unwrap() {
            Cseg | Db => return Ok(()),
            _ => {},
        }
        let label_table = table.clone();
        if self.op1.is_some(){
            let op = self.op1.clone();
            match op.unwrap() {
                OpType::Label(label_full) => {
                    if label_full.to_lowercase() != "at"
                    {
                        let split: Vec<&str> = label_full.split('.').collect();
                        let label = split[0];
                        let index = label_table.iter().position(|x| x.0.to_lowercase() == label);
                        if index.is_none(){
                            return Err(format!("Could not find label: line {}: {}",self.num,label));
                        }
                        let addr = label_table[index.unwrap()].1;
                        let mut add = 0;
                        if split.len()>1 {
                            add = match u8::from_str_radix(split[1], 16){
                                Ok(a) => a,
                                Err(_) => 0,
                            };
                        }
                        self.op1 = Some(OpType::Addr16(addr+add as u16));
                    }
                },
                _=>{},
            }
        }

        if self.op2.is_some(){
            let op = self.op2.clone();
            match op.unwrap() {
                OpType::Label(label_full) => {
                    let split: Vec<&str> = label_full.split('.').collect();
                    let label = split[0];
                    let index = label_table.iter().position(|x| x.0.to_lowercase() == label);
                    if index.is_none(){
                        return Err(format!("Could not find label:{}: {}",self.num,label));
                        // return Err(format!("Could not find label:{}: {}",self.num,label));
                    }
                    let addr = label_table[index.unwrap()].1;
                    let mut add =0;
                    if split.len()>1 {
                        add = match u8::from_str_radix(split[1], 16){
                            Ok(a) => a,
                            Err(_) => 0,
                        };
                    }
                    self.op2 = Some(OpType::Addr16(addr+add as u16));
                },
                _=>{},
            }
        }


        Ok(())
    }

    pub fn validate(&self) -> Result<(), String> {
        match &self.mnemonic {
            &None => Ok(()),
            &Some(Nop) => {
                if self.op1.is_some() | self.op1.is_some(){
                    return Err(String::from("NOP takes no operands"))
                }
                Ok(())
            },

            _ => Ok(()),
        }
    }

    pub fn is_new_section(&self) -> Option<u16> {
        let is_new = match self.mnemonic {
            Some(Org) | Some(Cseg) | Some(Dseg) => true,
            _ => false,
        };
        if !is_new{
            return None;
        }
        match &self.op1 {
            &Some(Addr(d)) => Some(d as u16),
            &Some(Addr16(d)) => Some(d),
            &Some(Label(_)) => {
                match &self.op2 {
                    &Some(Addr(d)) => Some(d as u16),
                    &Some(Addr16(d)) => Some(d),
                    _ =>panic!("ORG, CSEG, or DSEG has invalid addr at line {}",self.num)
                }
            },
            _ =>panic!("ORG, CSEG, or DSEG has invalid addr at line {}",self.num)
        }
    }
}

fn other_op(mut op: String) -> Result<OpType, String> {
    op.trim();
    if op.starts_with("#") {
        op.remove(0);
        if op.ends_with("h") {
            let len = op.len();
            op.remove(len -1);
            match u8::from_str_radix(&op, 16){
                Ok(d) => return Ok(Data(d)),
                Err(_) => {},
            };
            return  match u16::from_str_radix(&op, 16){
                Ok(d) => Ok(Data16(d)),
                Err(_) => return Err(String::from("Invalid Data")),
            }
        }
        match u8::from_str_radix(&op, 10) {
            Ok(a) => return Ok(Data(a)),
            Err(_) => {},
        }
        match u16::from_str_radix(&op, 10) {
            Ok(a) => return Ok(Data16(a)),
            Err(_) => return Err(String::from("Invalid Data")),
        }
    }
    if op.ends_with("h") {
        let len = op.len();
        let mut temp = op.clone();
        temp.remove(len -1);
        match u8::from_str_radix(&temp, 16){
            Ok(a) => return Ok( Addr(a)),
            Err(_) => {},
        }
        match u16::from_str_radix(&temp, 16){
            Ok(a) => return Ok( Addr16(a)),
            Err(_) => return Err(String::from("Invalid Address")),
        }
    }
    if op.starts_with("0x") {
        let mut temp = op.clone();
        temp.remove(0);
        temp.remove(0);
        match u8::from_str_radix(&temp, 16){
            Ok(a) => return Ok( Addr(a)),
            Err(_) => {},
        }
        match u16::from_str_radix(&temp, 16){
            Ok(a) => return Ok( Addr16(a)),
            Err(_) => return Err(String::from("Invalid Address")),
        }
    }
    match u8::from_str_radix(&op, 10){
        Ok(a) => return Ok( Addr(a)),
        Err(_) => {},
    }
    match u16::from_str_radix(&op, 10){
        Ok(a) => return Ok( Addr16(a)),
        Err(_) => {},
    }
    match u64::from_str_radix(&op, 10) {
        Ok(_) => return Err(String::from("Invalid Address")),
        Err(_) => {},
    };
    Ok(Label(op))
}


impl Display for Instruction {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let mut out :String = "".to_string();
        match self.label{
            Some(ref label) => out += &format!("{:02x}-label\t: {}\n",self.offset,label),
            _ => {},
        };
        match self.mnemonic{
            Some(ref mnu) => out += &format!("{:02x}-Mnumonic\t: {:?}\n",self.offset,mnu),
            _ => return write!(f, "{}", out),
        }
        match self.op1{
            Some(ref op) => out += &format!("{:02x}-Operand 1\t: {:?}\n",self.offset,op),
            _ => {},
        };
        match self.op2{
            Some(ref op) => out += &format!("{:02x}-Operand 2\t: {:?}\n",self.offset,op),
            _ => {},
        };
        write!(f, "{}", out)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn other_op_data() {
        assert_eq!(other_op(String::from("#0")), Ok(Data(0)));
        assert_eq!(other_op(String::from("#10")), Ok(Data(10)));
        assert_eq!(other_op(String::from("#10h")), Ok(Data(0x10)));
        assert_eq!(other_op(String::from("#100h")), Ok(Data16(0x100)));
        assert_eq!(other_op(String::from("#10000")), Ok(Data16(10000)));
        match other_op(String::from("#10000h")) {
            Ok(_) => assert!(false),
            Err(_) => assert!(true),
        }
        match other_op(String::from("#65536")) {
            Ok(_) => assert!(false),
            Err(_) => assert!(true),
        }


    }

    #[test]
    fn other_op_addr() {
        assert_eq!(other_op(String::from("0")), Ok(Addr(0)));
        assert_eq!(other_op(String::from("10")), Ok(Addr(10)));
        assert_eq!(other_op(String::from("0x10")), Ok(Addr(0x10)));
        assert_eq!(other_op(String::from("10h")), Ok(Addr(0x10)));
        assert_eq!(other_op(String::from("100h")), Ok(Addr16(0x100)));
        assert_eq!(other_op(String::from("10000")), Ok(Addr16(10000)));
        match other_op(String::from("#10000h")) {
            Ok(_) => assert!(false),
            Err(_) => assert!(true),
        }
        match other_op(String::from("#65536")) {
            Ok(_) => assert!(false),
            Err(_) => assert!(true),
        }

    }

    #[test]
    fn nop() {
        let ins1 = Instruction{offset:0, num:0, label:None, mnemonic:Some(Nop), op1:None, op2:None};
        let ins2 = Instruction{offset:22, num:4, label:Some(String::from("HI")), mnemonic:Some(Nop), op1:Some(R1), op2:None};

        assert_eq!(ins1.validate(), Ok(()));
        match ins2.validate() {
            Ok(_) => assert!(false),
            Err(_) => assert!(true),
        }
        assert_eq!(ins1.to_hex(), Ok(vec![00]));
        assert_eq!(ins1.len(), 1);
    }


}
