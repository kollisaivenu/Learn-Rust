use std::collections::HashMap;
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let mut v1: Vec<i32> = Vec::new();
    let v2 = vec![1,2,3,4];

    v1.push(5);
    v1.push(6);
    v1.push(7);

    let third: &i32 = &v1[2];
    println!("The third element is {}", third);

    let third: Option<&i32> = v1.get(2);
    match third {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
    for i in &v2 {
        println!("{}", i);
    }

    for i in &mut v1 {
        *i += 5;
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    let mut s = String::new();
    let data = "initial contents";
    let s = data.to_string();

    let mut s2 = String::from("foo");
    s2.push_str("bar");

    let s4 = String::from("tic");
    let s5 = String::from("tac");
    let s6 = String::from("toe");

    //let s7 = s4 + "-" + &s5 + "-" + &s6;

    let s7 = format!("{s4}-{s5}-{s6}");

    let s1 = String::from("hello");
    //let h = s1[0];

    for c in "Зд".chars() {
        println!("{c}");
    }

    for c in "Зд".bytes() {
        println!("{c}");
    }
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    scores.entry(String::from("Yellow")).or_insert(50);

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");
}
