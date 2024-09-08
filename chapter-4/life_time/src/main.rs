#![allow(unused)]

fn main() {
    use std::collections::HashMap;
    use std::hash::Hash;
    fn get_result<'m, K, V>(map: &'m mut HashMap<K, V>, key: K) -> &'m mut V
    where
        K: Clone + Eq + Hash,
        V: Default,
    {
        if let None = map.get_mut(&key) {
            map.insert(key.clone(), V::default());
        }
        map.get_mut(&key).unwrap()
    }

    fn f<'a, T>(x: *const T) -> &'a T {
        unsafe {
            &*x
        }
    }
    fn fn_elision(x: &i32) -> &i32 { x }
    // let closure_elision = |x: &i32| -> &i32 { x };
    let mut p = Point { x: 0, y : 0 };
    let r = &mut p;
    let rr: &Point = &*r;
    println!("{:?}", rr);
    r.move_to(10, 10);
    println!("{:?}", r);

    let (p, len) = get_memory_location();
    let msg = get_str_at_location(p, len);
    println!("The {} bytes at 0x:{:X} stored: {}", len, p, msg);

    let i = 5;
    print_it(&i);
    print_it2(i);
}

struct DoubleRef<'a, 'b: 'a, T> {
    r: &'a T,
    s: &'b T
}

struct Ref <'a, T> {
    r: &'a T
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}
impl<'a: 'b, 'b> ImportantExcerpt<'a> {
    fn announce_and_return_part(&'a self, announcement: &'b str) -> &'b str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}
impl Point {
    fn move_to(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }
}

use std::slice::from_raw_parts;
use std::str::from_utf8_unchecked;

fn get_memory_location() -> (usize, usize) {
    let string = "h, w";
    let p = string.as_ptr() as usize;
    let length = string.len();
    (p, length)
}

fn get_str_at_location(p: usize, len: usize) -> &'static str {
    unsafe { from_utf8_unchecked((from_raw_parts(p as *const u8, len))) }
}

use std::fmt::Debug;

fn print_it<T: Debug + 'static>(input: &T) {
    println!("static value passed in is: {:?}", input);
}
fn print_it2(input: impl Debug + 'static) {
    println!("static value passed in is: {:?}", input);
}
