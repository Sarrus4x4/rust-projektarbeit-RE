use crate::re3;
use crate::re3::Exp;


pub enum Automata {
    Transition{
        from: i32,
        c: char,
        to: i32,
        epsilon: bool,
    },
    // Eps_Transition{
    //     from: i32,
    //     to: i32,
    //     epsilon: bool,
    // },
    NFA {
        transitions: Vec<Automata>, //Vec<Automaa::Transition>
        initial_state: i32,
        final_state: Vec<i32>,
    },
    TransitionWorker{
        name_supply: i32,
    }
}

//impl Automata{
    fn eps_transition (_from: i32, _to: i32){
        Box::new(Automata::Transition{from: _from,c: ' ', to: _to, epsilon: true});
    }
    fn transition (_from: i32, _c: char, _to: i32){
        Box::new(Automata::Transition{from: _from, c: _c, to: _to, epsilon: false});
    }
    fn is_epsilon_transition(x: &Automata)->bool{ //Do i even need this with thow types of Transition?
        let is_eps: bool = false;
        is_eps
    }
    fn to_state(x: &Automata)->i32{
        let to: i32 = 1;
        let to2: i32 = x.Transition.to;
        to
    }
//}






