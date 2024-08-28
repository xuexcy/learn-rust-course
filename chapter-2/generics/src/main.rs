use std::fmt::Debug;
use std::fmt::Display;
use std::ops::Add;
use std::string::ParseError;

fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}
fn call<T>(f: T)
where
    T: Fn()
{
    println!("Start {}", type_of(&f));
    f();
    println!("End {}", type_of(&f));
}
fn main() {
    call(generic);
    call(the_trait);
    call(trait_object);
    call(more_trait);
}

fn add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    a + b
}
fn largest<T: std::cmp::PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}
struct Point<T> {
    x: T,
    y: T,
}
impl <T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
struct PointEnableUnsameType<T, U> {
    x: T,
    y: U,
}
impl <T, U> PointEnableUnsameType<T, U> {
    fn mixup<V, W>(self, other: PointEnableUnsameType<V, W>) -> PointEnableUnsameType<T, W> {
        PointEnableUnsameType {
            x: self.x,
            y: other.y,
        }
    }
}
fn display_array(arr: [i32; 3]) {
    println!("{:?}", arr);
}
// 数组切片
fn display_array_v2<T: std::fmt::Debug>(arr: &[T]) {
    println!("{:?}", arr);
}
// const 泛型
fn display_array_v3<T: std::fmt::Debug, const N: usize>(arr: [T; N]) {
    println!("{:?}", arr);
}
// 在 nightly 版本下使用
// const 泛型表达式
// fn something<T>(val: T)
// where
//     Assert<{ core::mem::size_of::<T>() < 768 }>: IsTrue,
// {
//     //
// }
fn generic() {
    println!("add i8: {}", add(2i8, 3i8));
    println!("add i32: {}", add(20, 30));
    println!("add f64: {}", add(1.23, 1.23));
    let number_list = vec![34, 50, 25, 100, 6];
    let result = largest(&number_list);
    println!("The largest number is {}", result);
    let char_list = vec!['y', 'a'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 5.0, y: 10.0 };
    let p = PointEnableUnsameType { x: 1, y: 1.1 };
    enum Option<T> {
        Some(T),
        None,

    }
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }

    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());

    let p1 = PointEnableUnsameType { x: 5, y: 10.4 };
    let p2 = PointEnableUnsameType { x: "hello", y: 'c'};
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
    let arr: [i32; 3] = [1, 2, 3];
    display_array(arr);
    display_array_v2(&arr);
    display_array_v3(arr);
    let arr: [i32; 2] = [1, 2];
    // display_array(arr);
    display_array_v2(&arr);
    display_array_v3(arr);


    let t = vec![1, 2];
    let res = t
        .iter()
        .map(|e| format!("{}", e))
        .collect::<Vec<_>>()
        .join(",");

}

pub trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        format!("(Read more from {}...", self.summarize_author())
    }
}
pub struct Post {
    pub title: String,
    pub author: String,
    pub content: String,
}
impl Summary for Post {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
    fn summarize(&self) -> String {
        format!("文章{}作者是{}", self.title, self.author)
    }
}
pub struct Weibo {
    pub username: String,
    pub content: String
}
impl Summary for Weibo {
    fn summarize_author(&self) -> String {
        format!("@{}",self.username)
    }
}

pub fn notify(item: &impl Summary) {
    println!("Breaking new! {}", item.summarize());
}
pub fn notify_v2<T: Summary>(item: &T) {
    println!("Breaking new! {}", item.summarize());
}
// item1 和 item2 类型可以不同
pub fn notify_v3(item1: &impl Summary, item2: &impl Summary) {}
// item1 和 item2 类型必须相同
pub fn notify_v4<T: Summary>(item1: &T, item2: &T) {}
// 多重约束
pub fn notify_v5<T: Summary + Display>(item: &T) {}
pub fn notify_v6(item: &(impl Summary + Display)) {}
// where 约束
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 { 1 }
fn some_function_v2<T, U>(t: &T, u: &U) -> i32
where T: Display + Clone,
    U: Clone + Debug
{ 1 }

struct Pair<T> {
    x: T,
    y: T,
}
impl <T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}
impl <T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x > self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

