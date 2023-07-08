// use modularity::Person;
// use modularity::Animal;
// use modularity::log_info;
// use modularity::log_info_2;

// short hand to import all the imports in one-line
// use modularity::*;
// package::module::{imports};
use modularity::learning_rust::{log_info, log_info_2, Animal, Person};

fn main() {
    let mut person = Person::new();
    let animal = Animal(String::from("dog"));

    person.change_age(38);

    log_info(person);
    log_info_2(&animal);
}
