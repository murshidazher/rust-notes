fn main() {
  // you can place on stack only values with static size
  let a = 10;
  let b = a;
  let c = 15;
  let d = add(a, b);

  println!("{}", d);

  // compiler doesn't know the size in advance
  // hence the string isn't placed in stack
  // hence it will be stored in the heap memory since its dynamic
  // the pointer (address) to the heap memory will be saved in stack
  // ptr -> capacity (size in memory) -> length (actual length of string)
  let message = String::from("Hello");
  println!("{}", message);
}

fn add(x: u32, y: u32) -> u32 {
  let sum = x + y;
  sum
}
