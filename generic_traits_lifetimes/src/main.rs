struct Point<T>{
    x: T,
    y: T,
}
impl<T> Point<T>{
    fn x(&self) -> &T{
        &self.x
    }
}
struct PointXY<X1, Y1> {
    x: X1,
    y: Y1,
}
impl<X1, Y1> PointXY<X1, Y1> {
    fn mixup<X2, Y2>(self, other: PointXY<X2, Y2>) -> PointXY<X1, Y2> {
        PointXY {
            x: self.x,
            y: other.y,
        }
    }
}

struct Point2<T, U>{
    x: T,
    y: U,
}

struct Tweet {
    author: String,
    content: String,
    reply: bool,
    retweet: bool,
}

struct NewsArticle {
    author: String,
    content: String,
    location: String,
    headline: String,
}

trait Summary {
    fn summarize(&self) ->String{
        String::from("Default Implementation")
    }
}

impl Summary for NewsArticle{
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}
fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    // let result = largest(&number_list);
    // println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];

    // let result = largest(&char_list);
    // println!("The largest char is {result}");
    let integer = Point{x: 1, y: 2};
    let float = Point{x: 1.0, y: 2.0};
    let one_integer_one_float = Point2{x:1, y:2.0};

    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());

    let toi = NewsArticle{
        author: String::from("abc"),
        content: String::from("cdf"),
        location: String::from("efg"),
        headline: String::from("bdf"),
    };
    println!("{}", toi.summarize());

    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {result}");
    }

}


fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// fn largest<T>(list: &[T]) -> &T {
//     let mut largest = &list[0];
//
//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }