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
pub trait TransformWorkerRulesAlt{
    // Kein Plan
    fn init(self);
    fn fresh(self)->i32;
    fn transform<T: RE, J: RE>(self, re: Alt<T,J> );
    fn transform_worker<T: RE,J: RE>(self, re: Alt<T,J> );
    //i need to 'overload' the transform_worker function but overloading is not possible in rust -> i need to use multible similar functions with different parameters (Alt, Eps,...) like 'transform_worker_alt' 
    //if i use multible functions (like 'transform_worker_alt' i would have to implement all of them for all my Types -> multible traits 'TransformWorkerRulesAlt'?)
}
//do i need mutlible traits for Transformworker?





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



//In this file i try to implement the TransformWorkerRules for every type (phi, eps,...) seperatly -> TransformWorkerRulesAlt for Type Alt<T,J>
//I am calling the transformWorker function with the parameter re of type Alt<T,J>

//Implementing the TransformWorkerRules for the Alt type
impl <T:RE+NFARules, J:RE+NFARules> TransformWorkerRulesAlt for Alt<T,J>{

    fn init(mut self){
        self.name_supply = 0;
    }
    fn fresh(self)->i32 {
        self.name_supply+1
    }
    fn transform<S:RE, I:RE >(self, re: Alt<S,I> ){
        self.init();
        self.transform_worker(re)
    }
    fn transform_worker<S:RE, I:RE>(self, re: Alt<S,I>){
        // TODO
        let mut transitions: Vec<Transition> = vec![];
        let mut start: i32 = 0;
        let mut stop: i32 = 0;

        //this is what i need 
        // NFA n1 = transformWorker(r2->getLeft());
        // NFA n2 = transformWorker(r2->getRight());
        
        //this is what i have
        //i need to know the type of re.l and re.r so i can call the right transformworker function -> this is why i need to use the Alt<S,I> parameter instead of a generic one

        let n1;
        let n2;

        //check type of left argument to call next transformworker function
        //type_of(re) -> regular_expression::re::Alt
        match type_of(re.l){
            "regular_expression::re::Alt" => {
                type_of(re.l);
                n1 = self.transform_worker(re.l); //<- this is my problem: expected struct `re2::Alt`, found type parameter `S` (but if i use re as a generic parameter i can't use re.l and re.r)
            }
            //others cases follow...
            //in other cases i would have to call the transformworker function of another Trait (for example the transformWorker that was implemented for Eps)
        }

        //check type of right argument to call next transformworker function
        //type_of(re) -> regular_expression::re::Alt
        match type_of(re.r){
            "regular_expression::re::Alt" => {
                n2 = self.transform_worker(re.r); //<- this is my problem: expected struct `re2::Alt`, found type parameter `I` (but if i use re as a generic parameter i can't use re.l and re.r)
            }
            //others cases follow...
            //in other cases i would have to call the transformworker function of another Trait (for example the transformWorker that was implemented for Eps)
        }

        //more code...
    }
    
}


//TODO
// transformworker als überladene Methode mit den Instantzen eps, phi,...
// Die jeweiligen Methoden die zu einem Trait gehören können fuer bestimmte Typen implementiert werden.

pub fn  run (){
}