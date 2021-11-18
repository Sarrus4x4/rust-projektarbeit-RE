// ######### Structs #########
// This is where the different struct are defined 
pub struct Eps {
    //The Eps object has no parameters 
}
pub struct Phi {
    //The Phi object has no parameters
}
pub struct C {
    pub val: String,
    //The C struct has a value parameter of type String. Here the the smalest element of a RE is saved
}
pub struct Alt<T, J> {
    pub l: T,
    pub r: J,
    //The Alt struct has tho parameters of a variable Type. They are the left and right part of the alternative clause (l|r)
}
pub struct Conc<T, J> {
    pub l: T,
    pub r: J,
    //The Conc struct has tho parameters of a variable Type. They are the left and right part of the concatination clause (lr)
}
pub struct Star<T> {
    pub obj: T,
    //The Star struct has a single parameter of variable Type. It is the argument which is supposed to get a "*" added to it like so: obj*
}

// ######### Traits #########
//Here all the necessary traits are defined to format the RE's and to determine whether they contain Eps or are Phi 
pub trait RE {
    fn pretty(&self) -> String;
    fn contains_eps(&self) -> bool;
    fn is_phi(&self) -> bool;
}

// ######### Implementation #########
//Here the RE trait is implemented for every struct that was defined at the start
impl RE for C {

    fn pretty(&self) -> String { //The pretty function for the C struct is simply returning the String value
        self.val.to_string()
    }

    fn contains_eps(&self) -> bool { //The contains_eps function is simply checking if the current argument is equal to the empty String
        let mut contains: bool = false;
        if self.val == "" {
            contains = true;
        }
        contains
    }
    
    fn is_phi(&self) -> bool {
        let mut contains: bool = false; //The is_phi function is simply checking if the current argument is equal to phi
        if self.pretty() == "phi" {
            contains = true;
        }
        contains
    }

}

impl<T: RE, J: RE> RE for Alt<T, J> { 

    fn pretty(&self) -> String { //The pretty function for the Alt struct is checking for different special cases and is formatting the result accordingly
        if self.l.contains_eps() || self.l.is_phi() {
            if self.r.contains_eps() || self.r.is_phi() {
                format!("") //If both the left and the right argument of the alternative Struct are either eps or phi than the whole term is empty
            } else {
                format!("{}", self.r.pretty()) //If just the left argument of the alternative Struct is either eps or phi than just the right argument is formatted (r1 + r2 ==> r2 if L(r1)={})
            }
        } else if self.r.contains_eps() || self.r.is_phi() {
            format!("{}", self.l.pretty()) //If just the right argument of the alternative Struct is either eps or phi than just the left argument is formatted (r1 + r2 ==> r1 if L(r2)={})
        } else {
            if self.l.pretty() == self.r.pretty(){
                format!("{}", self.l.pretty()) //If the left and right argument of the alternative Stuct are the same than just one of them is formatted (r + r ==> r)
            }else{
                format!("({}|{})", self.l.pretty(), self.r.pretty()) //If both arguments of the alternative Stuct are of regular type than they are formated like so: (l|r) 
            }
        }
    }

    fn contains_eps(&self) -> bool { //The contains_eps function is simply checking if the current argument is equal to the empty String
        let mut contains: bool = false;
        if self.pretty() == "" {
            contains = true;
        }
        contains
    }

    fn is_phi(&self) -> bool { //The is_phi function is simply checking if the current argument is equal to phi
        let mut contains: bool = false;
        if self.pretty() == "phi" {
            contains = true;
        }
        contains
    }

}

impl<T: RE, J: RE> RE for Conc<T, J> {

    fn pretty(&self) -> String { //The pretty function for the Conc struct is checking for different special cases and is formatting the result accordingly
        if self.l.is_phi() || self.r.is_phi() {
            format!("") //If the left or the right argument of the concatination Struct are phi than the whole term is phi
        } else {
            if self.l.contains_eps() {
                if self.r.contains_eps() {
                    format!("") //If both the left and the right argument of the concatination Struct eps than the whole term is empty (phi if L(r1)={} or L(r2)={})
                } else {
                    format!("{}", self.r.pretty()) //If just the left argument of the concatination Struct is eps than only the right argument is formatted (eps r ==> r)
                }
            } else if self.r.contains_eps() {
                format!("{}", self.l.pretty()) //If just the right argument of the concatination Struct is eps than only the left argument is formatted (r eps ==> r)
            } else {
                format!("({}{})", self.l.pretty(), self.r.pretty()) //If both arguments of the concatination Stuct are of regular type than they are formated like so: (lr) 
            }
        }
    }

