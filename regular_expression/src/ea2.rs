//use crate::re;
use crate::re2::RE;
//use std::any::type_name;
use std::fmt::Debug;

//extern crate re;
use crate::re2::Eps;
use crate::re2::Phi;
use crate::re2::C;
use crate::re2::Alt;
use crate::re2::Conc;
use crate::re2::Star;
use std::any::type_name;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

// ######### Structs #########
pub struct Transition{
    pub from: i32,
    pub char: char,
    pub to: i32,
    pub epsilon: bool,
}

pub struct NFA{
    pub transitions: Vec<Transition>,
    pub initial_state: i32,
    pub final_state: Vec<i32>,
}
#[derive(Copy, Clone)]
pub struct TransformWorker{
    pub name_supply: i32 ,
}

// ######### Traits #########
pub trait TransitionRules{
    fn transition(self,_from: i32, _to: i32);
    fn eps_transition(self, _from: i32, _c: char, _to: i32);
    fn is_epsilon_transition(self)->bool;
    fn to_state(self)->i32;
    fn trigger(self, from: i32, char: char)->bool;
    fn eps_trigger(self, from: i32)->bool;
}
pub trait NFARules {
    fn create_nfa(self, _transitions: Vec<Transition>, _initial_state: i32, _final_state: Vec<i32>);
    fn create_nfa_one_final(self, _transitions: Vec<Transition>, _initial_state: i32, _final_state: i32);
    fn get_transitions(self)->Vec<Transition>;
    fn get_initial(self)->i32;
    fn get_finals(self)->Vec<i32>;
}
pub trait TransformWorkerRules{
    // Kein Plan
    fn init(self);
    fn fresh(self)->i32;
    fn transform<T: RE+Debug+NFARules>(self, re: &T )->NFA;

    fn transform_worker_eps(self, re: Eps )->NFA;
    fn transform_worker_phi(self, re: Phi )->NFA;
    fn transform_worker_char(self, re: C )->NFA;
    fn transform_worker_alt<T:RE, J:RE>(self, re: Alt<T,J> )->NFA;
    fn transform_worker_conc<T:RE, J:RE>(self, re: Conc<T,J> )->NFA;
    fn transform_worker_star<T:RE>(self, re: Star<T> )->NFA;
}

// ######### Implementation #########

//implenting the Transition Rules for the Transition Struct
impl TransitionRules for Transition {
    fn transition(mut self,_from: i32, _to:i32){
        self.from = _from;
        self.to = _to;
        self.epsilon = true;
    }
    fn eps_transition(mut self,_from: i32, _c: char, _to: i32){
        self.from = _from;
        self.char = _c;
        self.to = _to;
        self.epsilon = false;
    }
    fn is_epsilon_transition(self)->bool{
        self.epsilon
    }
    fn to_state(self)->i32{
        self.to
    }
    fn trigger(self, from: i32, char: char)->bool{
        !self.epsilon && from == self.from && char == self.char
    }
    fn eps_trigger(self, from: i32) ->bool{
        self.epsilon && from == self.from
    }
}

//Implementing the NFA Rules for the NFA Struct
impl NFARules for NFA {
    fn create_nfa(mut self, _transitions: Vec<Transition>, _initial_state: i32, _final_state: Vec<i32>){
        self.transitions = _transitions;
        self.initial_state = _initial_state;
        self.final_state = _final_state;
    }
    fn create_nfa_one_final( mut self, _transitions:  Vec<Transition>, _initial_state: i32, _final_state: i32){
        self.transitions = _transitions;
        self.initial_state = _initial_state;
        self.final_state.push(_final_state); //append final state
    }
    fn get_transitions(self)-> Vec<Transition>{
        self.transitions
    }
    fn get_initial(self)->i32{
        self.initial_state
    }
    fn get_finals(self)->Vec<i32>{
        self.final_state
    }
}

impl TransformWorkerRules for TransformWorker{

    fn init(mut self){
        self.name_supply = 0;
    }
    fn fresh(self)->i32 {
        self.name_supply+1
    }
    fn transform<T:RE+Debug+NFARules>(self, re: &T )->NFA{
        self.init();
        match type_of(re){
            "regular_expression::re::C" => { self.transform_worker_char(re);}
        }
        self.transform_worker_eps(re) // <--- i need to know the type of re to call the right transform_worker function!
        //Now i have the same problem in transform that i had in transform_worker before... I need a match (switch case) for all types of re
    }

