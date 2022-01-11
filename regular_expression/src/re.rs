#[derive( Clone, Debug)]

//enum that contains all building blocks of regular expressions
//the variants can be called recursive to construct any required expression 
pub enum Exp {

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

//pretty Method that returns a String with a easily readable expression
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
            let s = "(".to_string() + &pretty(&obj) + &"*)".to_string();
            return s;
        }
    }
}

//simplify Method that returns a simplified version of the expression as Exp
fn simplify(x : &Exp) -> Exp {
    match x {

        Exp::Eps{} => {
            *Box::new(Exp::Eps{})
        }

        Exp::Phi{} => {
            *Box::new(Exp::Phi{})
        }

        Exp::Char{val} => {
            *Box::new(Exp::Char{val: *val})
        } 

        Exp::Alt{left,right} => {
            //if left is Phi and right is not -> return right
            if (is_phi(left)) && (!is_phi(right)){
                simplify(right) 
            }

            //if right is Phi and left is not -> return left
            else if (is_phi(right)) && (!is_phi(left)){
                simplify(left) 
            }

            //if left and right are the same  -> return left/right
            else if pretty(&left) == pretty(&right){
                simplify(left) 
            }

            //if no special case occurs i need to return a Alt with two simplified arguments
            else{
                *Box::new(Exp::Alt{ left: Box::new(simplify(left)), right: Box::new(simplify(right))})
            }
        }

        Exp::Conc{left,right} => {
            //if left is eps and right is not -> return right
            if (is_eps(left)) && (!is_eps(right)){
                simplify(right)
            }

            //if right is eps and left is not -> return left
            else if (is_eps(right)) && (!is_eps(left)){
                simplify(left)
            }

            //if left or right (or both) are phi -> return phi
            else if (is_phi(right)) || (is_phi(left)){
                *Box::new(Exp::Phi{})
            }

            //if no special case occurs i need to return a Conc with two simplified arguments
            else{
                *Box::new(Exp::Conc{ left: Box::new(simplify(left)), right: Box::new(simplify(right))})
            }
        }

        Exp::Star{obj} => {
            //if obj is phi -> return eps
            if is_phi(obj){
                *Box::new(Exp::Eps{})
            }

            //if obj is star -> return obj
            else if pretty(&obj).ends_with("*)"){
                simplify(&obj)
            }

            //if no special case occurs i need to return a Star with a simplified argument
            else{
                *Box::new(Exp::Star{ obj: Box::new(simplify(obj))})
            }
        }
    }
}


//helper functions to check for eps and phi
fn is_phi(x: &Exp)-> bool{
    if pretty(x) == "phi"{
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
   
    //### This are my building blocks ###
    // Box::new(Exp::Char{val : 'a'})  
    // Box::new(Exp::Eps{})
    // Box::new(Exp::Phi{})
    // Box::new(Exp::Conc{left: , right: })
    // Box::new(Exp::Alt{left: , right: })
    // Box::new(Exp::Star{obj: })
    
    //This needs to be displayed correctly: eps ((a*)* (phi | b)) -> (a*) b
    let e_final = Box::new(Exp::Conc{left: Box::new(Exp::Eps{}) , right: Box::new(Exp::Conc{left: Box::new(Exp::Star{obj: Box::new(Exp::Star{obj: Box::new(Exp::Char{val : 'a'})}) }) , right: Box::new(Exp::Alt{left: Box::new(Exp::Phi{}) , right: Box::new(Exp::Char{val : 'b'}) })}) });
    println!("This should print the expression tree: {:?}",simplify(&e_final));
    println!("This should print the expression: {:?}",pretty(&simplify(&e_final)));

}



