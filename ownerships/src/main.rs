fn main() {
    {
        // s is not a valid here, it's not yet declared
        let s: String = String::from("hello"); // s is valid from this forward
        // do stuff with s
    }// this scope is now over, and s is no longer valid


    let s1 = String::from("hello");

    // won't work
    // let s2 = s1;
    //println!("{}, world!", s1);
    let s3 = s1.clone();


    println!("{}, world!", s1);

    // Copy trait
    let mut x = 5;
    let y = x;

    let x: i32 = 5;
    let y: i32 = x;

    let s1: String = String::from("hello");
    // let s2: String = s1; // move
    let s2: String = s1.clone();

    println!("{}, world", s1);

    let s: String = String::from("hello");
    takes_ownership(s);
    // won't work
    // println!(s);

    let s0: String = String::from("hello");
    let (s00, len) = calculate_length(s0);

    let s4: String = String::from("hello");
    let l = calculate_length_with_reference(&s4);

    let mut s3: String = String::from("hello");
    change(&mut s3);

    //Can have only 1 mutable reference
    let mut s = String::from("hello");

    //let r1 = &mut s;
    //let r2 = &mut s;

    // Can have any number of immutable reference.
    let r1 = &s;
    let r2 = &s;

    println!("{}, {}", r1, r2);

    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    let word = first_word(&s);
    // won't work as reference lives longer since word contains immutable reference hence a mutable reference cannot be created.
    //s.clear(); // error!

    println!("the first word is: {word}");
}

fn takes_ownership(some_string: String){
    println!("{}", some_string);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}

fn calculate_length_with_reference(s: &String) -> usize {
    let length = s.len(); // len() returns the length of a String

    length
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

