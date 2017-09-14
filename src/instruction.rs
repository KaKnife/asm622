use line::Line;
use std::fmt::Display;
use self::Mnemonic::*;
use self::OpType::*;

#[derive(Debug, Clone)]
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
    Addr(u8),
    AtR0,
    AtR1,
    C,
    Label(String),
    Dptr,
}

#[derive(Debug, Clone)]
enum Mnemonic {
    Cpl,
    Db,
    Cseg,
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
}

#[derive(Clone)]
pub struct Instruction {
    offset:u8 ,
    num: u8,
    pub label: Option<String>,
    mnemonic: Option<Mnemonic>,
    op1: Option<OpType>,
    op2: Option<OpType>,
}

impl Instruction {
    pub fn short_jmp(&self, addr: u8) -> u8 {
        ((addr as u16+0x100-(self.offset as u16+self.len() as u16))%0x100)as u8
    }

    pub fn to_hex(&self) -> Result<Vec<u8>, String> {

        match (&self.mnemonic, & self.op1, & self.op2) {
            (&Some(Nop),&None,&None) => Ok(vec![0x00]),
            //TODO: code 01
            //TODO: code 02
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

            (&Some(Jbc),&Some(Addr(b)),&Some(Addr(d))) => Ok(vec![0x10,b,self.short_jmp(d)]),
            //TODO: code 11
            //TODO: code 12
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

            //TODO: code 20
            //TODO: code 21
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

            //TODO: code 30
            //TODO: code 31
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

            //TODO: code 40
            //TODO: code 41
            //TODO: code 42
            //TODO: code 43
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

            //TODO: code 50
            //TODO: code 51
            //TODO: code 52
            //TODO: code 53
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

            //TODO: code 60
            //TODO: code 61
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

            //TODO: code 70
            //TODO: code 71
            (&Some(Orl),&Some(C),&Some(Addr(d))) => Ok(vec![0x72,d]),
            //TODO: code 73
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

            (&Some(Sjmp),&Some(Addr(d)),&None) => Ok(vec![0x80,self.short_jmp(d)]),
            //TODO: code 81
            (&Some(Anl),&Some(C),&Some(Addr(d))) => Ok(vec![0x82,d]),
            //TODO: code 83
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

            //TODO: code 90
            //TODO: code 91
            (&Some(Mov),&Some(Addr(d)),&Some(C)) => Ok(vec![0x92,d]),
            //TODO: code 93
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

            //TODO: code A0
            //TODO: code A1
            (&Some(Mov),&Some(C),&Some(Addr(d))) => Ok(vec![0xA2,d]),
            (&Some(Inc),&Some(Dptr),&None) => Ok(vec![0xA3]),
            (&Some(Mov),&Some(A),&Some(Addr(d))) => Ok(vec![0xA4,d]),
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

            //TODO: code B0
            //TODO: code B1
            (&Some(Cpl),&Some(Addr(d)),&None) => Ok(vec![0xB2,d]),
            (&Some(Cpl),&Some(C),&None) => Ok(vec![0xB3]),
            //TODO: codes B4-BF

            //TODO: codes C0-CF

            //TODO: codes D0-D7
            (&Some(Djnz),&Some(R0),&Some(Addr(d))) => Ok(vec![0xD8,self.short_jmp(d)]),
            (&Some(Djnz),&Some(R1),&Some(Addr(d))) => Ok(vec![0xD9,self.short_jmp(d)]),
            (&Some(Djnz),&Some(R2),&Some(Addr(d))) => Ok(vec![0xDA,self.short_jmp(d)]),
            (&Some(Djnz),&Some(R3),&Some(Addr(d))) => Ok(vec![0xDB,self.short_jmp(d)]),
            (&Some(Djnz),&Some(R4),&Some(Addr(d))) => Ok(vec![0xDC,self.short_jmp(d)]),
            (&Some(Djnz),&Some(R5),&Some(Addr(d))) => Ok(vec![0xDD,self.short_jmp(d)]),
            (&Some(Djnz),&Some(R6),&Some(Addr(d))) => Ok(vec![0xDE,self.short_jmp(d)]),
            (&Some(Djnz),&Some(R7),&Some(Addr(d))) => Ok(vec![0xDF,self.short_jmp(d)]),

            //TODO: Codes E0-FF

            (&Some(Org),_,_) => Ok(vec![]),
            (&Some(Cseg),_,_) => Ok(vec![]),
            (&Some(Db), &Some(Label(ref l)), &None) => {
                let mut v = l.clone().into_bytes();
                v.remove(l.len()-1);
                v.remove(0);
                v.push(0);
                Ok(v)
            },
            (&Some(Ds),&Some(Addr(d)),&None) => Ok(vec![0x10;d as usize]),
            (&None,_,_) => Ok(vec![]),
            (a@_,b@_,c@_) => Err(format!("{:?},{:?},{:?}", a,b,c)),
        }
    }

