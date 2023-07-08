#[derive(Debug)]
enum PersonId {
  Passport(u32),
  IdentityCard(u32, u32, u32),
}

struct Person {
    name: String, // fields
    last_name: String,
    age: u32,
    id: PersonId,
}

impl Person {
    fn new() -> Person {
        Person {
            name: "Default".to_string(),
            last_name: "Default".to_string(),
            age: 0,
            id: PersonId::IdentityCard(540, 320, 100)
        }
    }

    fn from(name: String, last_name: String, age: u32, id: PersonId) -> Person {
        Person {
            name,
            last_name,
            age,
            id
        }
    }

    fn change_age(&mut self, new_age: u32) {
       self.age = new_age;
    }

    fn display_info(&self) {
        println!("{} {} {} {:?}", self.name, self.last_name, self.age, self.id)
    }
}

fn main() {
    let mut person = Person::new();
    let person_2 = Person::from(
        String::from("John"),
        String::from("Snow"),
        35,
        PersonId::Passport(123172371)
    );

    person.change_age(38);
    person.display_info();

    // PersonId::IdentityCard(540, 320, 100)
    check_person_id(person.id);
    check_person_id(person_2.id);
}

fn check_person_id(id: PersonId) {

    // pattern matching with if let
    if let PersonId::Passport(num) = id {
        println!("It matching Passport {}", num);
    } else {
        println!("It doesn't match!");
    }

    // It should match the identity card pattern or passport pattern
    let result = match id {
        PersonId::IdentityCard(_x, y, _z) => {
            y
        },
        PersonId::Passport(val) => {
            val
        }
    };

    println!("Result: {}", result);
}