fn returns_summarizable() -> impl Summary {
    Weibo {
        username: String::from("s"),
        content: String::from("new bee")
    }
    // if switch {
    //     Weibo {
    //         username: String::from("s"),
    //         content: String::from("new bee")
    //     }
    // } else {
    //     Post {
    //         title: String::from(
    //             "Penguins win the Stanley Cup Championship!",
    //         ),
    //         author: String::from("Iceburgh"),
    //         content: String::from(
    //             "The Pittsburgh Penguins once again are the best \
    //              hockey team in the NHL.",
    //         ),
    //     }
    // }
}

#[derive(Debug)]
struct PointV2<T: Add<T, Output = T>> {
    x: T,
    y: T,
}
impl <T: Add<T, Output = T>> Add for PointV2<T> {
    type Output = PointV2<T>;
    fn add(self, rhs: Self) -> Self::Output {
        PointV2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[derive(Debug, PartialEq)]
enum FileState {
    Open,
    Closed,
}
#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
    state: FileState,
}
impl Display for FileState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            FileState::Open => write!(f, "OPEN"),
            FileState::Closed => write!(f, "Closed"),
        }
    }
}
impl Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{} ({})>", self.name, self.state)
    }
}
impl File {
    fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
            state: FileState::Closed,
        }
    }
}
fn the_trait() {
    let post = Post {title: "a".to_string(), author: "b".to_string(), content: "c".to_string()};
    let weibo = Weibo { username: "d".to_string(), content: "e".to_string()};
    println!("{}", post.summarize());
    println!("{}", weibo.summarize());
    notify(&weibo);
    notify(&weibo);

    let p1 = PointV2 { x: 1.1_f32, y: 1.1_f32 };
    let p2 = PointV2 { x: 1.1_f32, y: 1.1_f32 };
    println!("{:?}", add(p1, p2));
    let f6 = File::new("f6.txt");
    println!("{:?}", f6);
    println!("{}", f6);
}

pub trait Draw {
    fn draw(&self)-> String;
}
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}
impl Draw for Button {
    fn draw(&self) -> String {
        format!(
            "Button: width[{}], height[{}], label[{}]",
            self.width, self.height, self.label)
    }
}
struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}
impl Draw for SelectBox {
    fn draw(&self) -> String {
        format!(
            "SelectBox: width[{}], height[{}], options[{}]",
            self.width, self.height, self.options.join(", "))
    }
}
impl Draw for u8 {
    fn draw(&self) -> String {
        format!("u8: {}", *self)
    }
}
impl Draw for f64 {
    fn draw(&self)-> String {
        format!("f64: {}", *self)
    }
}
fn draw1(x: Box<dyn Draw>) {
    x.draw();
}
fn draw2(x: &dyn Draw) {
    x.draw();
}
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}
impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            println!("{}", component.draw());
        }
    }
}
fn trait_object() {
    let x = 1.1_f64;
    let y = 8_u8;
    draw1(Box::new(x));
    draw1(Box::new(y));
    draw2(&x);
    draw2(&y);
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ]
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            })
        ]
    };
    screen.run();
}
pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
struct Counter;
impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        Some(1)
    }
}

trait Container {
    type A;
    type B;
    fn contains(&self, a: &Self::A, b: &Self::B) -> bool;

}
trait MyAdd<RHS=Self> {
    type Output;
    fn add(self, rhs: RHS) -> Self::Output;
}
struct Millimeters(u32);
struct Meters(u32);
impl Add<Meters> for Millimeters {
    type Output = Millimeters;
    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}
trait Pilot {
    fn fly(&self);
}
trait Wizard {
    fn fly(&self);
}
struct Human;
impl Pilot for Human {
    fn fly(&self) {
        println!("fly as pilot");
    }
}
impl Wizard for Human {
    fn fly(&self) {
        println!("fly as wizard");
    }
}
impl Human {
    fn fly(&self) {
        println!("fly as human");
    }

}
trait Animal {
    fn baby_name() -> String;
}
struct Dog;
impl Dog {
    fn baby_name() -> String {
        String::from("spot")
    }
}
impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

trait OutlinePrint: Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}
impl<T> Display for Point<T>
where
    T: Display {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
impl<T> OutlinePrint for Point<T>
where
    T: Display
{
}
fn more_trait() {
    let mut c = Counter;
    println!("next: {:?}", c.next());
    let person = Human;
    person.fly();
    Pilot::fly(&person);
    Wizard::fly(&person);
    println!("dog name: {}", Dog::baby_name());
    println!("dog name: {}", <Dog as Animal>::baby_name());
}
