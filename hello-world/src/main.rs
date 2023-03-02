// entry point -> main
// println is a macro and they end with an ! (exclamation)
// compiled using LLVM to binary
fn main() {
  let message = "Hello World"; // this is an immutable variable
  println!("Some text: {}", message);

  // mutable variables
  let mut age: i16 = 10;
  println!("{}", age);
  age = 30;
  println!("{}", age);

  // mutable string
  let mut greetings  = "Hello World";
  println!("Some text: {}", greetings);
  greetings = "Hello there";
  println!("Some text: {}", greetings);

  // in rust, you can re-initialize the same variable name
  // this concept is called 'Shadowing', because the second initialization is shadowing the first initialization
  let message = "Hello there";
  println!("Some text: {}", message);
}
