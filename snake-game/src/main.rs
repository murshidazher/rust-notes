fn main() {
  let mut message = String::from("Hello"); // immutable borrow
  let message_2: &mut String = &mut message; // mutable reference
  // mutable reference will change both the pointers
  // message_2 is not owner of data
  // message_2 is "borrowing" a reference to message
  // message    |ptr|cap|len|
  // message_2  |ptr|

  message_2.push_str(" World");

  println!("{}", message_2);
  println!("{}", message);
}

// message and message_2 going out of the scope
// message_2 is not dropped because it doesn't have ownership of what it refers to
// message is dropped
