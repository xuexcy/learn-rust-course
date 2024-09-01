use std::fmt;
#[derive(Debug)]
struct Person {
    name: String,
    age: u8
}
impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "gogo, name: {}, age: {}",
            self.name,
            self.age
        )
    }
}

struct Array(Vec<i32>);
impl fmt::Display for Array {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "array is {:?}", self.0)
    }

}

fn main() {
    println!("Hello, world!");
    println!("{:?}", (3, 4));
    println!("{:04}", 42);  // leading zeros

    let s = "hello";
    println!("{}, world", s);
    let s1: String = format!("{}, world", s);
    eprintln!("error: output to std err {}", s1);  // 标准错误输出

    let v = vec![1, 2, 3];
    let v2 = Array(v.clone());
    let p = Person {
        name: "sunface".to_string(),
        age: 28,
    };
    println!("{} {}", v2, p);
    println!("{:?} {:?}", v, p);
    println!("{:#?} {:#?}", v, p);
    println!("{}{}", 1, 2);
    println!("{1}{0}", 1, 2); // 21
    // A, this is B. B, this A
    println!("{0}, this {1}. {1}, this is {0}", "A", "B");
    println!("{}, my name is {name}", "xue", name="chengyun");

    let v = 3.141592;
    println!("{:.2}", v);  // Display 3.14
    println!("{:.2?}", v);  // Debug 3.14
    println!("{:.3?}", v);  // Debug 3.141

    println!("Hello, {:5}!", "x");  // Hello x    !
    println!("Hello, {:<5}!", "x");
    println!("Hello, {:>5}!", "x");
    println!("Hello, {:^5}!", "x");

    println!("{:#b}!", 27);  // 0b11011!
    println!("{:#o}!", 27);  // 0o33!
    println!("{:2e}", 10000000);  // 1e7
    println!("{:2e}", 10000000);  // 1E7

    let v = vec![1, 2, 3];
    println!("{:p}", v.as_ptr());
}
