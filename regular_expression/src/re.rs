#[derive(Debug)]

// ######### Structs #########
pub struct Eps { //can it stay empty?
    //pub val: String
}
pub struct Phi { //can it stay empty?
    //pub val: String
}
pub struct C {
    pub val: String
}
pub struct Alt <T,J> {
    pub l: T,
    pub r: J
}
pub struct Conc <T,J>{
    pub l: T,
    pub r: J
}
pub struct Star <T> {
    pub obj: T
}


// ######### Traits #########
pub trait Pretty { //for printing out the result
    fn pretty (&self) -> String;
}
pub trait ContainsEps { //check if the element contains Eps
    fn contains_eps (&self) -> bool;
}
pub trait IsPhi { //check if the whole RE is Phi (not shure if i even need this)
    fn is_phi (&self) -> bool;
}


// ######### Implementation Pretty #########
impl Pretty for C {
    fn pretty(&self)->String{
        self.val.to_string()
    }
}
impl <T:Pretty, J:Pretty> Pretty for Alt <T,J> {
    fn pretty(&self)->String{
        format!("({}|{})",self.l.pretty(),self.r.pretty()) //Add logic to check for eps/phi and format accordingly 
    }
}
impl <T:Pretty, J:Pretty> Pretty for Conc <T,J>{
    fn pretty(&self)->String{
        format!("({}{})",self.l.pretty(),self.r.pretty()) //Add logic to check for eps/phi and format accordingly 
    }
}
impl <T:Pretty> Pretty for Star <T>{
    fn pretty(&self)->String{
        format!("({}*)",self.obj.pretty()) //Add logic to check for eps/phi and format accordingly 
    }
}
impl Pretty for Eps{ //do i even neet a pretty method for Eps? Yes i think so! :D
    fn pretty(&self)->String{
        format!("")
    }
}
impl Pretty for Phi{ //do i even neet a pretty method for Phi? Yes i think so! :D
    fn pretty(&self)->String{
        format!("")
    }
}

// ######### Implementation ContainsEps #########
impl ContainsEps for C  { //do i only need to check for eps in C?
    fn contains_eps(&self)->bool{
        let mut contains:bool = false;
        if self.pretty() == "" {
            contains = true;
        }
        contains
    }
}

// ######### Implementation IsPhi #########
//do i even need those? It is the exact same implementation as 'ContainsEps' for every single one of them!
impl IsPhi for C {
    fn is_phi(&self)->bool{
        let mut contains:bool = false;
        if self.pretty() == "" {
            contains = true;
        }
        contains
    }
}
impl<T:Pretty, U:Pretty> IsPhi for Alt<T,U> { 
    fn is_phi(&self)->bool{
        let mut contains:bool = false;
        if self.pretty() == "" {
            contains = true;
        }
        contains
    }
}
impl <T:Pretty, U:Pretty> IsPhi for Conc<T,U> { 
    fn is_phi(&self)->bool{
        let mut contains:bool = false;
        if self.pretty() == "" {
            contains = true;
        }
        contains
    }
}
impl <T:Pretty> IsPhi for Star<T> {
    fn is_phi(&self)->bool{
        let mut contains:bool = false;
        if self.pretty() == "" {
            contains = true;
        }
        contains
    }
}


// ######### main method #########
pub fn run(){

    let re1 = Alt{l: C{val: "a".to_string()}, r: C{val: "b".to_string()}};
    println!("{}",re1.pretty());

    let re2 = Conc{l: C{val: "a".to_string()}, r: C{val: "b".to_string()}};
    println!("{}",re2.pretty());

    let re3 = Star{obj: C{val: "a".to_string()}};
    println!("{}",re3.pretty());

    let re4 = C{val: "a".to_string()};
    println!("{}",re4.contains_eps());

    let re5 = C{val: "".to_string()};
    println!("{}",re5.contains_eps());

    //This needs to be displayed correctly ->  (a*) b
    let re6 = Conc{l: Eps{},r: Conc{l: Star{obj: Star{obj: C{val: "a".to_string()} }}, r: Alt{l: Phi{},r: C{val: "b".to_string()}}}}; 
    println!("{}",re6.pretty());
}