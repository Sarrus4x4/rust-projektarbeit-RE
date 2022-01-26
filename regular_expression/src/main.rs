mod automata;
mod expression;
mod language;

use crate::expression::Exp;

/*
create a Regular Expression re 
    -> create the automata nfa for that Expression 
        -> check if a word is in the language created by that automata/expression (the word to check for is currently defined in language.rs)
*/
pub fn main(){

    //this expression is equal to: eps ((a*)* (phi | b)) or even better (a*) b
    let test_expression = Box::new(Exp::Conc{
        left: Box::new(Exp::Eps{}) , 
        right: Box::new(Exp::Conc{
            left: Box::new(Exp::Star{
                obj: Box::new(Exp::Star{
                    obj: Box::new(Exp::Char{val : 'a'})
                }) 
            }), 
            right: Box::new(Exp::Alt{
                left: Box::new(Exp::Phi{}), 
                right: Box::new(Exp::Char{val : 'b'}) 
            })
        }) 
    });



    //Sinle Test (turn on prints in run functions for more information)
    let mut word_to_check = "aaab";
    language::run(automata::run(expression::run(*test_expression)), &mut word_to_check);


    // //Mass Test (turn off prints in run functions for better readability)
    // let mut word_list = ["","b","ab","abb","a","aab","aabb","aaaaaaaaaaaaaaaab","2", "This should return false"];
    // let result_list = ["false","true","true","false","false","true","false","true","false","false"];

    // for w in 0..word_list.len(){
    //     println!("This is Test {}", (w+1));
    //     language::run(automata::run(expression::run(*test_expression.clone())), &mut word_list[w]);
    //     println!("{}", result_list[w]);
    //     println!("\n");
    // }
    
}