    pub fn from_line(line:Line, offset: u8) -> Result<Self, String> {
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
                "cseg" | "dseg" => Some(Cseg),
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
                "dptr" => Some(Dptr),
                "b" =>Some(Label("B".to_string())),
                op @ _ => Some(other_op(op.to_string())),
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
                op @ _ => Some(other_op(op.to_string())),
            };
        }
        else {
            op2 = None;
        }

        Ok(Instruction{offset:offset, num: line.num, label:line.label, mnemonic: mne, op1: op1, op2: op2,})
    }

    pub fn offset(&self) ->u8 {
        self.offset
    }

    #[allow(unused_variables)]
    pub fn len(&self) ->i16 {
        let mut len = 0;
        if self.mnemonic.is_some() {
            match self.mnemonic.clone().unwrap(){
                Org => {
                    match self.op1.clone().unwrap() {
                        Data(d)| Addr(d)=> return d as i16 - self.offset as i16,
                        _ => {},
                    };
                },
                Cseg => {
                    match self.op2.clone().unwrap() {
                        Data(d)| Addr(d)=> return d as i16 - self.offset as i16,
                        _ => {},
                    };
                },
                Db => {
                    match self.op1.clone().unwrap() {
                        Label(l)=> return l.len() as i16,
                        _ => {},
                    };
                },
                Ds => {
                    match self.op1.clone().unwrap() {
                        Addr(l)=> return l as i16,
                        _ => {},
                    };
                },
                _ =>{},
            }
            len +=1;
            if self.op1.is_some() {
                match self.op1.clone().unwrap() {
                    Data(d)| Addr(d)=> len+=1,
                    Label(d) => len+=1,
                    _ => {},
                };
            }
            if self.op2.is_some() {
                match self.op2.clone().unwrap() {
                    Data(d)| Addr(d)=> len+=1,
                    Label(d) => len+=1,
                    _ => {},
                };
            }
        }

        len

    }

    #[allow(unused_variables)]
    pub fn fix_label(&mut self, table: &Vec<(String, u8)>) -> Result<(), String>{
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
                            Err(e) => 0,
                        };
                    }
                    self.op1 = Some(OpType::Addr(addr+add));
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
                            Err(e) => 0,
                        };
                    }
                    self.op2 = Some(OpType::Addr(addr+add));
                },
                _=>{},
            }
        }


        Ok(())
    }
}

#[allow(unused_variables)]
fn other_op(mut op: String) -> OpType {
    op.trim();
    if op.starts_with("#") {
        op.remove(0);
        if op.ends_with("h") {
            let len = op.len();
            op.remove(len -1);
            return OpType::Data(u8::from_str_radix(&op, 16).unwrap())
        }
        match u8::from_str_radix(&op, 10) {
            Ok(a) => return OpType::Data(a),
            Err(e) => {},
        }
        println!("Unknown Literal: {}",op );
    }
    if op.ends_with("h") {
        let len = op.len();
        let mut temp = op.clone();
        temp.remove(len -1);
        match u8::from_str_radix(&temp, 16){
            Ok(a) => return Addr(a),

            Err(e) => {},
        }
    }
    match u8::from_str_radix(&op, 16) {
        Ok(a) => return OpType::Addr(a),
        Err(e) => {},
    }
    OpType::Label(op)
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
