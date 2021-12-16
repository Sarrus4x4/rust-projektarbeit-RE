enum Exp {
    
    Eps{
        
    },
    Phi{

    },
    Char {
        val: char,
    },
    Alt {
        left: Box<Exp>,
        right: Box<Exp>,
    },
    Conc {
        left: Box<Exp>,
        right: Box<Exp>, 
    },
    Star{
        obj: Box<Exp>,
    }

}

// Show for expressions.
fn pretty(x : &Exp) -> String {
    match x {
        Exp::Eps{} => {
            return "".to_string();
        }
        Exp::Phi{} => {
            return "phi".to_string();
        }
        Exp::Char{val} => {
            return val.to_string();
            
        } 
        Exp::Alt{left,right} => {
            let s = "(".to_string() + &pretty(&left) + &"|".to_string()+ &pretty(&right) + &")".to_string();
            return s;
        }
        Exp::Conc{left,right} => {
            let s = "(".to_string() + &pretty(&left) + &pretty(&right) + &")".to_string();
            return s;
        }
        Exp::Star{obj} => {
            let s = "(".to_string() + &pretty(&obj) + &")*".to_string();
            return s;
        }
    }
}



// Show for expressions.
fn simplify(x : &Exp) -> Box<Exp> { //-> Box<Exp> 
    match x {

        Exp::Eps{} => {
            //simply return the given parameter x (currently returning new Exp because i dont know how to return x)
            Box::new(Exp::Eps{})
        }

        Exp::Phi{} => {
            //simply return the given parameter x (currently returning new Exp because i dont know how to return x)
            Box::new(Exp::Phi{})
        }

        Exp::Char{val} => {
            //simply return the given parameter x (currently returning new Exp because i dont know how to return x)
            let temp_val: char = *val;
            Box::new(Exp::Char{val: temp_val})
        } 

        Exp::Alt{left,right} => {
            //if left is Phi and right is not -> return right
            if (is_phi(left)) && (!is_phi(right)){
                right
            }

            //if right is Phi and left is not -> return left
            else if (is_phi(right)) && (!is_phi(left)){
                left
            }

            //if left and right are the same  -> return left/right
            else if pretty(&left) == pretty(&right){
                left
            }
            else{
                x
            }
        }

        Exp::Conc{left,right} => {
            //if left is eps and right is not -> return right
            if (is_eps(left)) && (!is_eps(right)){
                right
            }

            //if right is eps and left is not -> return left
            else if (is_eps(right)) && (!is_eps(left)){
                left
            }

            //if left or right (or both) are phi -> return phi
            else if (is_phi(right)) || (is_phi(left)){
                Box::new(Exp::Phi{})
            }
            else{
                x
            }
        }

        Exp::Star{obj} => {
            //if obj is phi -> return eps
            if is_phi(obj){
                Box::new(Exp::Eps{})
            }

            //if obj is star -> return obj
            else if pretty(&obj).ends_with("*)"){
                obj
            }
            else{
                x
            }
        }
    }
}



//helper functions to check for eps and phi
fn is_phi(x: &Exp)-> bool{
    if pretty(x) == "Phi"{
        true
    } else{
        false
    }
}
fn is_eps(x: &Exp)->bool{
    if pretty(x) == ""{
        true
    } else{
        false
    }
}



pub fn main() {

    let e1 = Box::new(Exp::Char{val : 'a'});
    let etest = Box::new(Exp::Char{val : 'c'});
    
    let e2 = Exp::Alt{left: Box::new(Exp::Char{val : 'a'}), right : Box::new(Exp::Alt{left : e1, right : Box::new(Exp::Char{val : 'c'})})}; 

    simplify(&e2);

   
    
    // Box::new(Exp::Char{val : 'a'})  
    // Box::new(Exp::Eps{})
    // Box::new(Exp::Phi{})
    // Box::new(Exp::Conc{left: , right: })
    // Box::new(Exp::Alt{left: , right: })
    // Box::new(Exp::Star{obj: })

    //This needs to be displayed correctly: eps ((a*)* (phi | b)) -> (a*) b
    let e3 = Box::new(Exp::Conc{left: Box::new(Exp::Eps{}) , right: Box::new(Exp::Conc{left: Box::new(Exp::Star{obj: Box::new(Exp::Star{obj: Box::new(Exp::Char{val : 'a'})}) }) , right: Box::new(Exp::Alt{left: Box::new(Exp::Phi{}) , right: Box::new(Exp::Char{val : 'b'}) })}) });
                    
    //I will construct a regular expression -> i will call the simplify method on it -> i will call the pretty function on the result of the simplifyer
}



