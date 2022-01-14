use crate::re::Exp;

use std::fmt::Debug;



// ######### Structs #########
#[derive(Copy, Clone, Debug)]
pub struct Transition{
    pub from: i32,
    pub char: char,
    pub to: i32,
    //pub epsilon: bool,
}
#[derive(Clone, Debug)]
pub struct NFA{
    pub transitions: Vec<Transition>,
    pub initial_state: i32,
    pub final_state: Vec<i32>,
}
#[derive(Copy, Clone, Debug)]
pub struct TransformWorker{
    pub name_supply: i32 ,
}




// ######### Implementation #########

//implenting the Transition Rules for the Transition Struct
impl Transition {
    fn is_epsilon_transition(self)->bool{
        let mut is_epsilon: bool = false;
        if self.char == '\0' {
            is_epsilon = true;
        }
        is_epsilon
    }
    fn to_state(self)->i32{
        self.to
    }
    fn trigger(self, from: i32, char: char)->bool{
        !self.is_epsilon_transition() && from == self.from && char == self.char
    }
    fn eps_trigger(self, from: i32) ->bool{
        self.is_epsilon_transition() && from == self.from
    }
}

//Implementing the NFA Struct
impl NFA {
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
impl TransformWorker{

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
        let mut start: i32;
        let mut stop = Vec::<i32>::new(); 
        
        let n1 ;
        let n2 ;

        match re {
            Exp::Eps{} => {
                start = self.fresh();
                stop.push(self.fresh());
                
                let trans1 = Transition{from: start, char: '\0' ,to: stop.pop().unwrap() };
                transitions.push(trans1);
                let nfa = NFA{transitions: transitions, initial_state: start, final_state: stop};
                nfa
            }

            Exp::Phi{} => {
                start = self.fresh();
                stop.push(self.fresh());

                let nfa = NFA{transitions: transitions, initial_state: start, final_state: stop};
                nfa
            }

            Exp::Char{val} => {
                start = self.fresh();
                stop.push(self.fresh());

                let trans1 = Transition{from: start, char: *val ,to: stop.pop().unwrap() };
                transitions.push(trans1);
                let nfa = NFA{transitions: transitions, initial_state: start, final_state: stop};
                nfa
            }

            Exp::Alt{left,right} => {

                n1 = self.transform_worker(left);
                let test = n1.clone(); //vielleicht bringt das ja auch was... aber ich kann 'test' dann auchn nur einmal benutzen, bevor er über move meckert
                let n1_poiner: *const NFA = &n1; //vielleicht bringt der Pointer was beim lösen von dem Move Problem (Wie es aussieht bringt mich das auch nicht weiter)
                //https://stackoverflow.com/questions/28800121/what-do-i-have-to-do-to-solve-a-use-of-moved-value-error
                //bringt mich Box hier weiter?

                //aktuell 'löse' ich mein problem mit .clone()

                n2 = self.transform_worker(right);
            
                //Generate new start and stop
                start = self.fresh();
                stop.push(self.fresh());
                let n1_start: i32 = n1.clone().get_initial();
                let n1_stop: i32 = n1.clone().get_finals()[0]; //move error
                let n2_start: i32 = n2.clone().get_initial(); //move error
                let n2_stop: i32  = n2.clone().get_finals()[0]; //move error

                //collect all Transitions and connect old start stop with new ones
                let t1: Vec<Transition> = n1.clone().get_transitions(); //move error
                let t2: Vec<Transition> = n2.clone().get_transitions(); //move error
            
                transitions.extend(t1); 
                transitions.extend(t2);

                let trans1 = Transition{from: start, char: '\0' ,to: n1_start };
                let trans2 = Transition{from: start, char: '\0' ,to: n2_start };
                let trans3 = Transition{from: n1_stop, char: '\0', to: stop.pop().unwrap()};
                let trans4 = Transition{from: n2_stop, char: '\0', to: stop.pop().unwrap()};
                transitions.extend([trans1, trans2, trans3, trans4]);
        
                let nfa = NFA{transitions: transitions, initial_state: start, final_state: stop};
                nfa
            }

            Exp::Conc{left,right} => {
                n1 = self.transform_worker(left);
                start = n1.clone().get_initial();
                for t in 1..n1.clone().get_transitions().len(){ //move error
                    transitions.push(n1.clone().get_transitions()[t]);
                }    

                for t in 1..n1.clone().get_finals().len(){
                    let mut  n = self.transform_worker(right);
                    n.initial_state = n1.clone().get_finals()[t]; //move error
                    
                    for t in 1..n.clone().get_transitions().len(){
                        transitions.push(n1.clone().get_transitions()[t]); //move error
                    }
                    for t in 1..n.clone().get_finals().len(){ //move error
                        stop.push(n.clone().final_state[t]); //move error
                    }
                }
                
                let nfa = NFA{transitions: transitions, initial_state: start, final_state: stop};
                nfa
            }

            Exp::Star{obj} => {
                start = self.fresh();
                stop.push(self.fresh());

                n1 = self.transform_worker(obj);
                
                for t in 1..n1.clone().get_finals().len(){
                    let trans1 = Transition{from: n1.clone().get_finals()[t], char: '\0' ,to: n1.clone().get_initial() }; //move error
                    transitions.push(trans1);
                    let trans2 = Transition{from: n1.clone().get_finals()[t], char: '\0', to: stop.pop().unwrap() }; //move error
                    transitions.push(trans2);
                }
                let transition1 = Transition{from: start, char: '\0', to: n1.clone().get_initial()};
                transitions.push(transition1);
                let transition2 = Transition{from: start, char: '\0', to: stop.pop().unwrap()};
                transitions.push(transition2);
                
                let nfa = NFA{transitions: transitions, initial_state: start, final_state: stop};
                nfa
            }
        }
    }
}

pub fn  run (){
    let e_final = Box::new(Exp::Conc{left: Box::new(Exp::Eps{}) , right: Box::new(Exp::Conc{left: Box::new(Exp::Star{obj: Box::new(Exp::Star{obj: Box::new(Exp::Char{val : 'a'})}) }) , right: Box::new(Exp::Alt{left: Box::new(Exp::Phi{}) , right: Box::new(Exp::Char{val : 'b'}) })}) });

    let a = TransformWorker{name_supply: 0};
    let b = a.transform_worker(&e_final);
    println!("This is a Test: {:?}", b);

}