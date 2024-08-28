use std::{collections::HashMap, hash::Hash};

fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}
fn call<T: Fn()>(f: T) {
    let name = type_of(&f);
    println!("Start {}", name);
    f();
    println!("End {}", name);
}
fn main() {
    call(vectors);
    call(hashmap);
}

#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String)
}
fn show_addr(ip: &IpAddr) {
    println!("{:?}", ip);
}

trait IpAddrTrait {
    fn display(&self);
}
struct V4(String);
impl IpAddrTrait for V4 {
    fn display(&self) {
        println!("ipv4: {:?}", self.0);
    }
}
struct V6(String);
impl IpAddrTrait for V6 {
    fn display(&self) {
        println!("ipv6: {:?}", self.0);
    }
}
fn print_vec<T: std::fmt::Display>(v: &Vec<T>) {
    print!("[");
    for e in v {
        print!("{e} ");
    }
    println!("]");

}
#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}
impl Person {
    fn new(name: String, age: u32) -> Person {
        Person { name, age }
    }
}
fn vectors() {
    let v: Vec<i32> = Vec::new();
    let mut v = Vec::new();
    v.push(1);
    let v = vec![1, 2, 3];
    let third: &i32 = &v[2];
    println!("third: {}", third);
    match v.get(2) {
        Some(third) => println!("third: {}", third),
        None => println!("no third"),
    }

    for i in &v {
        println!("{i}");
    }
    let mut v = vec![1, 2, 3];
    for i in &mut v {
        *i += 10;
        println!("{i}");
    }
    let v = vec![
        IpAddr::V4("127.0.0.1".to_string()),
        IpAddr::V6("::1".to_string()),
    ];
    for ip in &v {
        show_addr(ip);
    }
    let v: Vec<Box<dyn IpAddrTrait>> = vec![
        Box::new(V4("127.0.0.1".to_string())),
        Box::new(V6("::1".to_string())),
    ];
    for ip in &v {
        ip.display();
    }
    let v = vec![0; 3];  // [0, 0, 0]
    let v_from = Vec::from([0, 0, 0]);
    assert_eq!(v, v_from);

    let mut v = Vec::with_capacity(10);
    v.extend([1, 2, 3]);
    // 3 10
    println!("Vector length: {}, capacity: {}", v.len(), v.capacity());
    // 3 103
    v.reserve(100); // 调整至【至少】100
    println!("Vector length: {}, capacity: {}", v.len(), v.capacity());
    // 3 3
    v.shrink_to_fit();  // 释放余量
    println!("Vector length: {}, capacity: {}", v.len(), v.capacity());

    let mut v = vec![1, 2];
    assert!(!v.is_empty());
    v.insert(2, 3);  // [1, 2] -> [1, 2, 3]
    assert_eq!(v.remove(1), 2);  // [1, 2, 3] -> [1, 3]
    assert_eq!(v.pop(), Some(3));  // [1, 3] -> [1]
    assert_eq!(v.pop(), Some(1));  // [1] -> []
    assert_eq!(v.pop(), None);  // [] -> []
    v.clear();  // [] -> []

    let mut v1 = [11, 22].to_vec();
    v.append(&mut v1);  // v: [] -> [11, 22]  v1: [11, 22] -> []
    assert_eq!(v1.pop(), None);  // [] -> []
    v.truncate(1);  // 截断到指定长度 [11, 22] -> [11]
    v.retain(|x| *x > 10);  // 保留满足条件的元素 [11] -> [11]
    assert_eq!([11].to_vec(), v);  //
    v.retain(|x| *x < 10);  // 保留满足条件的元素 [11] -> []
    assert_eq!([0; 0].to_vec(), v);

    let mut v = vec![11, 22 ,33 ,44, 55];
    // v: [11, 22, 33, 44, 55] -> [11, 55]
    // m: [22, 33, 44]
    let mut m: Vec<_> = v.drain(1..=3).collect();
    print_vec(&v);
    print_vec(&m);
    let v2 = m.split_off(1);
    // m: [22, 33, 44] -> [22]
    // v2: [33, 44]
    print_vec(&m);
    print_vec(&v2);
    let v = vec![11, 22, 33, 44, 55];
    let slice = &v[1..=3];
    assert_eq!(slice, &[22, 33, 44]);

    let mut vec = vec![1, 5, 10, 2, 15];
    vec.sort_unstable();
    assert_eq!(vec, vec![1, 2, 5, 10, 15]);
    let mut vec = vec![1.0, 5.6, 10.3, 2.0, 15_f32];
    vec.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
    assert_eq!(vec, vec![1.0, 2.0, 5.6, 10.3, 15_f32]);

    let mut people = vec![
        Person::new("Zoe".to_string(), 25),
        Person::new("Al".to_string(), 60),
        Person::new("John".to_string(), 1),
    ];
    people.sort_unstable_by(|a, b| b.age.cmp(&a.age));
    print!("{:?}", people);
}

fn hashmap() {
    let capacity = 12;
    // HashMap::new()
    let mut my_gems: HashMap<&str, i32> = HashMap::with_capacity(capacity);
    my_gems.insert("红宝石", 1);
    my_gems.insert("蓝宝石", 2);
    my_gems.insert("河边捡的误以为是宝石的破石头", 19);

    let teams_list = vec![
        ("中国队".to_string(), 100),
        ("美国队".to_string(), 10),
        ("日本队".to_string(), 50),
    ];
    let mut teams_map = HashMap::new();
    for team in &teams_list {
        teams_map.insert(&team.0, team.1);
    }
    println!("{:?}", teams_map);
    let teams_map: HashMap<_, _> = teams_list.into_iter().collect();
    println!("{:?}", teams_map);

    let name = String::from("sun");
    let age = 18;
    let mut handsome_boys = HashMap::new();
    println!("name: {}", name);
    handsome_boys.insert(name, age);
    // name moved into hashmap
    // println!("name: {}", name);
    println!("age: {}", age);

    let name = String::from("sun");
    let age = 18;
    let mut handsome_boys = HashMap::new();
    // reference name
    handsome_boys.insert(&name, age);
    std::mem::drop(name);
    // println!("boys: {:?}", handsome_boys);

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yello"), 50);
    let team_name = String::from("Blue");
    let score: Option<&i32> = scores.get(&team_name);
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    let mut scores = HashMap::new();
    scores.insert("Blue", 10);
    let old = scores.insert("Blue", 20);
    assert_eq!(old, Some(10));
    let new = scores.get("Blue");
    assert_eq!(new, Some(&20));

    let v = scores.entry("Yellow").or_insert(5);
    assert_eq!(*v, 5);
    let v = scores.entry("Yellow").or_insert(50);
    assert_eq!(*v, 5);

    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}",map);

    use std::hash::BuildHasherDefault;
    use twox_hash::XxHash64;
    let mut hash: HashMap<_, _, BuildHasherDefault<XxHash64>> = Default::default();
    hash.insert(42, "the answer");
    assert_eq!(hash.get(&42), Some(&"the answer"));
}
