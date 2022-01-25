mod automata;
mod expression;
mod language;

/*
create a Regular Expression re 
    -> create the automata nfa for that Expression 
        -> check if a word is in the language created by that automata/expression (the word to check for is currently defined in language.rs)
*/
pub fn main(){

    //each step will print its own result on the way
    language::run(automata::run(expression::run()));
}
