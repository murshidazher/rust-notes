mod another_lib_v2; // name of the file, 'mod' is to import the file

use another_lib_v2::another_mod; // 'use' keyword will only alias the module path to another_mod

fn outsider() {
    another_mod::another_fn();
    // crate::another_mod::another_fn();
    println!("outsider fn!");
}

// creating a new module using mod
pub mod learning_rust {
    use std::fmt;

    mod top_level {
        pub fn hi_there() {
            println!("hi there!");
        }

        pub mod low_level {
            pub fn hello_world() {
                println!("hello world!");
            }
        }
    }

    pub trait Log {
        fn display_info(&self);
        fn alert_something(&self) {
            println!("Default implementation!")
        }
    }

    pub enum PersonId {
        Passport(u32),
        IdentityCard(u32, u32, u32),
    }

    impl fmt::Display for PersonId {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                PersonId::Passport(x) => {
                    write!(f, "MY PASSPORT : {}", x)
                }
                PersonId::IdentityCard(x, y, z) => {
                    write!(f, "This is my ID: ----> {} {} {} <-----", x, y, z)
                }
            }
        }
    }

    pub struct Person {
        name: String, // fields
        last_name: String,
        age: u32,
        pub id: PersonId,
    }

    pub struct Animal(pub String);

    impl Log for Animal {
        fn display_info(&self) {
            println!("{}", self.0)
        }

        fn alert_something(&self) {
            println!("Animal implementation!")
        }
    }

    impl Log for Person {
        fn display_info(&self) {
            // absolute path
            // crate points to -> src/lib.rs and src/main.rs
            crate::learning_rust::top_level::hi_there();
            crate::learning_rust::top_level::low_level::hello_world();

            // relative path
            top_level::hi_there();
            top_level::low_level::hello_world();

            // ../
            super::another_mod::another_fn();
            // or absolute import
            crate::another_mod::another_fn();

            println!("{} {} {} {}", self.name, self.last_name, self.age, self.id)
        }
    }

    impl Person {
        pub fn new() -> Person {
            Person {
                name: "Default".to_string(),
                last_name: "Default".to_string(),
                age: 0,
                // id: PersonId::IdentityCard(540, 320, 100),
                id: PersonId::Passport(312398),
            }
        }

        fn from(name: String, last_name: String, age: u32, id: PersonId) -> Person {
            Person {
                name,
                last_name,
                age,
                id,
            }
        }

        pub fn change_age(&mut self, new_age: u32) {
            self.age = new_age;
        }
    }

    // impl makes the compiler determine type at the compile time
    // it will create multiple versions of the function, depending on
    // how many types Log trait implements (Person, Animal)
    pub fn log_info(val: impl Log) {
        val.alert_something();
    }

    // dyn is short for dynamic, and says that function should perform dynamic dispatch
    // decision of exactly which function to call `at the runtime`
    // smaller build size because it will only have one version of the Log implementation
    pub fn log_info_2(val: &dyn Log) {
        val.alert_something();
    }
}
