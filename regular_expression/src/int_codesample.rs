/*

AST for a simple expression language (numbers and addition)
and some basic functionality (evaluation)

*/





pub struct Plus<T:Exp> {
    pub left: T,
    pub right: T

}

pub struct Int {
    pub val: i32
}

pub trait Exp {
    fn eval(&self) -> i32;
}

impl Exp for Int {
    fn eval(&self) -> i32 {
        return self.val
    }
}

impl<T:Exp> Exp for Plus<T> {
    fn eval(&self) -> i32 {
      return self.left.eval() + self.right.eval()
    }

}


pub fn main() {
    let e = Plus { left: Int{val:55}, right: Int{val:2}};

    println!("{} done",e.eval());

}