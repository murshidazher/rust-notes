fn main() {
  let message = String::from("Hello"); // message coming into scope
  print_message(message); // message is moved to the print_message function
  // message is no longer valid

  // error: value borrowed here after move
  // this is because the value is moved to c and then removed
  // after the popping of print_message's stack frame
  // println!("{}", message);
}

fn print_message(a: String) { // a comes into the scope
  println!("{}", a);
  let c = a; // c is coming into scope and a is moved
  // a is no longer valid

  // this will also throw an error since a is moved
  // println!("{}", a);
}

// a is going out of scope, but nothing more will happen since it was moved
// c is going out of scope and 'drop' is called which clears the underlying memory from the heap
