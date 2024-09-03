#![feature(test)]
extern crate test;

fn internal_adder(left: i32, right: i32) -> i32 {
    left + right
}
pub fn add_two(i: i32) -> i32 {
    internal_adder(i, 2)
}
fn fibonacci(number: u64) -> u64 {
    let mut last: u64 = 1;
    let mut current: u64 = 0;
    let mut buffer: u64;
    let mut position: u64 = 1;
    return loop {
        if position == number {
            break current;
        }
        buffer = last;
        last = current;
        current = buffer + current;
        position += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use test::Bencher;

    #[test]
    fn it_works() {
        let result= internal_adder(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_then_100() {
        Guess::new(200);
    }

    #[test]
    fn it_works_too() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("2 + 2 != 4"))
        }
    }

    #[test]
    fn this_test_will_pass() {
        let value = prints_and_return_10(4);
        assert_eq!(10, value);
    }
    #[test]
    #[should_panic]
    fn this_test_will_fail() {
        let value = prints_and_return_10(8);
        assert_eq!(5, value);
    }
    #[test]
    fn test_fibonacci() {
        assert_eq!(fibonacci(1), 0);
        assert_eq!(fibonacci(2), 1);
        assert_eq!(fibonacci(12), 89);
        assert_eq!(fibonacci(30), 514229);
    }
    #[bench]
    fn bench_add_two(b: &mut Bencher) {
        b.iter(|| add_two(2) );
    }
    #[bench]
    fn bench_u64(b: &mut Bencher) {
        b.iter(|| {
            for i in 1..50 {
                fibonacci(i);
            }
        })
    }
}
fn prints_and_return_10(a: i32) -> i32 {
    println!("i got the value {}", a);
    10
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be greater than or equal to 1, got {}", value);
        } else if value > 100 {
            panic!("Guess value must be less than or equal to 100, got {}", value);
        }
        Guess { value }
    }
}

