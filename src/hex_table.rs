use instruction::Instruction;
use omf::Record;
use omf::ContentRecord;

pub struct HexTable {
    pub table: Vec<u8>,
}

impl HexTable {
    pub fn new(ins:& Vec<Instruction>) -> HexTable{
        let mut max=0;
        for inst in ins {
            if inst.offset()as i32+inst.len()>max{
                max = inst.offset()as i32+inst.len();
            }
        }
        let table =  vec![0u8; max as usize];
        HexTable{table:table}
    }
    pub fn new_empty()-> HexTable{
        HexTable{table:Vec::new()}
    }
    pub fn update(&mut self, offset:u16, hex:&Vec<u8>) {
        for i in 0..(hex.len()) {
            self.table[offset as usize+i] = hex[i];
        }
    }

    pub fn append_record<T> (&mut self, record:T) where T:Record{
        self.table.append(&mut record.hex());
    }

    pub fn append_content (&mut self, record:ContentRecord) {
        self.table.append(&mut record.data());
    }
}


use std::fmt::Display;
impl Display for HexTable {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let mut out :String = "".to_string();
        for i in 0..self.table.len() {
            if i%0x10 == 0 {
                out+= &format!("\n{:x}: ",i/0x10);
            }
            out+= &format!("{:02x} ", self.table[i]);
        }
        write!(f, "{}", out)
    }
}
