fn main() {
  // types of way to represent numbers
  let custom_num = 98_000; // 98000
  let hex_num = 0xfa; // 250
  let bin_num = 0b0010_1011; // 43

  let byte_num = b'A'; // Ox41 -> 65 https://www.utf8-chartable.de/

  println!("{}", custom_num);
  println!("{}", hex_num);
  println!("{}", bin_num);
  println!("{}", byte_num);
}
