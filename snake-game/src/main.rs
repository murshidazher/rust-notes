fn main() {
  let message = "Hello World";
  let message_2 = print_welcome(message);
  // println is a macro and they end with an ! (exclamation)
  println!("{}", message_2);
}

fn print_welcome(text: &str) -> &str {
  println!("{}", text);
  let new_message = "Hi There";
  // return new_message;
  new_message // we don't need to explicitly write the return keyword to return. A value ending without semi-colon would be returned
}
