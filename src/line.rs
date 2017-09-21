// use std::fmt::Display;

pub struct Line {
    pub label: Option<String>,
    pub mnu: Option<String>,
    pub ops: Vec<String>,
    pub num: u8,
}

impl Line {
    pub fn new(num: u8, label: Option<String>, mnu: Option<String>, ops: Vec<String>, ) -> Line {
        Line{
            label:label,
            mnu:mnu,
            ops:ops,
            num:num
        }
    }
}

// impl Display for Line {
//     fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
//         let mut out :String = "".to_string();
//         match self.label{
//             Some(ref label) => out += &format!("{}-label\t\t: {}\n",self.num,label),
//             _ => {},
//         };
//         match self.mnu{
//             Some(ref mnu) => out += &format!("{}-Mnumonic\t: {}\n",self.num,mnu),
//             _ => {},
//         }
//         match self.op1{
//             Some(ref op) => out += &format!("{}-Operand 1\t: {}\n",self.num,op),
//             _ => {},
//         };
//         match self.op2{
//             Some(ref op) => out += &format!("{}-Operand 1\t: {}\n",self.num,op),
//             _ => {},
//         };
//         write!(f, "{}", out)
//     }
// }

pub fn get_lines(file_text: String) -> Vec<Line>{
    let mut lines= Vec::new();
    let read_lines = file_text.lines();
    let mut i = 1u8;
    //get each line of the program
    for mut line in read_lines {
        let mut mnu = None;
        let mut ops = Vec::new();
        let mut label= None;
        line=line.trim();
        if line.contains(";"){
            let tmp: Vec<&str> = line.split(';').collect();
            line = tmp[0];
        }

        if line.contains(":"){
            let tmp: Vec<&str> = line.split(':').collect();
            label = Some(tmp[0].to_string());
            line = tmp[1];
        }

        let mut cmd: Vec<&str> = line.split(|c| c == ' ' || c == ',' || c == ':'|| c == '\t').collect();
        cmd.retain(|x| x.to_string() != "");

        if cmd.len() > 0 {
            mnu = Some(cmd[0].to_string());
            if cmd.len() > 1 {
                for i in 2 .. cmd.len(){
                    ops.push(cmd[i].to_string())
                }
            }
        }
        let l = Line::new(i, label, mnu, ops);
        lines.push(l);
        //println!("{}",l);
        i+=1;
    }

    lines
}
