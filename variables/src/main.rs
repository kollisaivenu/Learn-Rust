fn main() {
    let x = 5;
    println!("The value of x is:{}", x);
   //x = 6;
   let x: &str = "five";
   println!("The value of x is: {}", x);
    const CONST: i32 = 100000;

    // Data types

    // Compund Types
    let tup: (&str, i32) = ("Rust", 100_00);
    let (lang, number) = tup;

    let error_codes = [200, 404, 500];
    let not_found = error_codes[1];

    let byte = [0; 8];
    let sum = add(0, 8);

    // Control flow

    if sum < 5 {
        println!("sum is less than 5");
    } else if sum == 5{
        println!("sum is equal to 5");
    } else {
        println!("sum is greater than 5");
    }
    let mut counter:i32 = 0;
    let result: i32 = loop {
        counter += 1;

        if counter == 10{
            break counter;
        }
    };
    println!("Counter: {}", counter);

    let a = [10, 20, 40, 50, 60];

    for element in a.iter(){
        println!("The value is:{}", element);
    }

}

fn add(x: i32, y:i32) ->i32 {
    let sum = x+y;
    println!("Sum:{}", sum);
    return sum;
}
