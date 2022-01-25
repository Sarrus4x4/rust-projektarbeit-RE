use crate::ausdruck::Exp;
use crate::automat::NFA;
use crate::automat::Transition;

use std::fmt::Debug;

#[derive( Clone, Debug)]
pub struct Sprache{
    pub current: Vec<i32>,
    pub nfa: NFA,
}

impl Sprache {
    pub fn function(mut self, wort: &mut &str)-> bool{
        self.current.push(self.nfa.initial_state);
        
        while wort.chars().count() > 0 {
            self.all_eps_trans();
            self.add_states(consume_char(wort));
        }
        self.all_eps_trans();
        
        self.accepted()

    }
    fn get_trans(&mut self,initial_state: i32, char: char){
        for t in self.nfa.clone().transitions{
            if Transition::trigger(t, initial_state, char){
                if !(self.clone().check_state(t.to)){
                    self.current.push(t.to);
                }
                //wenn keine einziges mal in das if gelaufen wird alarm alarm

            }
        }
    }
    fn add_states(&mut self, char: char){
        for s in self.clone().current{
            self.get_trans(s, char);
        }
    }
    fn accepted (self)->bool{
        for s in self.current{
            if s == self.nfa.final_state {
                return true;
            }
        }
        false
    }
    fn check_state(self, state: i32)->bool{
        for s in self.current {
            if s == state{
                return true; 
            }
        }
        false
    }
    fn all_eps_trans (&mut self){
        let mut old_size = 0;
        while old_size < self.current.len(){
            for s in self.clone().current{
                self.get_trans(s, '\0');
            }
            old_size = self.current.len();
        } 
    }

}

pub fn consume_char(wort: &mut &str)->char{
    //if(wort.len == 0)
    let c: char = wort.chars().nth(0).unwrap();
    *wort = &wort[1..];
    c
}

pub fn run(nfa: NFA){
    let mut wort = "c";
    let sprache = Sprache{nfa, current: vec![]};
    let return_value = sprache.function(&mut wort, );
    println!("{}",return_value);
}