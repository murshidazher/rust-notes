use modularity::Person;
use modularity::Animal;
use modularity::log_info;
use modularity::log_info_2;

fn main() {
    let mut person = Person::new();
    let animal = Animal(String::from("dog"));

    person.change_age(38);

    log_info(person);
    log_info_2(&animal);
}
