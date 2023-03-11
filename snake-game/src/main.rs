fn main() {
  let message = String::from("Hello");
  let message_2 =extend_message(message);

  println!("{}", message_2);
}

// mark it as mutable string to append the world
fn extend_message(mut a: String) -> String {
  a.push_str(" World"); // we need to show a hello world message
  a
}

