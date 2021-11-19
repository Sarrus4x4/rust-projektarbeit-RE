
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
    fn transition(&self);
    fn eps_transition(&self);
    fn is_epsilon_transition(&self)->bool;
    fn to_state(&self)->i32;
    fn trigger(from: i32, char: char)->bool;
    fn eps_trigger(from: i32)->bool;
}
pub trait NFAtrait {
    fn create_NFA(_transitions: vec![], _initialState: i32, _finalState: vec![])->NFA;
    fn create_NFA_one_final(_transitions: vec![], _initialState: i32, _finalState: i32)->NFA;
    fn getTransitions(&self)->vec![]; //of 'type' Transition
    fn getInitial(&self)->i32;
    fn getFinals(&self)->vec![]; //of 'type' i32
}
pub trait TransformWorkerRules{
    // Kein Plan
    fn init(&self);
    fn fresh(&self)->i32;
    fn transform(/* RE */)->NFA;
    fn transformWorker(/* RE */)->NFA;
}