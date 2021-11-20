
// ######### Structs #########
pub struct Transition{
    pub from: i32,
    pub char: char,
    pub to: i32,
    pub epsilon: bool,
}
pub struct NFA{
    pub transitions: vec![], //of 'type' Transition 
    pub initialState: i32,
    pub finalState: vec![], //of 'type' i32
}
pub struct TransformWorker{
    // Kein Plan
    pub nameSupply: i32,
}

// ######### Traits #########
pub trait TransitionRules{
    fn transition(&self,_from: i32, _to: i32);
    fn eps_transition(&self, _from: i32, _c: char, _to: i32);
    fn is_epsilon_transition(&self)->bool;
    fn to_state(&self)->i32;
    fn trigger(&self, from: i32, char: char)->bool;
    fn eps_trigger(&self, from: i32)->bool;
}
pub trait NFARules {
    fn create_NFA(&self, _transitions: vec![], _initialState: i32, _finalState: vec![]);
    fn create_NFA_one_final(&self, _transitions: vec![], _initialState: i32, _finalState: i32);
    fn getTransitions(&self)->vec![]; //of 'type' Transition
    fn getInitial(&self)->i32;
    fn getFinals(&self)->vec![]; //of 'type' i32
}
pub trait TransformWorkerRules{
    // Kein Plan
    fn init(&self);
    fn fresh(&self)->i32;
    fn transform(/* r: RE */)->NFA;
    fn transformWorker(/* r: RE */)->NFA;
}

// ######### Implementation #########

//implenting the Transition Rules for the Transition Struct
impl TransitionRules for Transition {
    fn transition(&self,_from: i32, _to:i32){
        self.from = _from;
        self.to = _to;
        self.epsilon = true;
    }
    fn eps_transition(&self,_from: i32, _c: char, _to: i32){
        self.from = _from;
        self.char = _c;
        self.to = _to;
        self.epsilon = false;
    }
    fn is_epsilon_transition(&self)->bool{
        self.epsilon
    }
    fn to_state(&self)->i32{
        self.to
    }
    fn trigger(&self, from: i32, char: char)->bool{
        !self.epsilon && from == self.from && char == self.char
    }
    fn eps_trigger(&self, from: i32) ->bool{
        self.epsilon && from == self.from
    }
}

//Implementing the NFA Rules for the NFA Struct
impl NFARules for NFA { 
    fn create_NFA(&self, _transitions: vec![], _initialState: i32, _finalState: vec![]){
        self.transitions = _transitions;
        self.initialState = _initialState;
        self.finalState = _finalState;
    }
    fn create_NFA_one_final(&self, _transitions: vec![], _initialState: i32, _finalState: i32){
        self.transitions = _transitions;
        self.initialState = _initialState;
        //self.finalState = _finalState;
        self.finalState.push_back(_finalState);
    }
    fn getTransitions(&self)->vec![]{
        self.transitions
    }
    fn getInitial(&self)->i32{
        self.initialState
    }
    fn getFinals(&self)->i32{
        self.finalState
    }
}

impl TransformWorkerRules for TransformWorker{
    fn init(&self){
        self.nameSupply = 0;
    }
    fn fresh(&self)->i32 {
        self.nameSupply+1
    }
    fn transform(/* r: RE */) -> NFA{
        /*
        init(&self);
        transformWorker(r)
        */
    }
    fn transformWorker(/* r: RE */) -> NFA{
        // TODO
    }
}