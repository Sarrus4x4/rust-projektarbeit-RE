//#[derive(Debug)]

// Enum with all types of RE
pub enum ReType {
    Eps,
    Phi,
    C, //{val: char} could be used to save the the value of the char easily 
    Alt,
    Conc,
    Star
}
//Display for Enum ReType so it can be pritnted 
use std::fmt;
impl fmt::Display for ReType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match *self {
        ReType::Eps => write!(f, "Eps"),
        ReType::Phi => write!(f, "Phi"),
        ReType::C => write!(f, "C"),
        ReType::Alt => write!(f, "Alt"),
        ReType::Conc => write!(f, "Conc"),
        ReType::Star => write!(f, "Star")
       }
    }
}

// Struct of a regular expression with a left and right value
// left and right should be of type RE but this creates an endless loop 
pub struct RE <T,J> {
    pub left: T,
    pub right: J,
    pub re_type: ReType
}
// Generation of basic methods to work with regular expressions
pub trait BasicMethods {
    fn of_type (&self) -> ReType;
    fn pretty (&self) -> String;
    fn contains_eps (&self) -> bool;
}
// Implementation of the basic methods 
impl <T, J> BasicMethods for RE<T, J> {
    fn of_type (&self) -> ReType{
        //only did this because "self.re_type" would not work!
        if matches!(self.re_type, ReType::Eps){
            ReType::Eps
        } else if matches!(self.re_type, ReType::Phi){
            ReType::Phi
        } else if matches!(self.re_type, ReType::C ){
            ReType::C
        } else if matches!(self.re_type, ReType::Alt){
            ReType::Alt
        } else if matches!(self.re_type, ReType::Conc){
            ReType::Conc
        } else {
            ReType::Star
        }
    }
    fn pretty (&self) -> String{
        format!("Format the regular expression!")
    }
    // might need an actual check for an empty String
    fn contains_eps (&self) -> bool{
        let mut contains_eps = false;
        if matches!(self.of_type(), ReType::Eps) { // || matches!(self.of_type(), ReType::Phi) could be added for a is_Phi() function 
            contains_eps = true;
        }
        contains_eps
    }
}

// Should i use structs for all types of RE?

// pub struct Eps {
//     //empty?
// }
// pub struct Phi {
//     //empty?
// }
// pub struct C {
//     val: char
// }
// pub struct Alt {
//     pub left: RE ,
//     pub right: RE,
// }
// pub struct Conc {
//     pub left: RE ,
//     pub right: RE,
// }
// pub struct Star {
//     pub val_to_star: RE
// }
