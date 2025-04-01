use std::fs::File;
use std::io::{self, Read};
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}", value);
        }
        Guess { value }
    }
    pub fn value(&self) -> i32 {
        self.value
    }
}
fn main() {
    // panic!("crash and burn");
    let greeting_file_result = File::open("hello.txt");

    // let greet_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => panic!("There was a problem opening the file: {:?}", error),
    // };

    // let greet_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file{:?}", e),
    //         },
    //         other_error => panic!("Problem opening the file: {:?}", other_error),
    //     }
    // };

    let greeting_file = File::open("hello.txt").expect("hello.txt should exist");

    fn read_username_from_file() -> Result<String, io::Error> {
        let username_file_result = File::open("hello.txt");

        let mut username_file = match username_file_result {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        let mut username = String::new();

        match username_file.read_to_string(&mut username) {
            Ok(_) => Ok(username),
            Err(e) => Err(e),
        }
    }
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
