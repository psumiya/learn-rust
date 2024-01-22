fn reverse(tuple: (i32, bool)) -> (bool, i32) {
    let (ip, bp) = tuple;
    (bp, ip)
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

fn main() {
  let x = reverse((5, true));
  println!("{:?}", x);

  let y = Matrix(1.1, 2.2, 3.3, 4.4);
  println!("{:?}", y);
}

