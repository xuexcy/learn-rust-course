use std::convert::TryInto;
use std::fmt;
use std::ops::Add;
use std::sync::Arc;
use std::convert::TryFrom;
use std::convert::TryInto;

fn main() {
    println!("Hello, world!");
    let a = i8::MAX;
    println!("{}", a);

    let mut values: [i32; 2] = [1, 2];
    let p1: *mut i32 = values.as_mut_ptr();
    let first_address = p1 as usize;
    let second_address = first_address + std::mem::size_of::<i32>();
    let p2 = second_address as *mut i32;
    unsafe {
        *p2 += 1;
    }
    assert_eq!(values[1], 3);

    let a: u8 = 10;
    let b: u16 = 1500;
    let b_: u8 = match b.try_into() {
        Ok(b1) => b1,
        Err(e) => {
            println!("{:?}", e.to_string());
            0
        }
    };
    let t: &mut i32 = &mut 0;
    // foo(t);
    let p = fooo as *const ();
    let function = unsafe {
        std::mem::transmute::<*const(), fn() -> i32>(p)
    };
    assert_eq!(function(), 0);
    let w = Wrapper(vec!["Hello".to_string(), "w".to_string()]);
    println!("w = {}", w);

    let d = calculate_distance(Meters(10), Meters(20));
    println!("{}", d);

    // Box 接受固定大小
    // str 没有 Sized, 所以 Box 不接受 str
    // no: str -> Sized -> Box<str>
    // let s1: Box<str> = Box::new("h" as str);

    // str 实现了 Trait(From/Into), 编译器只需要让Box知道Trait能调用哪些方法就可以
    // yes: str -> Trait(From/Into) -> Box<Trait(From/Into)> -> Box<str>
    // 最后一步由`s1: Box<str>`确定
    let s1: Box<str> = "h".into();
    // no: 缺了最后一步 type annotation
    // let s1 = "h".into();

    let x = MyEnum::C as i32;
    match x.try_into() {
        Ok(MyEnum::A) => println!("a"),
        Ok(MyEnum::B) => println!("b"),
        Ok(MyEnum::C) => println!("c"),
        _ => eprintln!("convert error"),
    }

    let x = MyEnum::C;
    let y = x as i32;
    let z: MyEnum = unsafe { std::mem::transmute(y) };

}

struct R<'a>(&'a i32);
unsafe fn extend_lifetime<'b>(r: R<'b>) -> R<'static> {
    std::mem::transmute::<R<'b>, R<'static>>(r)
}
unsafe fn shorten_invariant_lifetime<'b, 'c>(r: &'b mut R<'static>) -> &'b mut R<'c> {
    std::mem::transmute::<&'b mut R<'static>, &'b mut R<'c>>(r)
}
struct Foo {
    x: u32,
    y: u16,
}

struct Bar {
    a: u32,
    b: u16,
}

fn reinterpret(foo: Foo) -> Bar {
    let Foo { x, y } = foo;
    Bar { a: x, b: y }
}

trait Trait {}
fn foo<X: Trait>(t: X) {}
impl<'a> Trait for &'a i32 {}

#[derive(Clone)]
struct Container<T>(Arc<T>);

fn clone_containers<T>(foo: &Container<i32>, bar: &Container<T>) {
    let foo_cloned = foo.clone();
    let bar_cloned = bar.clone();
}

fn fooo() -> i32 {
    0
}

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self.0.join(","))
    }
}

struct Meters(u32);
impl fmt::Display for Meters {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "distance {} meters", self.0)
    }
}
impl Add for Meters {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

fn calculate_distance(d1: Meters, d2: Meters) -> Meters {
    d1 + d2
}

fn takes_long_type(f: Box<dyn Fn() + Send + 'static>) {}
fn returns_long_type() -> Box<dyn Fn() + Send + 'static> {
    Box::new(|| ())
}

trait MyThing {}
fn foobar_1(thing: &dyn MyThing) {}
fn foobar_2(thing: Box<dyn MyThing>) {}
// fn foobar_3(thing: MyThing) {}

fn generic<T: ?Sized>(t: &T) {}

#[derive(FromPrimitive)]
#[repr(i32)]
enum MyEnum {
    A = 1,
    B,
    C,
}

impl TryFrom<i32> for MyEnum {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            x if x == MyEnum::A as i32 => Ok(MyEnum::A),
            x if x == MyEnum::B as i32 => Ok(MyEnum::B),
            x if x == MyEnum::C as i32 => Ok(MyEnum::C),
            _ => Err(()),
        }
    }
}
