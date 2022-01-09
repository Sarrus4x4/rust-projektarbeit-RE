use crate::re::Exp;

use std::fmt::Debug;


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
    fn init(self);
    fn fresh(self)->i32;
    fn transform(self, re: &Exp )->NFA;
    fn transform_worker(self, re: &Exp )->NFA;
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




//TODO: 
//This is what i am trying to do: 
//Transform the C++ Code (https://www.cplusplus.com/reference/vector/vector/insert/) into Rust Code
//The part I am working on below is the equivalent to the last 'Code Box' right befor Part3 of the C++ document.
impl TransformWorkerRules for TransformWorker{

    fn init(mut self){
        self.name_supply = 0;
    }
    fn fresh(self)->i32 {
        self.name_supply+1
    }
    fn transform(self, re: &Exp )->NFA{
        self.init();
        self.transform_worker(re)
    }
    fn transform_worker(self, re: &Exp)->NFA{

        let mut transitions: Vec<Transition> = vec![];
        let mut start: i32 = 0;
        let mut stop: i32 = 0; 
        //TODO: 
        //'stop' is initialized as a i32 but it could be required to be a Vec<i32> when i have to return 'create_nfa' down below! How do i solve this?

        let n1;
        let n2;
    
        match re {
            Exp::Eps{} => {
                //do i have to do anything here?
            }

            Exp::Phi{} => {
                //do i have to do anything here?
            }

            Exp::Char{val} => {
                //do i have to do anything here?
            }

            Exp::Alt{left,right} => {
                n1 = self.transform_worker(left);
                n2 = self.transform_worker(right);
            }

            Exp::Conc{left,right} => {
                n1 = self.transform_worker(left);
                n2 = self.transform_worker(right);
            }

            Exp::Star{obj} => {
                n1 = self.transform_worker(obj);
            }
        }

        //TODO:
        //Find out how to use functions of NFA! Traitbounds?! But i only use one parameter that has to be of type &Exp
      
        //Generate new start and stop
        start = self.fresh();
        stop = self.fresh();
        let n1_start: i32 = n1.getInitial();
        let n1_stop: i32 = n1.getFinals()[0];
        let n2_start: i32 = n2.getInitial();
        let n2_stop: i32  = n2.getFinals()[0];

        //collect all Transitions and connect old start stop with new ones
        let t1: Vec<Transition> = n1.getTransitions();
        let t2: Vec<Transition> = n2.getTransitions();
        
        //TODO:
        //Find out how Vectors work in rust and edit the lines below (those are from the C++ File i linked in my Repo and i don't get why insert has 3 arguments)
        // https://www.cplusplus.com/reference/vector/vector/insert/
        // end: Returns an iterator referring to the past-the-end element in the vector container.
        // begin: Returns an iterator pointing to the first element in the vector.
        // Do the C++ lines mean "put all the elements of t1 and t2 at the end of transitions"?

        //transitions.insert(transitions.end(),t1.begin(),t1.end()); <- those are the C++ lines i tried to transform into Rust
        //transitions.insert(transitions.end(),t2.begin(),t2.end());
        transitions.extend(t1); //<- this is (hopefully) the equivalent Rust code
        transitions.extend(t2);

        transitions.push(transition(start, n1_start));
        transitions.push(transition(start, n2_start));
        transitions.push(transition(n1_stop, stop));
        transitions.push(transition(n2_stop, stop));
   

        //TODO:
        //Differentiate between "one final state" and "final state vector" and return accordingly 
        create_nfa_one_final(transitions,start,stop)
        //create_nfa(transitions,start,stop)
    }
    
}

pub fn  run (){
    
}