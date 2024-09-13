use crate::List::Cons;
use crate::List::Nil;
use std::cell::RefCell;
use std::marker::PhantomPinned;
use std::pin::Pin;
use std::ptr::NonNull;
use std::rc::Rc;
use std::rc::Weak;

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}
impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

fn name_of<T>(_: &T) ->String {
    format!("{}", std::any::type_name::<T>())
}
fn call<F: Fn()>(f: &F) {
    let name = name_of(&f);
    println!("Start {}", name);
    f();
    println!("End {}", name);
}
fn main() {
    println!("Hello, world!");
    call(&weak_and_reference_loop);
    call(&self_ref);
}

struct Gadget {
    id: i32,
    owner: Rc<Owner>,
}
struct Owner {
    name: String,
    gadgets: RefCell<Vec<Weak<Gadget>>>,
}

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}
fn weak_and_reference_loop() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
    println!("a rc={}", Rc::strong_count(&a));
    println!("a tail={:?}", a.tail());
    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
    println!("a rc={}", Rc::strong_count(&a));
    println!("b rc={}", Rc::strong_count(&b));
    println!("b tail={:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b)
    }
    println!("a rc={}", Rc::strong_count(&a));
    println!("b rc={}", Rc::strong_count(&b));

    let five = Rc::new(5);
    let weak_five = Rc::downgrade(&five);
    let strong_five: Option<Rc<i32>> = weak_five.upgrade();
    assert_eq!(Some(Rc::new(5 as i32)), strong_five);
    assert_eq!(5, *strong_five.unwrap());
    assert_eq!(1, Rc::strong_count(&five));
    drop(five);
    let strong_five: Option<Rc<i32>> = weak_five.upgrade();
    assert_eq!(None, strong_five);

    let gadget_owner: Rc<Owner> = Rc::new(
        Owner {
            name: "man".to_string(),
            gadgets: RefCell::new(Vec::new()),
        }
    );
    let g1 = Rc::new(Gadget { id: 1, owner: gadget_owner.clone() });
    let g2 = Rc::new(Gadget { id: 2, owner: gadget_owner.clone() });
    gadget_owner.gadgets.borrow_mut().push(Rc::downgrade(&g1));
    gadget_owner.gadgets.borrow_mut().push(Rc::downgrade(&g2));

    for g_opt in gadget_owner.gadgets.borrow().iter() {
        let g = g_opt.upgrade().unwrap();
        println!("Gadget {} owned by {}", g.id, g.owner.name);
    }

    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),  // 1
        Rc::weak_count(&leaf),  // 0
    );
    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&leaf),  // 2
            Rc::weak_count(&leaf),  // 0
        );
        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),  // 1
            Rc::weak_count(&branch),  // 1
        );
    }
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());  // None
    println!(
        "branch strong = {}, weak = {}",
        Rc::strong_count(&leaf),  // 1
        Rc::weak_count(&leaf),  // 0
    );
}

struct SelfRef<'a> {
    value: String,
    pointer_to_value: &'a str,
}

#[derive(Debug)]
struct WhatAboutThis<'a> {
    name: String,
    nickname: Option<&'a str>,
}
#[derive(Debug)]
struct SelfRefV2 {
    value: String,
    pointer_to_value: *const String,
}
impl SelfRefV2 {
    fn new(txt: &str) -> Self {
        SelfRefV2 {
            value: String::from(txt),
            pointer_to_value: std::ptr::null(),
        }
    }
    fn init(&mut self) {
        self.pointer_to_value = &self.value;
    }
    fn value(&self) ->& str {
        &self.value
    }
    fn pointer_to_value(&self) -> &String {
        assert!(!self.pointer_to_value.is_null(), "without init");
        unsafe { &*self.pointer_to_value }
    }
}
struct Unmoveable {
    data: String,
    slice: NonNull<String>,
    _pin: PhantomPinned,
}
impl Unmoveable {
    fn new(data: String) -> Pin<Box<Self>> {
        let res = Unmoveable {
            data,
            slice: NonNull::dangling(),
            _pin: PhantomPinned,
        };
        let mut boxed = Box::pin(res);
        let slice = NonNull::from(&boxed.data);
        unsafe {
            let mut_ref: Pin<&mut Self> = Pin::as_mut(&mut boxed);
            Pin::get_unchecked_mut(mut_ref).slice = slice;
        }
        boxed
    }
}
fn self_ref() {
    let s = "aaa".to_string();
    // let v = SelfRef {
    //     value: s,
    //     pointer_to_value: &s,
    // };
    let mut tricky = WhatAboutThis {
        name: "a".to_string(),
        nickname: None,
    };
    tricky.nickname = Some(&tricky.name[..4]);
    println!("{:?}", tricky);

    let mut t = SelfRefV2::new("hello");
    t.init();
    println!("{}, {:p}", t.value(), t.pointer_to_value());
}
