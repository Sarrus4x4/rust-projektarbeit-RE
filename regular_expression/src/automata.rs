use crate::expression::Exp;
use std::fmt::Debug;


// ######### Structs #########
#[derive(Copy, Clone, Debug)]
pub struct Transition{
    pub from: i32,
    pub char: char,
    pub to: i32,
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

//Implementing the Transition Struct
impl Transition {
    pub fn trigger(self, from: i32, char: char)->bool{
        from == self.from && char == self.char  
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

    fn init(&mut self){
        self.name_supply = 0;
    }
    fn incr(&mut self) {
        self.name_supply += 1;
    }
    fn get(&self)-> i32 {
        self.name_supply
    }
    fn transform(mut self, re: &Exp )->Box<NFA>{
        self.init();
        self.transform_worker(re)
    }
    fn transform_worker(&mut self, re: &Exp)->Box<NFA>{

        let mut transitions: Vec<Transition> = vec![];
        let start: i32;
        let stop: i32;
        
        let n1 ;
        let n2 ;

        match re {
            Exp::Eps{} => {
                
                start = self.get();
                self.incr();
                stop = self.get();

                let trans1 = Transition{from: start, char: '\0' ,to: stop };
                transitions.push(trans1);

                Box::new(NFA{transitions: transitions, initial_state: start, final_state: stop})
            }

            Exp::Phi{} => {
                start = self.get();
                
                Box::new(NFA{transitions: transitions, initial_state: start, final_state: start})
            }

            Exp::Char{val} => {
               
                start = self.get();
                self.incr();
                stop = self.get();
                

                let trans1 = Transition{from: start, char: *val ,to: stop };
                transitions.push(trans1);

                Box::new(NFA{transitions: transitions, initial_state: start, final_state: stop})
            }

            Exp::Alt{left,right} => {
                
                start = self.get();
                self.incr();
                
                n1 = self.transform_worker(left);
                self.incr();
                n2 = self.transform_worker(right);

                self.incr();
                stop = self.get();

                let n1_start: i32 = n1.clone().get_initial();
                let n1_stop: i32 = n1.clone().get_finals();
                let n2_start: i32 = n2.clone().get_initial();
                let n2_stop: i32  = n2.clone().get_finals();

                let t1: Vec<Transition> = n1.clone().get_transitions();
                let t2: Vec<Transition> = n2.clone().get_transitions();
            
                transitions.extend(t1); 
                transitions.extend(t2);

                let trans1 = Transition{from: start, char: '\0' ,to: n1_start };
                let trans2 = Transition{from: start, char: '\0' ,to: n2_start };
                let trans3 = Transition{from: n1_stop, char: '\0', to: stop};
                let trans4 = Transition{from: n2_stop, char: '\0', to: stop};
                transitions.extend([trans1, trans2, trans3, trans4]);
        
                Box::new(NFA{transitions: transitions, initial_state: start, final_state: stop})
            }

            Exp::Conc{left,right} => {
                
                n1 = self.transform_worker(left);
                n2 = self.transform_worker(right);

                let n1_start: i32 = n1.clone().get_initial();
                let n2_stop: i32  = n2.clone().get_finals();
                 
                let t1: Vec<Transition> = n1.clone().get_transitions();
                let t2: Vec<Transition> = n2.clone().get_transitions();
            
                transitions.extend(t1); 
                transitions.extend(t2);

                Box::new(NFA{transitions: transitions, initial_state: n1_start, final_state: n2_stop})
            }

            Exp::Star{obj} => {
                
                start = self.get();
                self.incr();
                n1 = self.transform_worker(obj);
                self.incr();
                stop = self.get();

                let n1_start: i32 = n1.clone().get_initial();
                let n1_stop: i32 = n1.clone().get_finals();

                let n1: Vec<Transition> = n1.clone().get_transitions();
                transitions.extend(n1);

                let transition1 = Transition{from: start, char: '\0', to: n1_start};
                transitions.push(transition1);
                let transition2 = Transition{from: start, char: '\0', to: stop};
                transitions.push(transition2);
                let transition3 = Transition{from: n1_stop, char: '\0', to: stop};
                transitions.push(transition3);
                let transition4 = Transition{from: n1_stop, char: '\0', to: n1_start};
                transitions.push(transition4);
                
                Box::new(NFA{transitions: transitions, initial_state: start, final_state: stop})
            }
        }
    }
}

pub fn  run (exp: Exp)->NFA{
    //let e_final = Box::new(Exp::Conc{left: Box::new(Exp::Eps{}) , right: Box::new(Exp::Conc{left: Box::new(Exp::Star{obj: Box::new(Exp::Star{obj: Box::new(Exp::Char{val : 'a'})}) }) , right: Box::new(Exp::Alt{left: Box::new(Exp::Phi{}) , right: Box::new(Exp::Char{val : 'b'}) })}) });

    let my_transformworker = TransformWorker{name_supply: 0};
    let my_nfa = my_transformworker.transform(&exp);

   println!("This is the created NFA (not that readable):\n{:?}\n", my_nfa); //for debugging or display

   //return the created nfa
   *my_nfa
}