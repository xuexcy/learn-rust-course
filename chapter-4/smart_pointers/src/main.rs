use std::borrow::Borrow;
use std::ops::Deref;
use std::ops::DerefMut;
use std::rc::Rc;
use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;
use std::cell::Cell;
use std::cell::RefCell;

fn name_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}
fn call<F: Fn()>(f: &F) {
    let name = name_of(&f);
    println!("Start {}", name_of(&f));
    f();
    println!("End {}", name_of(&f));
}
fn main() {
    println!("Hello, world!");
    call(&box_pointer);
    call(&deref);
    call(&drop);
    call(&rc_and_arc);
    call(&cell_and_refcell);
}

fn foo(x: &str) -> String {
    "Hello ".to_string() + x
}

enum List {
    Cons(i32, Box<List>),
    Nil,
}
trait Draw {
    fn draw(&self);
}
struct Button {
    id: u32,
}
impl Draw for Button {
    fn draw(&self) {
        println!("this is button: {}", self.id);
    }
}
struct Select {
    id: u32
}
impl Draw for Select {
    fn draw(&self) {
        println!("this is select: {}", self.id);
    }
}

fn gen_static_str() -> &'static str {
    let mut s = String::new();
    s.push_str("hello world");
    Box::leak(s.into_boxed_str())
}
fn box_pointer() {
    let b = foo("world");
    println!("{}", b);

    let arr = [0; 1000];
    let arr1 = arr;
    println!("{:?}", arr.len());
    println!("{:?}", arr1.len());

    let arr = Box::new([0; 1000]);
    let arr1 = arr;
    println!("{:?}", arr1.len());
    // println!("{:?}", arr.len());

    let elems: Vec<Box<dyn Draw>> = vec![Box::new(Button { id: 1 }), Box::new(Select { id: 2 })];
    for e in elems {
        e.draw();
    }

    let arr = vec![Box::new(1), Box::new(2)];
    let (first, second) = (&arr[0], &arr[1]);
    let sum = **first + **second;
}

struct MyBox<T>(T);
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}


fn deref() {
    let y = MyBox::new(5);
    assert_eq!(5, *y);
    assert_eq!(5, *y.deref());

    let mut s = MyBox::new(String::from("hello "));
    display(&mut s);
}
fn display(s: &mut String) {
    s.push_str("world");
    println!("{}", s);
}

struct HasDrop1;
struct HasDrop2;
impl Drop for HasDrop1 {
    fn drop(&mut self) {
        println!("drop 1");
    }
}
impl Drop for HasDrop2 {
    fn drop(&mut self) {
        println!("drop 2");
    }
}
struct HasTwoDrops {
    one: HasDrop1,
    two: HasDrop2,
}
impl Drop for HasTwoDrops {
    fn drop(&mut self) {
        println!("dropping two drops");
    }
}
fn drop() {
    let _x = HasTwoDrops {
        one: HasDrop1,
        two: HasDrop2,
    };
    println!("running");
    std::mem::drop(_x);
    println!("a");
}

struct Owner {
    name: String,
}
struct Gadget {
    id: i32,
    owner: Rc<Owner>,
}
fn rc_and_arc() {
    let s = String::from("hello");
    let a = Rc::new(s);
    let b = Rc::clone(&a);
    assert_eq!(2, Rc::strong_count(&a));
    assert_eq!(Rc::strong_count(&b), Rc::strong_count(&a));

    let a = Rc::new(String::from("test ref counting"));
    assert_eq!(1, Rc::strong_count(&a));
    let b = Rc::clone(&a);
    assert_eq!(2, Rc::strong_count(&a));
    {
        let c = Rc::clone(&a);
        assert_eq!(3, Rc::strong_count(&a));
    }
    assert_eq!(2, Rc::strong_count(&a));

    let gadget_owner: Rc<Owner> = Rc::new(Owner { name: "Gadget Man".to_string() });
    let g1 = Gadget {
        id: 2,
        owner: Rc::clone(&gadget_owner),
    };
    let g2 = Gadget {
        id: 3,
        owner: Rc::clone(&gadget_owner),
    };
    assert_eq!(3, Rc::strong_count(&gadget_owner));
    std::mem::drop(gadget_owner);
    println!("id: {}, owner name: {}", g1.id, g1.owner.name);
    println!("id: {}, owner name: {}", g2.id, g2.owner.name);
    assert_eq!(2, Rc::strong_count(&g1.owner));

    let mut handles: Vec<JoinHandle<()>> = vec![];
    let s = Arc::new(String::from("multi thread"));
    for i in 0..10 {
        let local_s = Arc::clone(&s);
        let handle = thread::spawn(move || {
            println!("id: {}, {}", i, local_s);
        });
        handles.push(handle);
    }
    let res: Vec<_> = handles.into_iter().map(|handle| handle.join()).collect();
}

trait Messenger {
    fn send(&self, msg: String);
}
struct MsgQueue {
    msg_cache: RefCell<Vec<String>>,
}
impl Messenger for MsgQueue {
    fn send(&self, msg: String) {
        self.msg_cache.borrow_mut().push(msg);
    }
}

fn retain_even(nums: &mut Vec<i32>) {
    let slice: &[Cell<i32>] = Cell::from_mut(&mut nums[..])
        .as_slice_of_cells();
    let mut i = 0;
    for num in slice.iter().filter(|num| num.get() % 2 == 0) {
        slice[i].set(num.get());
        i += 1;
    }
    nums.truncate(i);
}
fn cell_and_refcell() {
    let c = Cell::new("asdf");
    let one = c.get();
    c.set("q");
    let two = c.get();
    println!("{}, {}", one, two);

    let s = RefCell::new(String::from("hello, world"));
    let s1 = s.borrow();
    // let s2 = s.borrow_mut();
    //println!("{}, {}", s1, s2);

    let x = Cell::new(1);
    let y = &x;
    let z = &x;
    x.set(2);
    y.set(3);
    z.set(4);
    println!("{}", x.get());

    let mq = MsgQueue {
        msg_cache: RefCell::new(Vec::new()),
    };
    mq.send("msg".to_string());

    let s = Rc::new(RefCell::new("he".to_string()));
    let s1 = Rc::clone(&s);
    let s2 = Rc::clone(&s);
    s2.borrow_mut().push_str("hi");
    println!("{:?}\n{:?}\n{:?}", s, s1, s2);
}
