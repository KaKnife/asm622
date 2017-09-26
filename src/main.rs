use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

mod section;
mod omf;
mod line;
mod instruction;
mod hex_table;
use hex_table::HexTable;

fn main() {
    let args: Vec<String> = env::args().collect();
    // let mut lines;
    let label_table;
    let mut errors = Vec::new(); //if there is an error in the compulation

    // Create a path to the desired file
    let path = Path::new(&args[1]);
    let display = path.display();
    let mut out_type = OutType::Bin;

    if args.len() > 2 {
        let obj_string = args[2].as_ref();
        match obj_string {
            "-o" => out_type = OutType::Obj,
            "-h" => out_type = OutType::Hex,
            _ => {},
        }
    }

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", display,
        why.description()),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut file_text = String::new();
    file.read_to_string(&mut file_text).unwrap();

    let lines = line::get_lines(file_text);

    let sections;
    match section::get_sections(lines){
        Ok(s) => sections = s,
        Err(e) => {
            for err in e {
                println!("{}",err);
            }
            return;
        }
    };



    // Generate a table of all known labels
    label_table = build_label_table(&sections);

    let mut records: Vec<omf::ContentRecord> = Vec::new();
    for mut sec in sections {
        match sec.fix_labels(&label_table) {
            Err(mut e) => errors.append(&mut e),
            _ => {},
        }
        if errors.is_empty() {
            let record;
            match sec.get_content_record(){
                Ok(r) => record = r,
                Err(mut e) => {
                    errors.append(&mut e);
                    continue;
                }
            }
            records.push(record);
        }
    }

    if errors.is_empty(){
        let name = String::from(path.file_stem().unwrap().to_str().unwrap());

        match out_type {
            OutType::Bin => output_bin(name, records),
            OutType::Obj => output_obj(name, records),
            OutType::Hex => output_hex(name, records),
        }
    }
    else {
        for e in errors{
            println!("{}", e);
        }
        println!("Failed to build {}", path.display());
    }

}

fn build_label_table(sections: &Vec<section::Section>) -> Vec<(String, u16)>{
    let mut table = Vec::new();
    for sec in sections {
        let mut  sub_table = sec.build_label_table();
        table.append(&mut sub_table);
    }
    //built in labels

    table.push(("P0".to_string(),0x80));
    table.push(("P1".to_string(),0x90));
    table.push(("P2".to_string(),0xA0));
    table.push(("P3".to_string(),0xB0));
    table.push(("PSW".to_string(), 0xD0));
    table.push(("ACC".to_string(), 0xE0));
    table.push(("B".to_string(), 0xF0));
    table.push(("SP".to_string(), 0x81));
    table.push(("DPL".to_string(), 0x82));
    table.push(("DPH".to_string(), 0x83));
    table.push(("PCON".to_string(), 0x87));
    table.push(("TCON".to_string(), 0x88));
    table.push(("TMOD".to_string(), 0x89));
    table.push(("TL0".to_string(), 0x8A));
    table.push(("TL1".to_string(), 0x8B));
    table.push(("TH0".to_string(), 0x8C));
    table.push(("TH1".to_string(), 0x8D));
    table.push(("IEN0".to_string(), 0xA8));
    table.push(("IP0".to_string(), 0xB8));
    table.push(("SCON".to_string(), 0x98));
    table.push(("SBUF".to_string(), 0x99));
    table.push(("AUXR1".to_string(), 0xA2));
    table.push(("SADDR".to_string(), 0xA9));
    table.push(("SADEN".to_string(), 0xB9));
    table.push(("TL2".to_string(), 0xCC));
    table.push(("TH2".to_string(), 0xCD));
    table.push(("BRGR0".to_string(), 0xBE));
    table.push(("BRGR1".to_string(), 0xBF));
    table.push(("BRGCON".to_string(),0xBD));
    table.push(("CCCRA".to_string(), 0xEA));
    table.push(("CCCRB".to_string(), 0xEB));
    table.push(("CCCRC".to_string(), 0xEC));
    table.push(("CCCRD".to_string(), 0xED));
    table.push(("CMP1".to_string(),0xAC));
    table.push(("CMP2".to_string(),0xAD));
    table.push(("DEECON".to_string(),0xF1));
    table.push(("DEEDAT".to_string(),0xF2));
    table.push(("DEEADR".to_string(),0xF3));
    table.push(("DIVM".to_string(),0x95));
    table.push(("I2ADR".to_string(), 0xDB));
    table.push(("I2CON".to_string(), 0xD8));
    table.push(("I2DAT".to_string(), 0xDA));
    table.push(("I2SCLH".to_string(),0xDD));
    table.push(("I2SCLL".to_string(),0xDC));
    table.push(("I2STAT".to_string(),0xD9));
    table.push(("ICRAH".to_string(), 0xAB));
    table.push(("ICRAL".to_string(), 0xAA));
    table.push(("ICRBH".to_string(), 0xAF));
    table.push(("ICRBL".to_string(), 0xAE));
    table.push(("IEN1".to_string(),0xE8));
    table.push(("IP1".to_string(), 0xF8));
    table.push(("IP1H".to_string(),0xF7));
    table.push(("KBCON".to_string(), 0x94));
    table.push(("KBMASK".to_string(),0x86));
    table.push(("KBPATN".to_string(),0x93));
    table.push(("OCRAH".to_string(), 0xEF));
    table.push(("OCRAL".to_string(), 0xEE));
    table.push(("OCRBH".to_string(), 0xFB));
    table.push(("OCRBL".to_string(), 0xFA));
    table.push(("OCRCH".to_string(), 0xFD));
    table.push(("OCRCL".to_string(), 0xFC));
    table.push(("OCRDH".to_string(), 0xFF));
    table.push(("OCRDL".to_string(), 0xFE));
    table.push(("P0M1".to_string(),0x84));
    table.push(("P0M2".to_string(),0x85));
    table.push(("P1M1".to_string(),0x91));
    table.push(("P1M2".to_string(),0x92));
    table.push(("P2M1".to_string(),0xA4));
    table.push(("P2M2".to_string(),0xA5));
    table.push(("P3M1".to_string(),0xB1));
    table.push(("P3M2".to_string(),0xB2));
    table.push(("PCONA".to_string(), 0xB5));
    table.push(("PT0AD".to_string(), 0xF6));
    table.push(("RSTSRC".to_string(),0xDF));
    table.push(("RTCCON".to_string(),0xD1));
    table.push(("RTCH".to_string(),0xD2));
    table.push(("RTCL".to_string(),0xD3));
    table.push(("SSTAT".to_string(), 0xBA));
    table.push(("SPCTL".to_string(), 0xE2));
    table.push(("SPSTAT".to_string(),0xE1));
    table.push(("SPDAT".to_string(), 0xE3));
    table.push(("TAMOD".to_string(), 0x8F));
    table.push(("TCR20".to_string(), 0xC8));
    table.push(("TCR21".to_string(), 0xF9));
    table.push(("TICR2".to_string(), 0xC9));
    table.push(("TIFR2".to_string(), 0xE9));
    table.push(("TISE2".to_string(), 0xDE));
    table.push(("TOR2H".to_string(), 0xCF));
    table.push(("TOR2L".to_string(), 0xCE));
    table.push(("TPCR2H".to_string(),0xCB));
    table.push(("TPCR2L".to_string(),0xCA));
    table.push(("TRIM".to_string(),0x96));
    table.push(("WDCON".to_string(), 0xA7));
    table.push(("WDL".to_string(), 0xC1));
    table.push(("WFEED1".to_string(),0xC2));
    table.push(("WFEED2".to_string(),0xC3));
    table.push(("IP0H".to_string(),0xB7));
    table
}

