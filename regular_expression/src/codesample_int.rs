pub trait Exp {
    fn eval(&self) -> i32;
}

pub struct Plus<T:Exp> {
    pub left: T,
    pub right: T

}

pub struct Int {
    pub val: i32
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