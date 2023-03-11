fn main() {
  let is_it_fun = false;
  // i32 -> signed integer of 32bits -> 2^31 – 1 (max number)
  // signed can hold positive and negative values
  let num = -10;

  // u32 -> 2^32 - 1
  // u8 -> unsigned integer of 8bits
  // 2^8 - 1 = 255
  let small_num: u8 = 255;

  // -2^7 -> 2^7 - 1
  // -128 -> 127
  let small_num_2: i8 = 127;

  // architecture related types
  // operating system related type
  // isize or usize depends on arch will hold either 32bit or 64bit
  let sys_num: isize = -10;
  let sys_num_2: usize = 10;
}