    //I need to know the "type" of the parameter i am getting so i can call the next function!
    //If i Start with a Conc of two Chars i need to know that the two agruments of Conc are of "type" Char, so i can call transform_worker_c
    //Maybe i need to add a Type_of function to the RE trait, so i can call it on Eps, Phi, ...

    fn transform_worker_eps(self, re: Eps )->NFA{
        // TODO
        let mut transitions: Vec<Transition> = vec![];
        let mut start: i32 = 0;
        let mut stop: i32 = 0;

        start = self.fresh();
        stop = self.fresh();
        let temp_nfa_eps =  NFA{initial_state: 0, final_state: vec![], transitions: vec![]};
        temp_nfa_eps.create_nfa_one_final(transitions, start, stop);
        temp_nfa_eps
    }
    fn transform_worker_phi(self, re: Phi )->NFA{
        // TODO
        let mut transitions: Vec<Transition> = vec![];
        let mut start: i32 = 0;
        let mut stop: i32 = 0;

        start = self.fresh();
        stop = self.fresh();
        let temp_nfa_phi =  NFA{initial_state: 0, final_state: vec![], transitions: vec![]} ;
        temp_nfa_phi.create_nfa_one_final(transitions, start, stop);
        temp_nfa_phi
    }
    fn transform_worker_char(self, re: C )->NFA{
        // TODO
        let mut transitions: Vec<Transition> = vec![];
        let mut start: i32 = 0;
        let mut stop: i32 = 0;

    }
    fn transform_worker_alt<T:RE, J:RE>(self, re: Alt<T,J> )->NFA{
        // TODO
        let mut transitions: Vec<Transition> = vec![];
        let mut start: i32 = 0;
        let mut stop: i32 = 0;

        //this is what i need 
        // NFA n1 = transformWorker(r2->getLeft());
        // NFA n2 = transformWorker(r2->getRight());
        
        //this is what i have
        // i need to know the type of re.l and re.r so i can call the right transformworker function
        let n1;
        let n2;
        match re.l{
            Eps => {
                n1 = self.transform_worker_eps(re.l); //<--- So hardcoding the parameter of transformworker into a set Type is not the right way?
            }
            Phi => {
                n1 = self.transform_worker_phi(re.l);
            }
            C => {
                n1 = self.transform_worker_char(re.l);
            }
            Alt => {
                type_of(re.l);
                n1 = self.transform_worker_alt(re.l);
            }
            Conc => {
                n1 = self.transform_worker_conc(re.l);
            }
            Star => {
                n1 = self.transform_worker_star(re.l);
            }
        }
        match re.r{
            Eps => {
                n2 = self.transform_worker_eps(re.r);
            }
            Phi => {
                n2 = self.transform_worker_phi(re.r);
            }
            C => {
                n2 = self.transform_worker_char(re.r);
            }
            Alt => {
                n2 = self.transform_worker_alt(re.r);
            }
            Conc => {
                n2 = self.transform_worker_conc(re.r);
            }
            Star => {
                n2 = self.transform_worker_star(re.r);
            }
        }

        start = self.fresh();
        stop = self.fresh();
        let n1_start: i32 = n1.get_initial();
        let n1_stop: i32 = n1.get_finals()[0];
        let n2_start: i32 = n2.get_initial();
        let n2_stop: i32 = n2.get_finals()[0];

        let t1: Vec<Transition> = n1.get_transitions();
        let t2: Vec<Transition> = n2.get_transitions();

        transitions.insert(transitions.end(),t1.begin(),t1.end()); //<--- why are there 3 arguments? How does insert work in C++?
   

        
        println!(""); //<-- to stop the return error
    }
    fn transform_worker_conc<T:RE, J:RE>(self, re: Conc<T,J> )->NFA{
        // TODO
        let mut transitions: Vec<Transition> = vec![];
        let mut start: i32 = 0;
        let mut stop: i32 = 0;


    }
    fn transform_worker_star<T:RE>(self, re: Star<T> )->NFA{
        // TODO
        let mut transitions: Vec<Transition> = vec![];
        let mut start: i32 = 0;
        let mut stop: i32 = 0;


    }
}

// transformworker als überladene Methode mit den Instantzen eps, phi,...
// Die jeweiligen Methoden die zu einem Trait gehören können fuer bestimmte Typen implementiert werden.

pub fn  run (){

}