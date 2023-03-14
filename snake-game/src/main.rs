fn main() {
  // store the number in the stack
  let num = 32;
  // store the number in the heap
  let num_3 = Box::new(100);

  println!("{}", num_3);
}
