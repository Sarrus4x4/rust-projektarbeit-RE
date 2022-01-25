use crate::automata::NFA;
use crate::automata::Transition;
use std::fmt::Debug;


// ######### Structs #########

#[derive(Clone, Debug)]
pub struct Language{
    pub current: Vec<i32>,
    pub new: Vec<i32>,
    pub nfa: NFA,
}

// ######### Implementation #########

//Implementing the Sprache Struct
impl Language {

    //main worker function 
    pub fn function(mut self, wort: &mut &str)-> bool{
        self.current.push(self.nfa.initial_state);
        
        while wort.chars().count() > 0 {
            self.all_eps_trans();
            //println!("{:?}", self.current); //debugging
            self.change_states(consume_char(wort));
            //println!("{:?}", self.current); //debugging
            if self.clone().current_empty() {
                //the provided word is NOT part of the language created by the automata
                return false;
            }
        }
        self.all_eps_trans();
        //println!("{:?}", self.current); //debugging
        
        //the provided word is part of the language created by the automata
        self.accepted()

    }

    //get all regular transitions
    fn get_trans(&mut self,initial_state: i32, char: char){

        for t in self.nfa.clone().transitions{
            if Transition::trigger(t, initial_state, char){
                //check if this state is already in the list and add it if not
                if !(self.clone().check_new_states(t.to)){
                    self.new.push(t.to);
                }
            }
        }
        
    }

    //get all eps transitions
    fn get_eps_trans(&mut self,initial_state: i32){

        for t in self.nfa.clone().transitions{
            if Transition::trigger(t, initial_state, '\0'){
                //check if this state is already in the list and add it if not
                if !(self.clone().check_state(t.to)){
                    self.current.push(t.to);
                }
            }
        }
        
    }

    //update state list
    fn change_states(&mut self, char: char){

        for s in self.clone().current{
            self.get_trans(s, char);
        }

        self.current = self.new.to_vec();
        self.new.clear();
    }

    //check if new state is already in list
    fn check_state(self, state: i32)->bool{

        for s in self.current {
            if s == state{
                return true; 
            }
        }

        false
    }

    //check if new state is already in new list
    fn check_new_states(self, state: i32) -> bool {

        for s in self.new {
            if s == state{
                return true; 
            }
        }

        false
    }

    //check if one of the states in the list is a final state
    fn accepted (self)->bool{

        for s in self.current{
            if s == self.nfa.final_state {
                return true;
            }
        }

        false
    }

    //get all eps transitions
    fn all_eps_trans (&mut self){

        let mut old_size = 0;

        while old_size < self.current.len(){
            old_size = self.current.len();

            //println!("checkpoint{}{}{:?}", old_size, self.current.len(),self.clone().current); //debugging
            for s in self.clone().current{
                self.get_eps_trans(s);
            }
        }
    }

    //clear current list
    fn current_empty(self) -> bool {
        self.current.is_empty()
    }

}

//helper function to 'consume' char from word
pub fn consume_char(wort: &mut &str)->char{

    let c: char = wort.chars().nth(0).unwrap();
    *wort = &wort[1..];
    c
}

pub fn run(nfa: NFA){

    let mut word_to_check = "aaaab";

    let sprache = Language{nfa, new: vec![], current: vec![]};
    let return_value = sprache.function(&mut word_to_check);

    println!("Is the provided word {} in the given language? \n {}",word_to_check, return_value);
}