    fn contains_eps(&self) -> bool { //The contains_eps function is simply checking if the current argument is equal to the empty String
        let mut contains: bool = false;
        if self.pretty() == "" {
            contains = true;
        }
        contains
    }

    fn is_phi(&self) -> bool { //The is_phi function is simply checking if the current argument is equal to phi
        let mut contains: bool = false;
        if self.pretty() == "phi" {
            contains = true;
        }
        contains
    }

}

impl<T: RE> RE for Star<T> {

    fn pretty(&self) -> String { //The pretty function for the Star struct is checking for different special cases and is formatting the result accordingly
        if self.obj.contains_eps() || self.obj.is_phi() {
            format!("") //If the obj argument of the star Struct is eps or phi than the whole term is empty
        } else {
            let s = self.obj.pretty();
            if s.ends_with("*)"){
                format!("{}", self.obj.pretty()) //If the star Struct is called twice in a row (nested call) than it is interpreted as just one call ((r*)* ==> r*)
            } else{
                format!("({}*)", self.obj.pretty()) //If the argument of the star Struct is of regular type than it is formated like so: (obj*)
            }
            
        }
    }

    fn contains_eps(&self) -> bool { //The contains_eps function is simply checking if the current argument is equal to the empty String
        let mut contains: bool = false;
        if self.pretty() == "" { 
            contains = true;
        }
        contains
    }

    fn is_phi(&self) -> bool { //The is_phi function is simply checking if the current argument is equal to phi
        let mut contains: bool = false;
        if self.pretty() == "phi" {
            contains = true;
        }
        contains
    }

}

impl RE for Eps {
    fn pretty(&self) -> String { //The pretty function for the Eps struct is simply formatting a empty String
        format!("")
    }
    fn contains_eps(&self) -> bool { //The contains_eps function is simply checking if the current argument is equal to the empty String
        let mut contains: bool = false;
        if self.pretty() == "" { 
            contains = true;
        }
        contains
    }
    fn is_phi(&self) -> bool { //The is_phi function is simply checking if the current argument is equal to phi
        let mut contains: bool = false;
        if self.pretty() == "phi" {
            contains = true;
        }
        contains
    }
}

impl RE for Phi {
    fn pretty(&self) -> String { //The pretty function for the Phi struct is simply formatting the "phi" String 
        format!("phi")
    }
    fn contains_eps(&self) -> bool { //The contains_eps function is simply checking if the current argument is equal to the empty String
        let mut contains: bool = false;
        if self.pretty() == "" { 
            contains = true;
        }
        contains
    }
    fn is_phi(&self) -> bool { //The is_phi function is simply checking if the current argument is equal to phi
        let mut contains: bool = false;
        if self.pretty() == "phi" {
            contains = true;
        }
        contains
    }
}

// ######### main method #########
//Here i am testing different RE's for their correct formatting and simplification
pub fn run() {
    let re1 = Alt {l: C {val: "a".to_string() }, r: C { val: "b".to_string() }};
    println!("{}", re1.pretty());

    let re2 = Conc { l: C {  val: "a".to_string()},  r: C {val: "b".to_string()}};
    println!("{}", re2.pretty());

    let re3 = Star { obj: Star { obj: C {  val: "a".to_string()}}};
    println!("{}", re3.pretty());

    let re4 = C {  val: "a".to_string()};
    println!("{}", re4.contains_eps());

    let re5 = C {  val: "".to_string()};
    println!("{}", re5.contains_eps());

    let re6 = C {val: "a".to_string()};
    println!("{}", re6.is_phi());

    let re7 = C { val: "phi".to_string()};
    println!("{}", re7.is_phi());

    let re8 = Alt {l: Phi {}, r: C { val: "b".to_string()}};
    println!("{}", re8.pretty());

    //This needs to be displayed correctly: eps ((a*)* (phi | b)) -> (a*) b
    let re9 = Conc {l: Eps {},r: Conc {l: Star { obj: Star { obj: C { val: "a".to_string() } }}, r: Alt {l: Phi {}, r: C {val: "b".to_string() }}}};
    println!("{}", re9.pretty());

    let re10 = Star {obj: Conc{l: C { val: "a".to_string() } ,r: Star { obj: Star { obj: C {  val: "a".to_string()}}}}};
    println!("{}", re10.pretty());

    let re11 = Phi{};
    println!("{}", re11.pretty());

    let re12 = Eps{};
    println!("{}", re12.pretty());
}