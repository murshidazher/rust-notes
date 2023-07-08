// use modularity::Person;
// use modularity::Animal;
// use modularity::log_info;
// use modularity::log_info_2;

// short hand to import all the imports in one-line
// use modularity::*;
// package::module::{imports};
use modularity::learning_rust::{Log, Person};

fn main() {
    let mut person = Person::new();
    person.display_info();
}
