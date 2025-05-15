pub fn add(left: u64, right: u64) -> u64 {
    left + right
}
struct Rectangle{
    width: u64,
    height: u64,
}
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
struct Guess{
    value: u32,
}
impl Guess{
    fn new(value: u32) -> Guess{
        if value < 1 || value > 100{
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }
        Guess{value}
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    #[test]
    fn test_add() {
        panic!("fail this test");
    }
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { width: 8, height: 7 };
        let smaller = Rectangle { width: 5, height: 1 };
        assert!(larger.can_hold(&smaller));
    }

    fn smaller_can_hold_larger() {
        let larger = Rectangle { width: 8, height: 7 };
        let smaller = Rectangle { width: 5, height: 1 };
        assert!(!smaller.can_hold(&larger));
    }
    #[test]
    #[should_panic(expected = "Guess value must be between 1 and 100")]
    fn greater_than_100() {
        Guess::new(200);
    }
}
