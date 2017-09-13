use std::fmt::Display;

pub struct Line {
    pub label: Option<String>,
    pub mnu: Option<String>,
    pub op1: Option<String>,
    pub op2: Option<String>,
    pub num: u8,
}

impl Line {
    pub fn new(num: u8, label: Option<String>, mnu: Option<String>, op1: Option<String>, op2: Option<String>, ) -> Line {
        Line{
            label:label,
            mnu:mnu,
            op1:op1,
            op2:op2,
            num:num
        }
    }
}

impl Display for Line {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let mut out :String = "".to_string();
        match self.label{
            Some(ref label) => out += &format!("{}-label\t\t: {}\n",self.num,label),
            _ => {},
        };
        match self.mnu{
            Some(ref mnu) => out += &format!("{}-Mnumonic\t: {}\n",self.num,mnu),
            _ => {},
        }
        match self.op1{
            Some(ref op) => out += &format!("{}-Operand 1\t: {}\n",self.num,op),
            _ => {},
        };
        match self.op2{
            Some(ref op) => out += &format!("{}-Operand 1\t: {}\n",self.num,op),
            _ => {},
        };
        write!(f, "{}", out)
    }
}
