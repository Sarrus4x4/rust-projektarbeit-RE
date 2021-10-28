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
pub struct Alt { //TODO: i need to use generics as type instead of C so i can do nested function calls
    pub left: C,
    pub right: C
}
pub struct Conc { //TODO: i need to use generics as type instead of C so i can do nested function calls
    pub left: C,
    pub right: C
}
pub struct Star { //TODO: i need to use generics as type instead of C so i can do nested function calls
    pub obj: C
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
impl Pretty for Alt {
    fn pretty(&self)->String{
        format!("({}|{})",self.left.pretty(),self.right.pretty())
    }
}
impl Pretty for Conc{
    fn pretty(&self)->String{
        format!("({}{})",self.left.pretty(),self.right.pretty())
    }
}
impl Pretty for Star{
    fn pretty(&self)->String{
        format!("({}*)",self.obj.pretty())
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
impl ContainsEps for C { //do i only need to check for eps in C?
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
impl IsPhi for Alt { 
    fn is_phi(&self)->bool{
        let mut contains:bool = false;
        if self.pretty() == "" {
            contains = true;
        }
        contains
    }
}
impl IsPhi for Conc { 
    fn is_phi(&self)->bool{
        let mut contains:bool = false;
        if self.pretty() == "" {
            contains = true;
        }
        contains
    }
}
impl IsPhi for Star {
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

    let re1 = Alt{left: C{val: "a".to_string()}, right: C{val: "b".to_string()}};
    println!("{}",re1.pretty());

    let re2 = Conc{left: C{val: "a".to_string()}, right: C{val: "b".to_string()}};
    println!("{}",re2.pretty());

    let re3 = Star{obj: C{val: "a".to_string()}};
    println!("{}",re3.pretty());

    let re4 = C{val: "a".to_string()};
    println!("{}",re4.contains_eps());

    let re5 = C{val: "".to_string()};
    println!("{}",re5.contains_eps());

    //No nested function calls possible yet because of parameter of type 'C'
    //Conc{Conc{left: C{val: "a".to_string()}, right: C{val: "b".to_string()}}, Star{obj: C{val: "a".to_string()}}}; 
}