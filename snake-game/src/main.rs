// De-referencing
// * means de-referencing
// only by de-referencing a number can you get the value
fn main() {
  let mut message = String::from("Hello"); // immutable borrow
  let message_2 = &mut message;

  (*message_2).push_str(" World");
  // same as
  // message_2.push_str(" World");

  println!("{}", message_2);

  let a = 10;
  let b = &a;
  let c = &b;

  println!("{}", a == **c);

  let mut c = &b;
  let d = b;

  let e = &&100;

  c = e;

  // we can print the pointer address instead of the value
  println!("e: {:p}", e);
  println!("e: {:p}", *e);
  println!("c: {:p}", c);
  println!("c: {:p}", *c);
}
