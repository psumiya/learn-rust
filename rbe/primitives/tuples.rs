fn reverse(tuple: (i32, bool)) -> (bool, i32) {
    let (ip, bp) = tuple;
    (bp, ip)
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

use std::fmt;
impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "( {} {} ) \n( {} {} )", self.0, self.1, self.2, self.3)
    }
}

fn main() {
  let x = reverse((5, true));
  println!("{:?}", x);

  let y = Matrix(1.1, 2.2, 3.3, 4.4);
  println!("{:?}", y);
  println!("{}", y);
}

