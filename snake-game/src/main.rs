fn main() {
  // float and compound types
  // https://google.github.io/comprehensive-rust/basic-syntax/compound-types.html
  let float_num: f32 = 3.14;
  let float_num_2: f64 = 3.2334327489;

  // The empty tuple () is also known as the “unit type”
  // it is used to indicate `void`
  let tup: (i32, &str, u8) = (20, "Hello", 1);

  println!("{}", tup.1); // Hello

  // destructing the tuple
  let (a, b, c) = tup;
  println!("{}", a); // 20

  // you cannot change the size of the array after declaration
  let x = [1, 5, 6, 7];

  println!("{}", x[2]); // 6

  // generate to a range
  let y = [2; 6]; // [2, 2, 2, 2, 2, 2]
  println!("{}", y[5]); // 2
}
