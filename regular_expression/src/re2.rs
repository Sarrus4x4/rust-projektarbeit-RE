// ######### Structs #########
pub struct Eps {
    //can it stay empty?
//pub val: String
}
pub struct Phi {
    //can it stay empty?
//pub val: String
}
pub struct C {
    pub val: String,
}
pub struct Alt<T, J> {
    pub l: T,
    pub r: J,
}
pub struct Conc<T, J> {
    pub l: T,
    pub r: J,
}
pub struct Star<T> {
    pub obj: T,
}

// ######### Traits #########

pub trait RE {
    fn pretty(&self) -> String;
    fn contains_eps(&self) -> bool;
    fn is_phi(&self) -> bool;
}

// ######### Implementation #########

impl RE for C {
    fn pretty(&self) -> String {
        self.val.to_string()
    }
    fn contains_eps(&self) -> bool {
        let mut contains: bool = false;
        if self.val == "" {
            contains = true;
        }
        contains
    }
    fn is_phi(&self) -> bool {
        let mut contains: bool = false;
        if self.pretty() == "phi" {
            contains = true;
        }
        contains
    }
}

impl<T: RE, J: RE> RE for Alt<T, J> {
    fn pretty(&self) -> String {
        if self.l.contains_eps() || self.l.is_phi() {
            if self.r.contains_eps() || self.r.is_phi() {
                format!("")
            } else {
                format!("{}", self.r.pretty())
            }
        } else if self.r.contains_eps() || self.r.is_phi() {
            format!("{}", self.l.pretty())
        } else {
            if self.l.pretty() == self.r.pretty(){
                format!("{}", self.l.pretty()) 
            }else{
                format!("({}|{})", self.l.pretty(), self.r.pretty())
            }
        }
    }
    fn contains_eps(&self) -> bool {
        let mut contains: bool = false;
        if self.pretty() == "" { //.val
            contains = true;
        }
        contains
    }
    fn is_phi(&self) -> bool {
        let mut contains: bool = false;
        if self.pretty() == "phi" {
            contains = true;
        }
        contains
    }
}

impl<T: RE, J: RE> RE for Conc<T, J> {
    fn pretty(&self) -> String {
        if self.l.is_phi() || self.r.is_phi() {
            //if left or right argument of concat is Phi -> the whole term is Phi
            format!("")
        } else {
            if self.l.contains_eps() {
                if self.r.contains_eps() {
                    format!("")
                } else {
                    format!("{}", self.r.pretty())
                }
            } else if self.r.contains_eps() {
                format!("{}", self.l.pretty())
            } else {
                format!("({}{})", self.l.pretty(), self.r.pretty()) //logic to check for phi and format accordingly
            }
        }
    }
    fn contains_eps(&self) -> bool {
        let mut contains: bool = false;
        if self.pretty() == "" { //.val
            contains = true;
        }
        contains
    }
    fn is_phi(&self) -> bool {
        let mut contains: bool = false;
        if self.pretty() == "phi" {
            contains = true;
        }
        contains
    }
}

impl<T: RE> RE for Star<T> {
    fn pretty(&self) -> String {
        if self.obj.contains_eps() || self.obj.is_phi() {
            //logic to check for eps/phi and format accordingly
            format!("")
        } else {
            let s = self.obj.pretty();
            let last: Vec<char> = s.chars().rev().take(1).collect(); // <--- only returns the actual chars like 'a' but not '*'  | I can not use .length during runtime!
            println!("{:?}", last);
          

            format!("({}*)", self.obj.pretty())  // <------------------------------------------- TODO: Add logic for (r*)* ==> r*
            
        }
    }
    fn contains_eps(&self) -> bool {
        let mut contains: bool = false;
        if self.pretty() == "" { //.val
            contains = true;
        }
        contains
    }
    fn is_phi(&self) -> bool {
        let mut contains: bool = false;
        if self.pretty() == "phi" {
            contains = true;
        }
        contains
    }
}

impl RE for Eps {
    fn pretty(&self) -> String {
        format!("")
    }
    fn contains_eps(&self) -> bool {
        let mut contains: bool = false;
        if self.pretty() == "" { //val
            contains = true;
        }
        contains
    }
    fn is_phi(&self) -> bool {
        let mut contains: bool = false;
        if self.pretty() == "phi" {
            contains = true;
        }
        contains
    }
}

impl RE for Phi {
    fn pretty(&self) -> String {
        format!("phi")
    }
    fn contains_eps(&self) -> bool {
        let mut contains: bool = false;
        if self.pretty() == "" { //.val
            contains = true;
        }
        contains
    }
    fn is_phi(&self) -> bool {
        let mut contains: bool = false;
        if self.pretty() == "phi" {
            contains = true;
        }
        contains
    }
}

// ######### main method #########
pub fn run() {
    // let re1 = Alt {l: C {val: "a".to_string() }, r: C { val: "b".to_string() }};
    // println!("{}", re1.pretty());

    // let re2 = Conc { l: C {  val: "a".to_string()},  r: C {val: "b".to_string()}};
    // println!("{}", re2.pretty());

    let re3 = Star { obj: C {  val: "a".to_string()}};
    println!("{}", re3.pretty());

    // let re4 = C {  val: "a".to_string()};
    // println!("{}", re4.contains_eps());

    // let re5 = C {  val: "".to_string()};
    // println!("{}", re5.contains_eps());

    // let re6 = C {val: "a".to_string()};
    // println!("{}", re6.is_phi());

    // let re7 = C { val: "phi".to_string()};
    // println!("{}", re7.is_phi());

    // let re8 = Alt {l: Phi {}, r: C { val: "b".to_string()}};
    // println!("{}", re8.pretty());

    // //This needs to be displayed correctly: eps ((a*)* (phi | b)) -> (a*) b
    // let re9 = Conc {l: Eps {},r: Conc {l: Star { obj: Star { obj: C { val: "a".to_string() } }}, r: Alt {l: Phi {}, r: C {val: "b".to_string() }}}};
    // println!("{}", re9.pretty());
}