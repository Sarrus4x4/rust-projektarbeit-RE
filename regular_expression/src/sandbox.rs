pub enum RE_Type {
    eps,
    phi,
    c,
    alt,
    conc,
    star
}

pub struct RE <T,J> {
    pub left: T ,
    pub right: J,
    pub re_type: RE_Type
}

pub trait basic_methods {
    fn of_type (&self) -> RE_Type;
    fn pretty (&self) -> String;
    fn contains_eps (&self) -> bool;
}

impl <T, J> basic_methods for RE<T, J> {
    fn of_type (&self) -> RE_Type{
        //only did this because "self.re_type" would not work!
        if matches!(self.re_type, RE_Type::eps){
            RE_Type::eps
        } else if matches!(self.re_type, RE_Type::phi){
            RE_Type::phi
        } else if matches!(self.re_type, RE_Type::c){
            RE_Type::c
        }else if matches!(self.re_type, RE_Type::c){
            RE_Type::c
        }else if matches!(self.re_type, RE_Type::alt){
            RE_Type::alt
        }else if matches!(self.re_type, RE_Type::conc){
            RE_Type::conc
        } else {
            RE_Type::star
        }
    }
    fn pretty (&self) -> String{
        format!("Here the content of RE needs to be formated (and printed)!")
    }
    fn contains_eps (&self) -> bool{
        let mut contains_eps = false;
        if matches!(self.of_type(), RE_Type::phi) || matches!(self.of_type(), RE_Type::eps) { 
            contains_eps = true;
        }
        contains_eps
    }
}

pub fn run(){ 
       let re1 = RE{
           left: 's',
           right: "os",
           re_type: RE_Type::conc
       };
    }