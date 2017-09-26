use line::Line;
use self::Mnemonic::*;
use self::OpType::*;
use std::ops::Index;

#[derive(Debug, Clone, PartialEq, Eq)]
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
    Div,
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
    num: u64,
    pub label: Option<String>,
    mnemonic: Option<Mnemonic>,
    ops: Vec<OpType>,
}

impl Instruction {
    fn short_jmp(&self, addr: u16) -> Result<u8, String> {
        // println!("{:x}", addr+0x7d );
        // println!("{:x}", self.offset+self.len() as u16);
        // println!("{:x}", (addr as i32-(self.offset as i32+self.len() as i32)));
        if addr as u64+0x7F <  self.offset as u64+self.len() as u64{
            return Err(format!("Address out of range at {}",self.num));

        }
        Ok(((addr as u16+0x100-(self.offset as u16+self.len() as u16))%0x100)as u8)
    }

    pub fn to_hex(&self) -> Result<Vec<u8>, String> {
        match &self.mnemonic {
            &Some(Div) => self.div(),
            &Some(Mov) => self.mov(),
            &Some(Movc) => self.movc(),
            &Some(Movx) => self.movx(),
            &Some(Inc) => self.inc(),
            &Some(Dec) => self.dec(),
            &Some(Cpl) => self.cpl(),
            &Some(Cjne) => self.cjne(),
            &Some(Pop) => self.pop(),
            &Some(Push) => self.push(),
            &Some(Add) => self.add(),
            &Some(Addc) => self.addc(),
            &Some(Orl) => self.orl(),
            &Some(Anl) => self.anl(),
            &Some(Subb) => self.subb(),
            &Some(Xch) => self.xch(),
            &Some(Xrl) => self.xrl(),
            &Some(Jb) => self.jb(),
            &Some(Ret) => self.ret(),
            &Some(Clr) => self.clr(),
            &Some(Setb) => self.setb(),
            &Some(Nop) => self.nop(),
            &Some(Djnz) => self.djnz(),
            &Some(Ajmp) => self.ajmp(),
            &Some(Ljmp)=> self.ljmp(),
            &Some(Acall) => self.acall(),
            &Some(Lcall)=> self.lcall(),
            &Some(Rr) => self.rr(),
            &Some(Rrc) => self.rrc(),
            &Some(Rl) => self.rl(),
            &Some(Reti) => self.reti(),
            &Some(Rlc) => self.rlc(),
            &Some(Jc) => self.jc(),

            &Some(Jnc)=> self.jnc(),
            &Some(Jz) => self.jz(),
            &Some(Jnz) => self.jnz(),
            &Some(Jmp) => self.jmp(),
            &Some(Mul) => self.mul(),
            &Some(Swap) => self.swap(),
            &Some(Da) => self.da(),
            &Some(Jnb) => self.jnb(),
            &Some(Jbc) => self.jbc(),
            &Some(Xchd) => self.xchd(),
            &Some(Org) => Ok(vec![]),
            &Some(Cseg)=> Ok(vec![]),
            &Some(Dseg) => Ok(vec![]),
            &Some(Db) => self.db(),
            &Some(Ds)=> self.ds(),
            &Some(Sjmp)=>self.sjmp(),
            &None => Ok(vec![]),
            //&Some(ref a) => Err(format!("Unimplemented: {:?}", a)),
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
        if self.ops.len()<1 {
            return None;
        }
        match self.ops[0] {
            Addr(d) => Some(d as u16),
            Addr16(d) => Some(d),
            Label(_) => {
                match self.ops[1] {
                    Addr(d) => Some(d as u16),
                    Addr16(d) => Some(d),
                    _ =>panic!("ORG, CSEG, or DSEG has invalid addr at line {}",self.num)
                }
            },
            _ =>panic!("ORG, CSEG, or DSEG has invalid addr at line {}",self.num)
        }
    }

    pub fn from_line(line:Line, offset: u16) -> Result<Self, String> {
        let mne;
        // let op1;
        // let op2;
        let mut ops = Vec::new();

        if line.mnu.is_some() {
            mne = match line.mnu.unwrap().to_lowercase().as_ref() {
                "div" => Some(Div),
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

        // if line.ops.len()>0 {
        //     op1 = match line.ops[0].to_lowercase().as_ref() {
        //         "@r0" => Some(OpType::AtR0),
        //         "@r1" => Some(OpType::AtR1),
        //         "r1" => Some(OpType::R1),
        //         "r2" => Some(OpType::R2),
        //         "r3" => Some(OpType::R3),
        //         "r4" => Some(OpType::R4),
        //         "r5" => Some(OpType::R5),
        //         "r6" => Some(OpType::R6),
        //         "r7" => Some(OpType::R7),
        //         "r0" => Some(OpType::R0),
        //         "c" => Some(OpType::C),
        //         "a" => Some(OpType::A),
        //         "ab" => Some(OpType::AB),
        //         "@dptr" => Some(&AtDptr),
        //         "@a+dptr" => Some(&AtADptr),
        //         "@a+pc" => Some(&AtAPc),
        //         "dptr" => Some(&Dptr),
        //         "b" =>Some(Label("B".to_string())),
        //         op @ _ => {
        //             match other_op(op.to_string()){
        //                 Ok(op) => Some(op),
        //                 Err(e) => return Err(e),
        //             }
        //         },
        //     };
        // }
        // else {
        //     op1 = None;
        // }
        //
        // if line.ops.len()>1 {
        //     op2 = match line.ops[1].to_lowercase().as_ref() {
        //         "@r0" => Some(OpType::AtR0),
        //         "@r1" => Some(OpType::AtR1),
        //         "r1" => Some(OpType::R1),
        //         "r2" => Some(OpType::R2),
        //         "r3" => Some(OpType::R3),
        //         "r4" => Some(OpType::R4),
        //         "r5" => Some(OpType::R5),
        //         "r6" => Some(OpType::R6),
        //         "r7" => Some(OpType::R7),
        //         "r0" => Some(OpType::R0),
        //         "c" => Some(OpType::C),
        //         "a" => Some(OpType::A),
        //         "b" =>Some(Label("B".to_string())),
        //         "dptr" => Some(&Dptr),
        //         "@a+pc" => Some(&AtAPc),
        //         "@a+dptr" => Some(&AtADptr),
        //         "@dptr" => Some(&AtDptr),
        //         op @ _ => {
        //             match other_op(op.to_string()){
        //                 Ok(op) => Some(op),
        //                 Err(e) => return Err(e),
        //             }
        //         },
        //     };
        // }
        // else {
        //     op2 = None;
        // }

        for op in line.ops
        {
            let op_tmp= match op.to_lowercase().as_ref() {
                "@r0" => OpType::AtR0,
                "@r1" => OpType::AtR1,
                "r1" => OpType::R1,
                "r2" => OpType::R2,
                "r3" => OpType::R3,
                "r4" => OpType::R4,
                "r5" => OpType::R5,
                "r6" => OpType::R6,
                "r7" => OpType::R7,
                "r0" => OpType::R0,
                "c" => OpType::C,
                "a" => OpType::A,
                "ab" => AB,
                "b" => Label("B".to_string()),
                "dptr" => Dptr,
                "@a+pc" =>AtAPc,
                "@a+dptr" => AtADptr,
                "@dptr" => AtDptr,
                op @ _ => {
                    match other_op(op.to_string()){
                        Ok(op) => op,
                        Err(e) => return Err(e),
                    }
                },
            };
            ops.push(op_tmp);

        }

        Ok(Instruction{offset:offset, num: line.num, label:line.label, mnemonic: mne, ops:ops})
    }

    pub fn offset(&self) ->u16 {
        self.offset
    }

    pub fn len(&self) ->i32 {
        let mut len = 0;
        let mut movdptr = 0;
        if self.mnemonic.is_some() {
            match self.mnemonic.clone().unwrap(){
                Org |Cseg | Dseg=> return 0,
                Db => {
                    if self.ops.len() == 1 {
                        match self.ops[0] {
                            Label(ref l)=> return l.len() as i32,
                            _ => {},
                        };
                    }
                },
                Ds => {
                    if self.ops.len() == 1 {
                        match self.ops[0] {
                            Addr(ref l)=> return *l as i32,
                            _ => {},
                        };
                    }
                },
                Mov => movdptr +=1,
                Lcall | Ljmp => return 3,
                Cjne => return 3,
                _ =>{},
            }
            len +=1;
            if self.ops.len() > 0 {
                match self.ops[0] {
                    Data(_)| Addr(_)| Addr16(_)=> len+=1,
                    Label(_) => len+=1,
                    Data16(_) => len+=2,
                    Dptr => movdptr+=1,
                    _ => {},
                };
            }
            if self.ops.len() > 1 {
                match self.ops[1] {
                    Addr16(_)| Addr(_) => len+=1,
                    Data(_)=> {
                        len+=1;
                        if movdptr == 2 {
                            len+=1;
                        }
                    },
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
        for (i, op) in self.ops.clone().iter().enumerate() {
            match op {
                &OpType::Label(ref label_full) => {
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
                        self.ops[i] = OpType::Addr16(addr+add as u16)

                    }
                    // else {
                    //     OpType::Label(*label_full)
                    // }
                },
                _=>{},
            };


        }
        if self.ops.len()>0{

        }

        if self.ops.len()>1{
            match self.ops.clone()[1] {
                OpType::Label(ref label_full) => {
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
                    self.ops[1] = OpType::Addr16(addr+add as u16);
                },
                _=>{},
            }
        }


        Ok(())
    }

    // pub fn validate(&self) -> Result<(), String> {
    //     match &self.mnemonic {
    //         &None => Ok(()),
    //         &Some(Nop) => {
    //             if self.ops.len() > 0 {
    //                 return Err(String::from("NOP takes no operands"))
    //             }
    //             Ok(())
    //         },
    //
    //         _ => Ok(()),
    //     }
    // }
}

impl Instruction {
    fn cjne(&self) -> Result<Vec<u8>, String> {
        if self.ops.len() < 3 {
            return Err(format!("Too few arguments for CJNE: {}",self.num));
        }
        if self.ops.len() > 3 {
            return Err(format!("Too many arguments for CJNE: {}",self.num));
        }
        match (self.ops.index(0), self.ops.index(1), self.ops.index(2)) {
            (&A, &Data(d), &Addr16(a)) => Ok(vec![0xB4,d,self.short_jmp(a)?]),
            (&A, &Addr(d), &Addr16(a)) => Ok(vec![0xB5,d as u8,self.short_jmp(a)?]),
            (&AtR0, &Data(d), &Addr16(a)) => Ok(vec![0xB6,d,self.short_jmp(a)?]),
            (&AtR1, &Data(d), &Addr16(a)) => Ok(vec![0xB7,d,self.short_jmp(a)?]),
            (&R0, &Data(d), &Addr16(a)) => Ok(vec![0xB8,d,self.short_jmp(a)?]),
            (&R1, &Data(d), &Addr16(a)) => Ok(vec![0xB9,d,self.short_jmp(a)?]),
            (&R2, &Data(d), &Addr16(a)) => Ok(vec![0xBA,d,self.short_jmp(a)?]),
            (&R3, &Data(d), &Addr16(a)) => Ok(vec![0xBB,d,self.short_jmp(a)?]),
            (&R4, &Data(d), &Addr16(a)) => Ok(vec![0xBC,d,self.short_jmp(a)?]),
            (&R5, &Data(d), &Addr16(a)) => Ok(vec![0xBD,d,self.short_jmp(a)?]),
            (&R6, &Data(d), &Addr16(a)) => Ok(vec![0xBE,d,self.short_jmp(a)?]),
            (&R7, &Data(d), &Addr16(a)) => Ok(vec![0xBF,d,self.short_jmp(a)?]),
            (ref a@_, ref b@_,ref c@_) => Err(format!("Invalid operation: CJNE {:?},{:?},{:?}", a,b,c)),
        }
    }

    fn clr(&self) -> Result<Vec<u8>, String> {
        if self.ops.len() < 1{
            return Err(format!("Too few arguments for CLR: {}",self.num));
        }
        if self.ops.len() > 1 {
            return Err(format!("Too many arguments for CLR: {}",self.num));
        }
        match self.ops[0] {
            A => Ok(vec![0xE4]),
            Addr(d) => Ok(vec![0xC2,d]),
            C => Ok(vec![0xC3]),
            ref a@_ => Err(format!("Invalid operation: CLR {:?}", a)),
        }
    }

    fn setb(&self) -> Result<Vec<u8>, String> {
        if self.ops.len() < 1{
            return Err(format!("Too few arguments for SETB: {}",self.num));
        }
        if self.ops.len() > 1 {
            return Err(format!("Too many arguments for SETB: {}",self.num));
        }
        match self.ops[0] {
            Addr(d)=> Ok(vec![0xD2,d]),
            C => Ok(vec![0xD3]),
            ref a@_ => Err(format!("Invalid operation: SETB {:?}", a)),
        }
    }

    fn cpl(&self) -> Result<Vec<u8>, String> {
        if self.ops.len() < 1{
            return Err(format!("Too few arguments for CPL: {}",self.num));
        }
        if self.ops.len() > 1{
            return Err(format!("Too many arguments for CPL: {}",self.num));
        }
        match self.ops[0] {
            Addr(d) => Ok(vec![0xB2,d]),
            C => Ok(vec![0xB3]),
            A => Ok(vec![0xF4]),
            ref a@_ => Err(format!("Invalid operation: CPL {:?}", a)),
        }
    }

    fn djnz(&self) -> Result<Vec<u8>, String> {
        let op = "DJNZ";
        if self.ops.len() < 2 {
            return Err(format!("Too few arguments for {}: {}",op,self.num));
        }
        if self.ops.len() > 2 {
            return Err(format!("Too many arguments for {}: {}",op,self.num));
        }
        match (self.ops.index(0),self.ops.index(1)) {
            (&R0,&Addr16(d)) => Ok(vec![0xD8,self.short_jmp(d)?]),
            (&R1,&Addr16(d)) => Ok(vec![0xD9,self.short_jmp(d)?]),
            (&R2,&Addr16(d)) => Ok(vec![0xDA,self.short_jmp(d)?]),
            (&R3,&Addr16(d)) => Ok(vec![0xDB,self.short_jmp(d)?]),
            (&R4,&Addr16(d)) => Ok(vec![0xDC,self.short_jmp(d)?]),
            (&R5,&Addr16(d)) => Ok(vec![0xDD,self.short_jmp(d)?]),
            (&R6,&Addr16(d)) => Ok(vec![0xDE,self.short_jmp(d)?]),
            (&R7,&Addr16(d)) => Ok(vec![0xDF,self.short_jmp(d)?]),
            (&Addr(d),&Addr16(a)) => Ok(vec![0xD5,d,self.short_jmp(a)?]),
            (a@_,b@_) => Err(format!("Invalid operation: {} {:?},{:?}",op, a,b)),
        }
    }

    fn mov(&self) -> Result<Vec<u8>, String> {
        let op = "MOV";
        if self.ops.len() < 2 {
            return Err(format!("Too few arguments for {}: {}",op,self.num));
        }
        if self.ops.len() > 2  {
            return Err(format!("Too many arguments for {}: {}",op,self.num));
        }
        match (self.ops.index(0),self.ops.index(1)) {
            (&Addr(d),&A) => Ok(vec![0xF5,d]),
            (&AtR0,&A) => Ok(vec![0xF6]),
            (&AtR1,&A) => Ok(vec![0xF7]),
            (&R0,&A) => Ok(vec![0xF8]),
            (&R1,&A) => Ok(vec![0xF9]),
            (&R2,&A) => Ok(vec![0xFA]),
            (&R3,&A) => Ok(vec![0xFB]),
            (&R4,&A) => Ok(vec![0xFC]),
            (&R5,&A) => Ok(vec![0xFD]),
            (&R6,&A) => Ok(vec![0xFE]),
            (&R7,&A) => Ok(vec![0xFF]),
            (&A,&Data(d)) => Ok(vec![0x74,d]),
            (&Addr(a),&Data(d)) => Ok(vec![0x75,a,d]),
            (&AtR0,&Data(d)) => Ok(vec![0x76,d]),
            (&AtR1,&Data(d)) => Ok(vec![0x77,d]),
            (&R0,&Data(d)) => Ok(vec![0x78,d]),
            (&R1,&Data(d)) => Ok(vec![0x79,d]),
            (&R2,&Data(d)) => Ok(vec![0x7A,d]),
            (&R3,&Data(d)) => Ok(vec![0x7B,d]),
            (&R4,&Data(d)) => Ok(vec![0x7C,d]),
            (&R5,&Data(d)) => Ok(vec![0x7D,d]),
            (&R6,&Data(d)) => Ok(vec![0x7E,d]),
            (&R7,&Data(d)) => Ok(vec![0x7F,d]),
            (&Addr(d),&AB) => Ok(vec![0x84,d]),
            (&Addr(d),&Addr(a)) => Ok(vec![0x85,a,d]),
            (&Addr(d),&AtR0) => Ok(vec![0x86,d]),
            (&Addr(d),&AtR1) => Ok(vec![0x87,d]),
            (&Addr(d),&R0) => Ok(vec![0x88,d]),
            (&Addr(d),&R1) => Ok(vec![0x89,d]),
            (&Addr(d),&R2) => Ok(vec![0x8A,d]),
            (&Addr(d),&R3) => Ok(vec![0x8B,d]),
            (&Addr(d),&R4) => Ok(vec![0x8C,d]),
            (&Addr(d),&R5) => Ok(vec![0x8D,d]),
            (&Addr(d),&R6) => Ok(vec![0x8E,d]),
            (&Addr(d),&R7) => Ok(vec![0x8F,d]),
            (&A,&Addr(d)) => Ok(vec![0xE5,d]),
            (&A,&AtR0) => Ok(vec![0xE6]),
            (&A,&AtR1) => Ok(vec![0xE7]),
            (&A,&R0) => Ok(vec![0xE8]),
            (&A,&R1) => Ok(vec![0xE9]),
            (&A,&R2) => Ok(vec![0xEA]),
            (&A,&R3) => Ok(vec![0xEB]),
            (&A,&R4) => Ok(vec![0xEC]),
            (&A,&R5) => Ok(vec![0xED]),
            (&A,&R6) => Ok(vec![0xEE]),
            (&A,&R7) => Ok(vec![0xEF]),
            (&Dptr,&Data16(d)) => Ok(vec![0x90,(d/0x100)as u8,(d%0x100) as u8]),
            (&Dptr,&Data(d)) => Ok(vec![0x90,0x00,d]),

            (&Addr(d),&C) => Ok(vec![0x92,d]),
            (&C,&Addr(d)) => Ok(vec![0xA2,d]),

            (&AtR0,&Addr(d)) => Ok(vec![0xA6,d]),
            (&AtR1,&Addr(d)) => Ok(vec![0xA7,d]),
            (&R0,&Addr(d)) => Ok(vec![0xA8,d]),
            (&R1,&Addr(d)) => Ok(vec![0xA9,d]),
            (&R2,&Addr(d)) => Ok(vec![0xAA,d]),
            (&R3,&Addr(d)) => Ok(vec![0xAB,d]),
            (&R4,&Addr(d)) => Ok(vec![0xAC,d]),
            (&R5,&Addr(d)) => Ok(vec![0xAD,d]),
            (&R6,&Addr(d)) => Ok(vec![0xAE,d]),
            (&R7,&Addr(d)) => Ok(vec![0xAF,d]),
            (a@_,b@_) => Err(format!("Invalid operation: {} {:?},{:?}",op, a,b)),
        }
    }

    fn inc(&self) -> Result<Vec<u8>, String> {
        let op ="INC";
        if self.ops.len() < 1 {
            return Err(format!("Too few arguments for {}: {}",op,self.num));
        }
        if self.ops.len() > 1 {
            return Err(format!("Too many arguments for {}: {}",op,self.num));
        }
        match self.ops[0] {
            A => Ok(vec![0x04]),
            Addr(d) => Ok(vec![0x05,d]),
            AtR0 => Ok(vec![0x06]),
            AtR1 => Ok(vec![0x07]),
            R0 => Ok(vec![0x08]),
            R1 => Ok(vec![0x09]),
            R2 => Ok(vec![0x0A]),
            R3 => Ok(vec![0x0B]),
            R4 => Ok(vec![0x0C]),
            R5 => Ok(vec![0x0D]),
            R6 => Ok(vec![0x0E]),
            R7 => Ok(vec![0x0F]),
            Dptr => Ok(vec![0xA3]),
            ref a@_ => Err(format!("Invalid operation: {} {:?}",op, a)),
        }
    }

    fn dec(&self) -> Result<Vec<u8>, String> {
        let op ="DEC";
        if self.ops.len() < 1 {
            return Err(format!("Too few arguments for {}: {}",op,self.num));
        }
        if self.ops.len() > 1 {
            return Err(format!("Too many arguments for {}: {}",op,self.num));
        }
        match self.ops[0] {
            A => Ok(vec![0x14]),
            Addr(d) => Ok(vec![0x15,d]),
            AtR0 => Ok(vec![0x16]),
            AtR1 => Ok(vec![0x17]),
            R0 => Ok(vec![0x18]),
            R1 => Ok(vec![0x19]),
            R2 => Ok(vec![0x1A]),
            R3 => Ok(vec![0x1B]),
            R4 => Ok(vec![0x1C]),
            R5 => Ok(vec![0x1D]),
            R6 => Ok(vec![0x1E]),
            R7 => Ok(vec![0x1F]),
            ref a@_ => Err(format!("Invalid operation: {} {:?}",op, a)),
        }
    }

    fn pop(&self) -> Result<Vec<u8>, String> {
        let op ="POP";
        if self.ops.len() < 1 {
            return Err(format!("Too few arguments for {}: {}",op,self.num));
        }
        if self.ops.len() > 1 {
            return Err(format!("Too many arguments for {}: {}",op,self.num));
        }
        match self.ops[0] {
            Addr(d) => Ok(vec![0xD0,d]),
            ref a@_ => Err(format!("Invalid operation: {} {:?}",op, a)),
        }
    }

    fn push(&self) -> Result<Vec<u8>, String> {
        let op ="PUSH";
        if self.ops.len() < 1 {
            return Err(format!("Too few arguments for {}: {}",op,self.num));
        }
        if self.ops.len() > 1 {
            return Err(format!("Too many arguments for {}: {}",op,self.num));
        }
        match self.ops[0] {
            Addr(d) => Ok(vec![0xC0,d]),

            ref a@_ => Err(format!("Invalid operation: {} {:?}",op, a)),
        }
    }

    fn add(&self) -> Result<Vec<u8>, String> {
        let op = "ADD";
        if self.ops.len() < 2 {
            return Err(format!("Too few arguments for {}: {}",op,self.num));
        }
        if self.ops.len() > 2 {
            return Err(format!("Too many arguments for {}: {}",op,self.num));
        }
        match (self.ops.index(0),self.ops.index(1)) {
            (&A,&Data(d)) => Ok(vec![0x24,d]),
            (&A,&Addr(d)) => Ok(vec![0x25,d]),
            (&A,&AtR0) => Ok(vec![0x26]),
            (&A,&AtR1) => Ok(vec![0x27]),
            (&A,&R0) => Ok(vec![0x28]),
            (&A,&R1) => Ok(vec![0x29]),
            (&A,&R2) => Ok(vec![0x2A]),
            (&A,&R3) => Ok(vec![0x2B]),
            (&A,&R4) => Ok(vec![0x2C]),
            (&A,&R5) => Ok(vec![0x2D]),
            (&A,&R6) => Ok(vec![0x2E]),
            (&A,&R7) => Ok(vec![0x2F]),
            (a@_,b@_) => Err(format!("Invalid operation: {} {:?},{:?}",op, a,b)),
        }
    }

    fn addc(&self) -> Result<Vec<u8>, String> {
        let op = "ADDC";
        if self.ops.len() < 2 {
            return Err(format!("Too few arguments for {}: {}",op,self.num));
        }
        if self.ops.len() > 2 {
            return Err(format!("Too many arguments for {}: {}",op,self.num));
        }
        match (self.ops.index(0),self.ops.index(1)) {
            (&A,&Data(d)) => Ok(vec![0x34,d]),
            (&A,&Addr(d)) => Ok(vec![0x35,d]),
            (&A,&AtR0) => Ok(vec![0x36]),
            (&A,&AtR1) => Ok(vec![0x37]),
            (&A,&R0) => Ok(vec![0x38]),
            (&A,&R1) => Ok(vec![0x39]),
            (&A,&R2) => Ok(vec![0x3A]),
            (&A,&R3) => Ok(vec![0x3B]),
            (&A,&R4) => Ok(vec![0x3C]),
            (&A,&R5) => Ok(vec![0x3D]),
            (&A,&R6) => Ok(vec![0x3E]),
            (&A,&R7) => Ok(vec![0x3F]),
            (a@_,b@_) => Err(format!("Invalid operation: {} {:?},{:?}",op, a,b)),
        }
    }

    fn orl(&self) -> Result<Vec<u8>, String> {
        let op = "ORL";
        if self.ops.len() < 2 {
            return Err(format!("Too few arguments for {}: {}",op,self.num));
        }
        if self.ops.len() > 2 {
            return Err(format!("Too many arguments for {}: {}",op,self.num));
        }
        match (self.ops.index(0),self.ops.index(1)) {
            (&Addr(a),&A) => Ok(vec![0x42,a]),
            (&Addr(a),&Data(d)) => Ok(vec![0x43,a,d]),
            (&A,&Data(d)) => Ok(vec![0x44,d]),
            (&A,&Addr(d)) => Ok(vec![0x45,d]),
            (&A,&AtR0) => Ok(vec![0x46]),
            (&A,&AtR1) => Ok(vec![0x47]),
            (&A,&R0) => Ok(vec![0x48]),
            (&A,&R1) => Ok(vec![0x49]),
            (&A,&R2) => Ok(vec![0x4A]),
            (&A,&R3) => Ok(vec![0x4B]),
            (&A,&R4) => Ok(vec![0x4C]),
            (&A,&R5) => Ok(vec![0x4D]),
            (&A,&R6) => Ok(vec![0x4E]),
            (&A,&R7) => Ok(vec![0x4F]),
            (&C,&Addr(d)) => Ok(vec![0x72,d]),
            (a@_,b@_) => Err(format!("Invalid operation: {} {:?},{:?}",op, a,b)),
        }
    }

    fn anl(&self) -> Result<Vec<u8>, String> {
        let op = "ANL";
        if self.ops.len() < 2 {
            return Err(format!("Too few arguments for {}: {}",op,self.num));
        }
        if self.ops.len() > 2 {
            return Err(format!("Too many arguments for {}: {}",op,self.num));
        }
        match (self.ops.index(0),self.ops.index(1)) {
            (&Addr(a),&A) => Ok(vec![0x52,a]),
            (&Addr(a),&Data(d)) => Ok(vec![0x53,a,d]),
            (&A,&Data(d)) => Ok(vec![0x54,d]),
            (&A,&Addr(d)) => Ok(vec![0x55,d]),
            (&A,&AtR0) => Ok(vec![0x56]),
            (&A,&AtR1) => Ok(vec![0x57]),
            (&A,&R0) => Ok(vec![0x58]),
            (&A,&R1) => Ok(vec![0x59]),
            (&A,&R2) => Ok(vec![0x5A]),
            (&A,&R3) => Ok(vec![0x5B]),
            (&A,&R4) => Ok(vec![0x5C]),
            (&A,&R5) => Ok(vec![0x5D]),
            (&A,&R6) => Ok(vec![0x5E]),
            (&A,&R7) => Ok(vec![0x5F]),
            (&C,&Addr(d)) => Ok(vec![0x82,d]),

            (a@_,b@_) => Err(format!("Invalid operation: {} {:?},{:?}",op, a,b)),
        }
    }

    fn subb(&self) -> Result<Vec<u8>, String> {
        let op = "SUBB";
        if self.ops.len() < 2 {
            return Err(format!("Too few arguments for {}: {}",op,self.num));
        }
        if self.ops.len() > 2 {
            return Err(format!("Too many arguments for {}: {}",op,self.num));
        }
        match (self.ops.index(0),self.ops.index(1)) {
            (&A,&Data(d)) => Ok(vec![0x94,d]),
            (&A,&Addr(d)) => Ok(vec![0x95,d]),
            (&A,&AtR0) => Ok(vec![0x96]),
            (&A,&AtR1) => Ok(vec![0x97]),
            (&A,&R0) => Ok(vec![0x98]),
            (&A,&R1) => Ok(vec![0x99]),
            (&A,&R2) => Ok(vec![0x9A]),
            (&A,&R3) => Ok(vec![0x9B]),
            (&A,&R4) => Ok(vec![0x9C]),
            (&A,&R5) => Ok(vec![0x9D]),
            (&A,&R6) => Ok(vec![0x9E]),
            (&A,&R7) => Ok(vec![0x9F]),

            (a@_,b@_) => Err(format!("Invalid operation: {} {:?},{:?}",op, a,b)),
        }
    }

    fn xch(&self) -> Result<Vec<u8>, String> {
        let op = "XCH";
        if self.ops.len() < 2 {
            return Err(format!("Too few arguments for {}: {}",op,self.num));
        }
        if self.ops.len() > 2 {
            return Err(format!("Too many arguments for {}: {}",op,self.num));
        }
        match (self.ops.index(0),self.ops.index(1)) {
            (&A,&Addr(d)) => Ok(vec![0xC5,d]),
            (&A,&AtR0) => Ok(vec![0xC6]),
            (&A,&AtR1) => Ok(vec![0xC7]),
            (&A,&R0) => Ok(vec![0xC8]),
            (&A,&R1) => Ok(vec![0xC9]),
            (&A,&R2) => Ok(vec![0xCA]),
            (&A,&R3) => Ok(vec![0xCB]),
            (&A,&R4) => Ok(vec![0xCC]),
            (&A,&R5) => Ok(vec![0xCD]),
            (&A,&R6) => Ok(vec![0xCE]),
            (&A,&R7) => Ok(vec![0xCF]),

            (a@_,b@_) => Err(format!("Invalid operation: {} {:?},{:?}",op, a,b)),
        }
    }

    fn xrl(&self) -> Result<Vec<u8>, String> {
        let op = "XRL";
        if self.ops.len() < 2 {
            return Err(format!("Too few arguments for {}: {}",op,self.num));
        }
        if self.ops.len() > 2 {
            return Err(format!("Too many arguments for {}: {}",op,self.num));
        }
        match (self.ops.index(0),self.ops.index(1)) {
            (&Addr(d),&A) => Ok(vec![0x62,d]),
            (&Addr(a),&Data(d)) => Ok(vec![0x63,a,d]),
            (&A,&Data(d)) => Ok(vec![0x64,d]),
            (&A,&Addr(d)) => Ok(vec![0x65,d]),
            (&A,&AtR0) => Ok(vec![0x66]),
            (&A,&AtR1) => Ok(vec![0x67]),
            (&A,&R0) => Ok(vec![0x68]),
            (&A,&R1) => Ok(vec![0x69]),
            (&A,&R2) => Ok(vec![0x6A]),
            (&A,&R3) => Ok(vec![0x6B]),
            (&A,&R4) => Ok(vec![0x6C]),
            (&A,&R5) => Ok(vec![0x6D]),
            (&A,&R6) => Ok(vec![0x6E]),
            (&A,&R7) => Ok(vec![0x6F]),

            (a@_,b@_) => Err(format!("Invalid operation: {} {:?},{:?}",op, a,b)),
        }
    }

    fn movx(&self) -> Result<Vec<u8>, String> {
        let op = "MOVX";
        if self.ops.len() < 2 {
            return Err(format!("Too few arguments for {}: {}",op,self.num));
        }
        if self.ops.len() > 2 {
            return Err(format!("Too many arguments for {}: {}",op,self.num));
        }
        match (self.ops.index(0),self.ops.index(1)) {
            (&A,&AtDptr) => Ok(vec![0xE0]),
            // (&Some(&Acall),&Addr16(d),&None) => Ok(vec![0xE1,d as u8]),
            (&A,&AtR0) => Ok(vec![0xE2]),
            (&A,&AtR1) => Ok(vec![0xE3]),


            (&AtDptr,&A) => Ok(vec![0xF0]),
            // (&Some(&Acall),&Addr16(d),&None) => Ok(vec![0xF1,d as u8]),
            (&AtR0,&A) => Ok(vec![0xF2]),
            (&AtR1,&A) => Ok(vec![0xF3]),
            (a@_,b@_) => Err(format!("Invalid operation: {} {:?},{:?}",op, a,b)),
        }
    }

    fn movc(&self) -> Result<Vec<u8>, String> {
        let op = "MOVC";
        if self.ops.len() < 2 {
            return Err(format!("Too few arguments for {}: {}",op,self.num));
        }
        if self.ops.len() > 2 {
            return Err(format!("Too many arguments for {}: {}",op,self.num));
        }
        match (self.ops.index(0),self.ops.index(1)) {
            (&A,&AtAPc) => Ok(vec![0x83]),
            (&A,&AtADptr) => Ok(vec![0x93]),
            (a@_,b@_) => Err(format!("Invalid operation: {} {:?},{:?}",op, a,b)),
        }
    }

    fn ret(&self) -> Result<Vec<u8>, String> {
        let op = "RET";
        if self.ops.len() > 0 {
            return Err(format!("Too many arguments for {}: {}",op,self.num));
        }
        Ok(vec![0x22])
    }

    fn nop(&self) -> Result<Vec<u8>, String> {
        let op = "NOP";
        if self.ops.len() > 0 {
            return Err(format!("Too many arguments for {}: {}",op,self.num));
        }
        Ok(vec![0x00])
    }

    fn jb(&self) -> Result<Vec<u8>, String> {
        let op = "JB";
        if self.ops.len() < 2 {
            return Err(format!("Too few arguments for {}: {}",op,self.num));
        }
        if self.ops.len() > 2 {
            return Err(format!("Too many arguments for {}: {}",op,self.num));
        }
        match (self.ops.index(0),self.ops.index(1)) {
            (&Addr(a),&Addr16(d)) => Ok(vec![0x20,a,self.short_jmp(d)?]),
            (a@_,b@_) => Err(format!("Invalid operation: {} {:?},{:?}",op, a,b)),
        }
    }

    fn ajmp(&self) -> Result<Vec<u8>, String> {
        let op = "AJMP";
        if self.ops.len() < 1 {
            return Err(format!("Too few arguments for {}: {}",op,self.num));
        }
        if self.ops.len() > 1 {
            return Err(format!("Too many arguments for {}: {}",op,self.num));
        }
        match self.ops[0] {
            Addr16(d) => Ok(vec![0x01,d as u8]),
            ref a@_ => Err(format!("Invalid operation: {} {:?}",op, a)),
        }
    }

    fn ljmp(&self) -> Result<Vec<u8>, String> {
        let op = "LJMP";
        if self.ops.len() < 1 {
            return Err(format!("Too few arguments for {}: {}",op,self.num));
        }
        if self.ops.len() > 1 {
            return Err(format!("Too many arguments for {}: {}",op,self.num));
        }
        match self.ops[0] {
            Addr16(d) => Ok(vec![0x02,(d/0x100) as u8, (d%100) as u8]),
            ref a@_ => Err(format!("Invalid operation: {} {:?}",op, a)),
        }
    }

    fn acall(&self) -> Result<Vec<u8>, String> {
        let op = "ACALL";
        if self.ops.len() < 1 {
            return Err(format!("Too few arguments for {}: {}",op,self.num));
        }
        if self.ops.len() > 1 {
            return Err(format!("Too many arguments for {}: {}",op,self.num));
        }
        match self.ops[0] {
            Addr16(d) => Ok(vec![0x11,d as u8]),
            ref a@_ => Err(format!("Invalid operation: {} {:?}",op, a)),
        }
    }

    fn lcall(&self) -> Result<Vec<u8>, String> {
        let op = "LCALL";
        if self.ops.len() < 1 {
            return Err(format!("Too few arguments for {}: {}",op,self.num));
        }
        if self.ops.len() > 1 {
            return Err(format!("Too many arguments for {}: {}",op,self.num));
        }
        match self.ops[0] {
            Addr16(d) => Ok(vec![0x12,(d/0x100) as u8, (d%100) as u8]),
            ref a@_ => Err(format!("Invalid operation: {} {:?}",op, a)),
        }
    }

    fn sjmp(&self) -> Result<Vec<u8>, String> {
        let op = "SJMP";
        if self.ops.len() < 1 {
            return Err(format!("Too few arguments for {}: {}",op,self.num));
        }
        if self.ops.len() > 1 {
            return Err(format!("Too many arguments for {}: {}",op,self.num));
        }
        match self.ops[0] {
            Addr16(d) => Ok(vec![0x80,self.short_jmp(d)?]),
            ref a@_ => Err(format!("Invalid operation: {} {:?}",op, a)),
        }
    }

    fn rr(&self) -> Result<Vec<u8>, String> {
        let op = "rr";
        if self.ops.len() < 1 {
            return Err(format!("Too few arguments for {}: {}",op,self.num));
        }
        if self.ops.len() > 1 {
            return Err(format!("Too many arguments for {}: {}",op,self.num));
        }
        match self.ops[0] {
            A => Ok(vec![0x03]),
            ref a@_ => Err(format!("Invalid operation: {} {:?}",op, a)),
        }
    }

    fn rrc(&self) -> Result<Vec<u8>, String> {
        let op = "RRC";
        if self.ops.len() < 1 {
            return Err(format!("Too few arguments for {}: {}",op,self.num));
        }
        if self.ops.len() > 1 {
            return Err(format!("Too many arguments for {}: {}",op,self.num));
        }
        match self.ops[0] {
            A => Ok(vec![0x13]),
            ref a@_ => Err(format!("Invalid operation: {} {:?}",op, a)),
        }
    }

    fn rl(&self) -> Result<Vec<u8>, String> {
        let op = "RL";
        if self.ops.len() < 1 {
            return Err(format!("Too few arguments for {}: {}",op,self.num));
        }
        if self.ops.len() > 1 {
            return Err(format!("Too many arguments for {}: {}",op,self.num));
        }
        match self.ops[0] {
            A => Ok(vec![0x23]),
            ref a@_ => Err(format!("Invalid operation: {} {:?}",op, a)),
        }
    }

    fn reti(&self) -> Result<Vec<u8>, String> {
        let op = "RETI";
        if self.ops.len() > 0 {
            return Err(format!("Too many arguments for {}: {}",op,self.num));
        }
        Ok(vec![0x32])
    }

    fn rlc(&self) -> Result<Vec<u8>, String> {
        let op = "RLC";
        if self.ops.len() < 1 {
            return Err(format!("Too few arguments for {}: {}",op,self.num));
        }
        if self.ops.len() > 1 {
            return Err(format!("Too many arguments for {}: {}",op,self.num));
        }
        match self.ops[0] {
            A => Ok(vec![0x33]),
            ref a@_ => Err(format!("Invalid operation: {} {:?}",op, a)),
        }
    }

    fn jc(&self) -> Result<Vec<u8>, String> {
        let op = "JC";
        if self.ops.len() < 1 {
            return Err(format!("Too few arguments for {}: {}",op,self.num));
        }
        if self.ops.len() > 1 {
            return Err(format!("Too many arguments for {}: {}",op,self.num));
        }
        match self.ops[0] {
            Addr16(d) => Ok(vec![0x40,self.short_jmp(d)?]),
            ref a@_ => Err(format!("Invalid operation: {} {:?}",op, a)),
        }
    }

    fn jnc(&self) -> Result<Vec<u8>, String> {
        let op = "JNC";
        if self.ops.len() < 1 {
            return Err(format!("Too few arguments for {}: {}",op,self.num));
        }
        if self.ops.len() > 1 {
            return Err(format!("Too many arguments for {}: {}",op,self.num));
        }
        match self.ops[0] {
            Addr16(d) => Ok(vec![0x50,self.short_jmp(d)?]),
            ref a@_ => Err(format!("Invalid operation: {} {:?}",op, a)),
        }
    }

    fn jz(&self) -> Result<Vec<u8>, String> {
        let op = "JZ";
        if self.ops.len() < 1 {
            return Err(format!("Too few arguments for {}: {}",op,self.num));
        }
        if self.ops.len() > 1 {
            return Err(format!("Too many arguments for {}: {}",op,self.num));
        }
        match self.ops[0] {
            Addr16(d) => Ok(vec![0x60,self.short_jmp(d)?]),
            ref a@_ => Err(format!("Invalid operation: {} {:?}",op, a)),
        }
    }

    fn jnz(&self) -> Result<Vec<u8>, String> {
        let op = "JNZ";
        if self.ops.len() < 1 {
            return Err(format!("Too few arguments for {}: {}",op,self.num));
        }
        if self.ops.len() > 1 {
            return Err(format!("Too many arguments for {}: {}",op,self.num));
        }
        match self.ops[0] {
            Addr16(d) => Ok(vec![0x70,self.short_jmp(d)?]),
            ref a@_ => Err(format!("Invalid operation: {} {:?}",op, a)),
        }
    }

    fn jmp(&self) -> Result<Vec<u8>, String> {
        let op = "JMP";
        if self.ops.len() < 1 {
            return Err(format!("Too few arguments for {}: {}",op,self.num));
        }
        if self.ops.len() > 1 {
            return Err(format!("Too many arguments for {}: {}",op,self.num));
        }
        match self.ops[0] {
            AtADptr => Ok(vec![0x73]),
            ref a@_ => Err(format!("Invalid operation: {} {:?}",op, a)),
        }
    }

    fn mul(&self) -> Result<Vec<u8>, String> {
        let op = "MUL";
        if self.ops.len() < 1 {
            return Err(format!("Too few arguments for {}: {}",op,self.num));
        }
        if self.ops.len() > 1 {
            return Err(format!("Too many arguments for {}: {}",op,self.num));
        }
        match self.ops[0] {
            AB => Ok(vec![0xA4]),
            ref a@_ => Err(format!("Invalid operation: {} {:?}",op, a)),
        }
    }

    fn div(&self) -> Result<Vec<u8>, String> {
        let op = "DIV";
        if self.ops.len() < 1 {
            return Err(format!("Too few arguments for {}: {}",op,self.num));
        }
        if self.ops.len() > 1 {
            return Err(format!("Too many arguments for {}: {}",op,self.num));
        }
        match self.ops[0] {
            AB => Ok(vec![0x84]),
            ref a@_ => Err(format!("Invalid operation: {} {:?}",op, a)),
        }
    }

    fn swap(&self) -> Result<Vec<u8>, String> {
        let op = "SWAP";
        if self.ops.len() < 1 {
            return Err(format!("Too few arguments for {}: {}",op,self.num));
        }
        if self.ops.len() > 1 {
            return Err(format!("Too many arguments for {}: {}",op,self.num));
        }
        match self.ops[0] {
            A => Ok(vec![0xC4]),
            ref a@_ => Err(format!("Invalid operation: {} {:?}",op, a)),
        }
    }

    fn da(&self) -> Result<Vec<u8>, String> {
        let op = "DA";
        if self.ops.len() < 1 {
            return Err(format!("Too few arguments for {}: {}",op,self.num));
        }
        if self.ops.len() > 1 {
            return Err(format!("Too many arguments for {}: {}",op,self.num));
        }
        match self.ops[0] {
            A => Ok(vec![0xD4]),
            ref a@_ => Err(format!("Invalid operation: {} {:?}",op, a)),
        }
    }

    fn jnb(&self) -> Result<Vec<u8>, String> {
        let op = "JNB";
        if self.ops.len() < 2 {
            return Err(format!("Too few arguments for {}: {}",op,self.num));
        }
        if self.ops.len() > 2 {
            return Err(format!("Too many arguments for {}: {}",op,self.num));
        }
        match (self.ops.index(0), self.ops.index(1)) {
            (&Addr(a),&Addr16(d)) => Ok(vec![0x30,a,self.short_jmp(d)?]),
            (a@_,b@_) => Err(format!("Invalid operation: {} {:?},{:?}",op, a,b)),
        }
    }

    fn jbc(&self) -> Result<Vec<u8>, String> {
        let op = "JBC";
        if self.ops.len() < 2 {
            return Err(format!("Too few arguments for {}: {}",op,self.num));
        }
        if self.ops.len() > 2 {
            return Err(format!("Too many arguments for {}: {}",op,self.num));
        }
        match (self.ops.index(0),self.ops.index(1)) {
            (&Addr(b),&Addr16(d)) => Ok(vec![0x10,b,self.short_jmp(d)?]),
            (ref a@_,ref b@_) => Err(format!("Invalid operation: {} {:?},{:?}",op, a,b)),
        }
    }

    fn xchd(&self) -> Result<Vec<u8>, String> {
        let op = "XCHD";
        if self.ops.len() < 2 {
            return Err(format!("Too few arguments for {}: {}",op,self.num));
        }
        if self.ops.len() > 2 {
            return Err(format!("Too many arguments for {}: {}",op,self.num));
        }
        match (self.ops.index(0),self.ops.index(1)) {
            (&A,&AtR0) => Ok(vec![0xD6]),
            (&A,&AtR1) => Ok(vec![0xD7]),
            (ref a@_,ref b@_) => Err(format!("Invalid operation: {} {:?},{:?}",op, a,b)),
        }
    }

    fn db(&self) -> Result<Vec<u8>, String> {
        //TODO: make this better
        let op = "DB";
        if self.ops.len() < 1 {
            return Err(format!("Too few arguments for {}: {}",op,self.num));
        }
        if self.ops.len() > 2 {
            return Err(format!("Too many arguments for {}: {}",op,self.num));
        }
        match self.ops[0] {
            Label(ref l) => {
                let mut v = l.clone().into_bytes();
                v.remove(l.len()-1);
                v.remove(0);
                v.push(0);
                Ok(v)
            },
            ref a@_ => Err(format!("Invalid operation: {} {:?}",op, a)),
        }
    }

    fn ds(&self) -> Result<Vec<u8>, String> {
        //TODO: make this better
        let op = "DS";
        if self.ops.len() < 1 {
            return Err(format!("Too few arguments for {}: {}",op,self.num));
        }
        if self.ops.len() > 1 {
            return Err(format!("Too many arguments for {}: {}",op,self.num));
        }
        match self.ops[0] {
            Addr(d) => Ok(vec![0x10;d as usize]),
            ref a@_ => Err(format!("Invalid operation: {} {:?}",op, a)),
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
