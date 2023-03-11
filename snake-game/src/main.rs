fn main() {
  let mut message = String::from("Hello"); // immutable borrow
  let message_3 = &message;
  println!("{}", message_3);
  let message_2 = &mut message;
  // let message_4 = &message;
  // we can't do this too since the is a mutate call above

  // rust compiler doesn't allow the immutable to be print before mutable
  // error -> cannot borrow `message` as immutable because it is also borrowed as mutable
  // println!("{}", message);
  // it's because this function can be async and can update the value of message while the program is running. This is unpredictable variability.
  // unpredictable_mutate(message_2);

  unpredictable_mutate(message_2);
  println!("{}", message);
}

fn unpredictable_mutate(val: &mut String) {
  val.push_str("_unpredictable");
}