enum OutType {
    Bin,
    Obj,
    Hex,
}

fn output_bin(name:String, records:Vec<omf::ContentRecord>) {
    let mut hex_table = HexTable::new_empty();
    for record in records {
        hex_table.append_content(record);
    }


    let ext = ".bin";
    let file_name = &([&name, ext].join(""));
    let path = Path::new(file_name);
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}",
        display,
        why.description()),
        Ok(file) => file,
    };

    // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`

    match file.write_all(&hex_table.table) {
        Err(why) => {
            panic!("couldn't write to {}: {}", display,
            why.description())
        },
        Ok(_) => println!("successfully assembled to {}", display),
    }

}

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

fn output_hex(name:String, records:Vec<omf::ContentRecord>) {



    let ext = ".hex";
    let file_name = &([&name, ext].join(""));
    let path = Path::new(file_name);
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}",
        display,
        why.description()),
        Ok(file) => file,
    };

    // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
    for record in records {
        if record.data().len()>0{
            for i in 0 .. record.data().len()/0x10 +1{
                let mut line_out_hex = Vec::new();
                if i < record.data().len()/0x10 {
                    line_out_hex.push(0x10);
                }
                else {
                    line_out_hex.push((record.data().len()%0x10) as u8);
                }
                line_out_hex.push(((record.offset()+i as u16*0x10)/0x100 )as u8);
                line_out_hex.push(((record.offset()+i as u16*0x10)%0x100) as u8);
                line_out_hex.push(00);
                let mut temp_vec = Vec::new();
                let mut end = (i+1)*0x10;
                if end >= record.data().len() {
                    end = record.data().len();
                }
                temp_vec.extend_from_slice(&record.data()[0x10*i .. end]);
                if temp_vec.len() == 0 {
                    continue;
                }
                line_out_hex.append(&mut temp_vec);
                let chk_some = checksome(&line_out_hex);
                line_out_hex.push(chk_some);


                let mut line_out = String::from(":");
                for h in line_out_hex{
                    line_out+=&format!("{:02x}", h);

                }
                write!(file, "{}\n", line_out.to_uppercase()).unwrap();


            }

        }

    }
    write!(file, ":00000001FF\n").unwrap();

    println!("successfully assembled to {}", display);

}

fn output_obj(name: String, records:Vec<omf::ContentRecord>) {
    let mut hex_table = HexTable::new_empty();
    let header = omf::HeaderRecord::new(name.clone().to_uppercase());
    let end = omf::EndRecord::new(name.clone().to_uppercase(), (true,false,false,false));
    hex_table.append_record(header);
    for record in records {
        hex_table.append_record(record);
    }
    hex_table.append_record(end);


    let ext = ".obj";
    let file_name = &([&name, ext].join(""));
    let path = Path::new(file_name);
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}",
        display,
        why.description()),
        Ok(file) => file,
    };

    // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`

    match file.write_all(&hex_table.table) {
        Err(why) => {
            panic!("couldn't write to {}: {}", display,
            why.description())
        },
        Ok(_) => println!("successfully assembled to {}", display),
    }
}
