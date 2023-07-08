// The reason why we are not using mod in the main fn is that it is imported automatically

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
