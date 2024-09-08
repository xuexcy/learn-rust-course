use std::thread;
use std::time::Duration;

fn type_of<T>(_: T) -> String {
    format!("{}", std::any::type_name::<T>())
}
fn call<T: Fn()>(f: &T) {
    let name = type_of(f);
    println!("Start {}", name);
    f();
    println!("End {}", name);
}
fn main() {
    call(&closure);
    call(&iterator);
}

fn closure() {
    let x = 1;
    let sum = |y| x + y;
    assert_eq!(3, sum(2));
    let intensity = 10;
    let random_number = 7;
    workout(intensity, random_number);

    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });
    handle.join().unwrap();

    let mut s = String::new();
    let mut update_string = |str| s.push_str(str);
    update_string("hello");
    println!("{:?}", s);
    let update_string = |str| s.push_str(str);
    exec(update_string);
    println!("{:?}", s);
}
fn muuuuu(intensity: u32) -> u32 {
    println!("m...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn workout(intensity: u32, random_number: u32) {
    let action = muuuuu;
    if intensity < 25 {
        println!("push up {}", action(intensity));
        println!("push up again {}", action(intensity));
    } else if random_number == 3 {
        println!("relax");
    } else {
        println!("run {}", action(intensity))
    }
}

struct Cacher<T, E>
where
    T: Fn(E) -> E,
    E: Copy,
{
    query: T,
    value: Option<E>,
}

impl<T, E> Cacher<T, E>
where
    T: Fn(E) -> E,
    E: Copy
{
    fn new(query: T) -> Cacher<T, E> {
        Cacher {
            query,
            value: None,
        }
    }
    fn value(&mut self, arg: E) -> E{
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.query)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn fn_once<F>(func: F)
where
    F: FnOnce(usize) -> bool + Copy,
{
    println!("{}", func(3));
    println!("{}", func(4));
}

fn exec<'a, F: FnMut(&'a str)>(mut f: F) {
    f("Hello")
}

fn iterator() {
    let arr = [1, 2, 3];
    for v in arr.iter() {
        println!("{}", v);
    }
    let values = vec![1, 2, 3];
    for v in values.iter() {
        println!("{}", v);
    }
    {
        let result = match IntoIterator::into_iter(values) {
            mut iter => loop {
                match iter.next() {
                    Some(x) => { println!("{}", x); },
                    None => break,
                }
            },
        };
    }
    let values = vec![1, 2, 3];
    let mut iter = IntoIterator::into_iter(values);
    loop {
        match iter.next() {
            Some(x) => { println!("{}", x); },
            None => break,
        }
    }

    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();
    assert_eq!(total, 6);

    use std::collections::HashMap;
    let names = ["s", "sun"];
    let ages = [18, 18];
    let f: HashMap<_, _> = names.into_iter().zip(ages.into_iter()).collect();
    println!("{:?}", f);

    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);

    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();
    // zip [(1, 2), (2, 3), (3, 4), (4, 5)]
    // map [1 * 2, 2 * 3, 3 * 4, 4 * 5]
    // filter [2 * 3, 3 * 4]
    assert_eq!(18, sum);

    let v = vec![1u64, 2, 3, 4, 5, 6];
    for (i, v) in v.iter().enumerate() {
        println!("{}: {}", i, v);
    }

    let val = v.iter()
        .enumerate()
        .filter(|&(idx, _)| idx % 2 == 0)
        .map(|(_, val)| val)
        .fold(0u64, |sum, acm| sum + acm);
    // 1 + 3 + 5
    assert_eq!(9, val);
}

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}
impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }

}
