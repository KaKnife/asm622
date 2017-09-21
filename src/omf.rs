fn checksome(vec: &Vec<u8>)->u8{
    let mut sum = 0u64;
    for v in vec {
        sum+=*v as u64;
    }
    let mut sum8 = (sum%256) as u8;
    sum8 ^= 0xFF;
    if sum8 == 0xFF{
        return 0;
    }
    sum8+1
}

pub trait Record {
    fn hex(&self) -> Vec<u8>;
    fn len(&self) -> u16;
    const TYPE: u8;
}

pub struct HeaderRecord {
    pub name: String,
}

impl HeaderRecord {

    //const ID: u16 = 622;
    pub fn new(name: String) -> HeaderRecord{
        HeaderRecord{name:name}
    }
}

impl Record for HeaderRecord {
    const TYPE: u8 = 0x02;
    fn len(&self) -> u16 {
        self.name.len() as u16 + 4
    }

    fn hex (&self) ->Vec<u8>{
        let mut vec = Vec::new();
        vec.push(HeaderRecord::TYPE);
        vec.push((self.len()%0xFF) as u8);
        vec.push((self.len()/0xFF) as u8);
        vec.push(self.name.len() as u8);
        let name_vec = self.name.as_bytes();

        for c in name_vec {
            vec.push(*c);
        }
        vec.push(0xFF);
        vec.push(0x00);
        let chk_some = checksome(&vec);
        vec.push(chk_some);
        vec
    }
}

pub struct EndRecord {
    pub name: String,
    reg_msk: u8,
}

impl EndRecord {
    pub fn new(name: String, bank:(bool, bool, bool,  bool)) -> EndRecord{
        let reg_msk = 0b1000*bank.3 as u8+0b100*bank.2 as u8+0b10*bank.1 as u8+0b1*bank.0 as u8;
        EndRecord{name:name, reg_msk:reg_msk}
    }
}

impl Record for EndRecord {
    const TYPE: u8 = 0x04;
    fn len(&self) -> u16 {
        self.name.len() as u16 + 6
    }

    fn hex (&self) ->Vec<u8>{
        let mut vec = Vec::new();
        vec.push(EndRecord::TYPE);
        vec.push((self.len()&0xFF) as u8);
        vec.push((self.len()/0xFF) as u8);
        vec.push(self.name.len() as u8);
        let name_vec = self.name.as_bytes();
        for c in name_vec {
            vec.push(*c);
        }
        vec.push(00);
        vec.push(00);
        vec.push(self.reg_msk);
        vec.push(00);
        let chk_some = checksome(&vec);
        vec.push(chk_some);
        vec
    }
}

#[derive(Debug)]
pub struct ContentRecord {
    offset: u16,
    data: Vec<u8>,
    seg_id: u8,
}

impl ContentRecord {
    pub fn new(offset: u16, data: Vec<u8>, seg_id: u8,) -> ContentRecord{
        ContentRecord{offset:offset, data:data, seg_id:seg_id}
    }
    pub fn data(&self) -> Vec<u8> {
        self.data.clone()
    }

    pub fn offset(&self) -> u16 {
        self.offset
    }
}

impl Record for ContentRecord {
    const TYPE: u8 = 0x06;
    fn len(&self) -> u16 {
        self.data.len() as u16 + 4
    }

    fn hex (&self) ->Vec<u8>{
        if self.data.len() ==0 {
            return vec![];
        }
        let mut vec = Vec::new();
        vec.push(ContentRecord::TYPE);
        vec.push((self.len()%0xFF) as u8);
        vec.push((self.len()/0xFF) as u8);
        vec.push(self.seg_id);
        vec.push((self.offset%0xFF) as u8);
        vec.push((self.offset/0xFF) as u8);
        for d in &self.data {
            vec.push(*d);
        }
        let chk_some = checksome(&vec);
        vec.push(chk_some);
        vec
    }
}

// pub struct SegmentRecord {
//     pub name: String,
//     seg_id: u8,
//     seg_info: u8,
//     rel_typ: u8,
//     base: u16,
//     size: u16,
// }
//
// impl SegmentRecord {
//
//     const ID: u16 = 622;
//     pub fn new(name: String) -> SegmentRecord{
//         SegmentRecord{name:name}
//     }
// }
//
// impl Record for SegmentRecord {
//     const TYPE: u8 = 0x02;
//     fn len(&self) -> u16 {
//         self.name.len() as u16 + 6
//     }
//
//     fn hex (&self) ->Vec<u8>{
//         let mut vec = Vec::new();
//         vec.push(SegmentRecord::TYPE);
//         vec.push((self.len()/0xFF) as u8);
//         vec.push((self.len()%0xFF) as u8);
//         let name_vec = self.name.as_bytes();
//         for c in name_vec {
//             vec.push(*c);
//         }
//         vec.push((HeaderRecord::ID/0xFF) as u8);
//         vec.push((HeaderRecord::ID%0xFF) as u8);
//         let chk_some = checksome(&vec);
//         vec.push(chk_some);
//         vec
//     }
// }
