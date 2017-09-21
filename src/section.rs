use omf::ContentRecord;
use instruction::Instruction;
use line::Line;
use hex_table::HexTable;

pub fn get_sections(lines: Vec<Line>) -> (Vec<Section>, bool) {
    let mut error = false;
    let mut offset = 0;
    use section::Section;
    let mut sections = Vec::new();
    let mut curr_sec = Section::new(0);
    //get the instructions from each line
    for line in lines {
        // println!("{}", line );
        let ins =  match Instruction::from_line(line,offset){
            Ok(i) => i,
            Err(e) => {
                println!("Error: {}", e);
                error = true;
                continue;
            }
        };
        offset =(offset as i32 +ins.len() as i32)as u16;
        // print!("{}", ins);
        match ins.is_new_section() {
            Some(d) => {
                sections.push(curr_sec);
                curr_sec = Section::new(d);
                offset = 0;
            },
            None =>{},
        }
        curr_sec.push(ins);
    }
    sections.push(curr_sec);
    (sections, error)
}

#[derive(Debug)]
pub struct Section {
    offset: u16,
    instructions: Vec<Instruction>,
}

impl Section {
    pub fn new(offset: u16) ->Section{
        Section{offset:offset, instructions:Vec::new()}
    }
    pub fn push(&mut self, instruction: Instruction) {
        self.instructions.push(instruction);
    }
    pub fn build_label_table(&self) -> Vec<(String, u16)> {
        let mut table: Vec<(String, u16)> = Vec::new();
        let mut label_instructions = self.instructions.clone();
        label_instructions.retain(|x| x.label.is_some());
        for label in label_instructions {
            let offset = label.offset()+self.offset;
            table.push((label.label.unwrap().to_lowercase(),offset));
        }
        table
    }
    // pub fn instructions(&self) -> Vec<Instruction>{
    //     self.instructions.clone()
    // }
    pub fn fix_labels(&mut self, label_table: &Vec<(String, u16)>) -> bool{
        let mut error = false;
        for mut instruction in &mut self.instructions {
            match instruction.fix_label(label_table) {
                Ok(()) =>{},
                Err(e) => {
                    println!("Error: {}", e);
                    error = true;
                    continue;
                }
            };
        }
        error
    }
    pub fn get_content_record(&self) -> ContentRecord {
        let mut error = false;
        let mut hex_table = HexTable::new(&(self.instructions));

        // Update lables and add instructions to hex file;
        for instruction in &self.instructions {
            if !error {
                let hex = match instruction.to_hex(){
                    Ok(h) => h,
                    Err(e) => {
                        println!("Error: {}", e);
                        error = true;
                        continue;
                    }
                };
                hex_table.update(instruction.offset(), &hex);
            }
        }
        ContentRecord::new(self.offset,hex_table.table,0)
    }
}
