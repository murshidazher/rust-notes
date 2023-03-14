struct Person {
  name: String,
  last_name: String,
  age: u32,
}

// Struct is similar to objects in other languages
// Used when we need to group multiple values together
fn main() {
  let person =  Person {
    name: "John".to_string(), // &str -> String
    last_name: "Doe".to_string(), // &str -> String
    age: 24,
  };

  println!("{} {} {}", person.name, person.last_name, person.age);
}
