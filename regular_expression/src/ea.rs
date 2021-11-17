pub struct Transition{
    pub from: i32,
    pub char: char,
    pub to: i32,
    pub epsilon: bool,
}

pub trait TransitionRules{
    fn transition(&self);
    fn eps_transition(&self);
    fn is_epsilon_transition(&self)->bool;
    fn to_state(&self)->i32;
    fn trigger(from: i32, char: char)->bool;
    fn eps_trigger(from: i32)->bool;
}