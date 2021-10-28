// ######### Structs #########
pub struct Eps {
}
pub struct Phi {
}
pub struct C {
    pub val: String
}
pub struct Alt { //i need to use generics as type, not C
    pub left: C,
    pub right: C
}
pub struct Conc { //i need to use generics as type, not C 
    pub left: C,
    pub right: C
}
pub struct Star { //i need to use generics as type, not C 
    pub obj: C
}


// ######### Traits #########
pub trait Pretty { //for printing out the result
    fn pretty (&self) -> String;
}
// pub trait Compute { //for evaluating the result 
//     fn compute(&self)->String;
// }
pub trait ContainsEps { //check of it contains Eps
    fn contains_eps (&self) -> bool;
}
pub trait IsPhi { //check of it is Phi
    fn is_phi (&self) -> bool;
}


// ######### Implementations #########
impl Pretty for C { //pretty method only relevant for C
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
impl Pretty for Eps{ //do i even neet a pretty method for Eps?
    fn pretty(&self)->String{
        format!("")
    }
}
impl Pretty for Phi{ //do i even neet a pretty method for Eps?
    fn pretty(&self)->String{
        format!("")
    }
}




pub fn run(){

    let re1 = Alt{left: C{val: "a".to_string()}, right: C{val: "b".to_string()}};
    println!("{}",re1.pretty());

    let re2 = Conc{left: C{val: "a".to_string()}, right: C{val: "b".to_string()}};
    println!("{}",re2.pretty());

    let re3 = Star{obj: C{val: "a".to_string()}};
    println!("{}",re3.pretty());


    //Conc(C: 'a', Conc(C: 'b', Star(C: 'd'))
    //this is how i create a RE?
}


