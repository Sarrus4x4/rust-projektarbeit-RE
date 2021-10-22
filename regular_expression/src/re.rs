//Do i even need an Enum for the Types?
pub enum ReType {
    Eps,
    Phi,
    C, //{val: char}?
    Alt,
    Conc,
    Star
}


pub struct Eps {
}
pub struct Phi {
}
pub struct C {
    pub val: String
}
pub struct Alt {
    pub left: C,
    pub right: C
}
pub struct Conc {
    pub left: C,
    pub right: C
}
pub struct Star {
    pub obj: C
}


pub trait Pretty {
    fn pretty (&self) -> String;
}
pub trait ContainsEps {
    fn contains_eps (&self) -> bool;
}
pub trait IsPhi {
    fn is_phi (&self) -> bool;
}


pub fn run(){
    //Conc(char: 'a', Conc(char: 'b', Star(char'c'))
}


