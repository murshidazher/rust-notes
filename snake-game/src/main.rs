
fn main() {
  let mut message = String::from("Hello");
  let name = "Filip";

  message.push_str(" World");
  // name.push_str(" Jerga");
  // String is a stack pointer while &str is a pointer to read-only memory in heap
}
