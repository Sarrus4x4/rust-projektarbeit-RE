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
    pub final_state: i32,
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
    fn get_finals(self)->i32{
        self.final_state
    }
}




//TODO: 
impl TransformWorker{

    fn init(mut self){
        self.name_supply = 0;
    }
    fn fresh(mut self)->i32 {
        self.name_supply = self.name_supply + 1;
        self.name_supply
    }
    fn transform(self, re: &Exp )->Box<NFA>{
        self.init();
        self.transform_worker(re)
    }
    fn transform_worker(self, re: &Exp)->Box<NFA>{

        let mut transitions: Vec<Transition> = vec![];
        let mut start: i32;
        let mut stop: i32;
        
        let n1 ;
        let n2 ;

        match re {
            Exp::Eps{} => {
                start = self.fresh();
                stop = self.fresh();
                
                let trans1 = Transition{from: start, char: '\0' ,to: stop };
                transitions.push(trans1);

                Box::new(NFA{transitions: transitions, initial_state: start, final_state: stop})
            }

            Exp::Phi{} => {
                start = self.fresh();
                stop = self.fresh();

                Box::new(NFA{transitions: transitions, initial_state: start, final_state: stop})
            }

            Exp::Char{val} => {
                start = self.fresh();
                stop = self.fresh();

                let trans1 = Transition{from: start, char: *val ,to: stop }; //stop is same as start -> i need to increase stop
                transitions.push(trans1);

                Box::new(NFA{transitions: transitions, initial_state: start, final_state: stop})
            }

            Exp::Alt{left,right} => {

                n1 = self.transform_worker(left);
                n2 = self.transform_worker(right);
            
                //Generate new start and stop
                start = self.fresh();
                stop = self.fresh();

                let n1_start: i32 = n1.clone().get_initial();
                let n1_stop: i32 = n1.clone().get_finals(); //move error
                let n2_start: i32 = n2.clone().get_initial(); //move error
                let n2_stop: i32  = n2.clone().get_finals(); //move error

                //collect all Transitions and connect old start stop with new ones
                let t1: Vec<Transition> = n1.clone().get_transitions(); //move error
                let t2: Vec<Transition> = n2.clone().get_transitions(); //move error
            
                transitions.extend(t1); 
                transitions.extend(t2);

                let trans1 = Transition{from: start, char: '\0' ,to: n1_start };
                let trans2 = Transition{from: start, char: '\0' ,to: n2_start };
                let trans3 = Transition{from: n1_stop, char: '\0', to: stop}; // <- replaced stop.pop().unwrap() with this 
                let trans4 = Transition{from: n2_stop, char: '\0', to: stop};
                transitions.extend([trans1, trans2, trans3, trans4]);
        
                Box::new(NFA{transitions: transitions, initial_state: start, final_state: stop})
            }

            Exp::Conc{left,right} => {
                n1 = self.transform_worker(left);
                n2 = self.transform_worker(right);

                start = self.fresh();
                stop = self.fresh();

                let n1_start: i32 = n1.clone().get_initial();
                let n1_stop: i32 = n1.clone().get_finals(); //move error
                let n2_start: i32 = n2.clone().get_initial(); //move error
                let n2_stop: i32  = n2.clone().get_finals(); //move error
                 
                let t1: Vec<Transition> = n1.clone().get_transitions(); //move error
                let t2: Vec<Transition> = n2.clone().get_transitions(); //move error
            
                transitions.extend(t1); 
                transitions.extend(t2);

                let transition1 = Transition{from: start, char: '\0', to: n1_start};
                transitions.push(transition1);
                let transition2 = Transition{from: n1_stop, char: '\0', to: n2_start};
                transitions.push(transition2);                
        
                Box::new(NFA{transitions: transitions, initial_state: start, final_state: stop})
            }

            Exp::Star{obj} => {
             // i think stop is not incremented 
                n1 = self.transform_worker(obj);

                start = self.fresh();
                stop = self.fresh();

                let n1_start: i32 = n1.clone().get_initial();
                let n1_stop: i32 = n1.clone().get_finals();

                let n1: Vec<Transition> = n1.clone().get_transitions();
                transitions.extend(n1);

                let transition1 = Transition{from: start, char: '\0', to: n1_stop};
                transitions.push(transition1);
                let transition2 = Transition{from: start, char: '\0', to: stop};
                transitions.push(transition2);
                let transition3 = Transition{from: n1_start, char: '\0', to: stop};
                transitions.push(transition3);
                
                Box::new(NFA{transitions: transitions, initial_state: start, final_state: stop})
            }
        }
    }
}

pub fn  run (){
    let e_final = Box::new(Exp::Conc{left: Box::new(Exp::Eps{}) , right: Box::new(Exp::Conc{left: Box::new(Exp::Star{obj: Box::new(Exp::Star{obj: Box::new(Exp::Char{val : 'a'})}) }) , right: Box::new(Exp::Alt{left: Box::new(Exp::Phi{}) , right: Box::new(Exp::Char{val : 'b'}) })}) });

    let test1 = Box::new(Exp::Char{val: 'a'});
    let test2 = Box::new(Exp::Conc{left: Box::new(Exp::Char{val: 'a'}) , right: Box::new(Exp::Char{val: 'b'})});
    let test3 = Box::new(Exp::Alt{left: Box::new(Exp::Char{val: 'a'}) , right: Box::new(Exp::Char{val: 'b'})});
    let test4 = Box::new(Exp::Star{obj: Box::new(Exp::Char{val: 'c'})});


    
    let a = TransformWorker{name_supply: 10};
    //let b = a.transform_worker(&test2);
    let b = a.transform(&test2);

    println!("This is a Test: {:?}", b);

}