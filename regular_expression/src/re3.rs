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

// Show expressions.
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


//first i wanted to use a simplify method before using the pretty method but the simplify method turned out to to the job of pretty already...

// // Show for expressions.
// fn simplify(x : &Exp) -> Box<Exp> { //-> Box<Exp> 
//     match x {

//         Exp::Eps{} => {
//             //simply return the given parameter x (currently returning new Exp because i dont know how to return x)
//             Box::new(Exp::Eps{})
//         }

//         Exp::Phi{} => {
//             //simply return the given parameter x (currently returning new Exp because i dont know how to return x)
//             Box::new(Exp::Phi{})
//         }

//         Exp::Char{val} => {
//             //simply return the given parameter x (currently returning new Exp because i dont know how to return x)
//             let temp_val: char = *val;
//             Box::new(Exp::Char{val: temp_val})
//         } 

//         Exp::Alt{left,right} => {
//             //if left is Phi and right is not -> return right
//             if (is_phi(left)) && (!is_phi(right)){
//                 right //<-- how can i return this as Box<Exp>?
//             }

//             //if right is Phi and left is not -> return left
//             else if (is_phi(right)) && (!is_phi(left)){
//                 left //<-- how can i return this as Box<Exp>?
//             }

//             //if left and right are the same  -> return left/right
//             else if pretty(&left) == pretty(&right){
//                 left //<-- how can i return this as Box<Exp>?
//             }
//             else{
//                 x //<-- how can i return this as Box<Exp>?
//             }
//         }

//         Exp::Conc{left,right} => {
//             //if left is eps and right is not -> return right
//             if (is_eps(left)) && (!is_eps(right)){
//                 right //<-- how can i return this as Box<Exp>?
//             }

//             //if right is eps and left is not -> return left
//             else if (is_eps(right)) && (!is_eps(left)){
//                 left //<-- how can i return this as Box<Exp>?
//             }

//             //if left or right (or both) are phi -> return phi
//             else if (is_phi(right)) || (is_phi(left)){
//                 Box::new(Exp::Phi{})
//             }
//             else{
//                 x //<-- how can i return this as Box<Exp>?
//             }
//         }

//         Exp::Star{obj} => {
//             //if obj is phi -> return eps
//             if is_phi(obj){
//                 Box::new(Exp::Eps{}) 
//             }

//             //if obj is star -> return obj
//             else if pretty(&obj).ends_with("*)"){
//                 obj //<-- how can i return this as Box<Exp>?
//             }
//             else{
//                 x //<-- how can i return this as Box<Exp>?
//             }
//         }
//     }
// }

//simplify Method that also returns the expression ready to be printed (the pretty method is called inside of simplify)
fn simplify(x : &Exp) -> String { //-> Box<Exp> 
    match x {

        Exp::Eps{} => {
            //simply return the given parameter x (currently returning new Exp because i dont know how to return x)
            let e = "";
            e.to_string()
        }

        Exp::Phi{} => {
            //simply return the given parameter x (currently returning new Exp because i dont know how to return x)
            let e = "phi";
            e.to_string()
        }

        Exp::Char{val} => {
            //simply return the given parameter x (currently returning new Exp because i dont know how to return x)
            let e = *val;
            e.to_string()
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
            else{
                let e = "(".to_string() + &simplify(&left) + &"|".to_string()+ &simplify(&right) + &")".to_string();
                e
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
                let e = "phi";
                e.to_string()
            }
            else{
                let e = "(".to_string() + &simplify(&left) + &simplify(&right) + &")".to_string();
                e 
            }
        }

        Exp::Star{obj} => {
            //if obj is phi -> return eps
            if is_phi(obj){
                let e = "";
                e.to_string() 
            }

            //if obj is star -> return obj
            else if pretty(&obj).ends_with("*)"){
                let e = simplify(&obj);
                e 
            }
            else{
                let e = "(".to_string() + &simplify(&obj) + &"*)".to_string();
                e
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

    let e1 = Box::new(Exp::Char{val : 'a'});
    println!("This is a test: {}",pretty(&e1));

    //This needs to be displayed correctly: eps ((a*)* (phi | b)) -> (a*) b
    let e_final = Box::new(Exp::Conc{left: Box::new(Exp::Eps{}) , right: Box::new(Exp::Conc{left: Box::new(Exp::Star{obj: Box::new(Exp::Star{obj: Box::new(Exp::Char{val : 'a'})}) }) , right: Box::new(Exp::Alt{left: Box::new(Exp::Phi{}) , right: Box::new(Exp::Char{val : 'b'}) })}) });
    println!("This is the final Test: {}",simplify(&e_final));
    //It works! Only the outer braces could be removed later...
}



