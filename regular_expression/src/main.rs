mod ausdruck;
mod automat;
mod sprache;




pub fn main(){
    //ausdruck::main();
    sprache::run(automat::run());
}
