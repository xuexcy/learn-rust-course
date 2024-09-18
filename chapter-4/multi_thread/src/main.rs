use std::borrow::BorrowMut;
use std::cell::Cell;
use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::Sub;
use std::rc::Rc;
use std::sync::{
    atomic::{AtomicBool, AtomicU64, Ordering},
    mpsc::{self, Receiver, Sender},
    Arc, Barrier, Condvar, Mutex, Once,
    RwLock,
};
use std::thread::{self, spawn, JoinHandle, LocalKey};
use std::time::{Duration, Instant};

use rand::thread_rng;
use rand::Rng;
use thread_local::ThreadLocal;
use tokio::io::Join;
use tokio::sync::Semaphore;

fn name_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}
fn call<F: Fn()>(f: &F) {
    let name = name_of(&f);
    println!("Start {}", name);
    f();
    println!("End {}\n", name);
}

fn main() {
    call(&multi_thread);
    call(&msg_transmit);
    call(&lock_condvar_semaphore);
    call(&atomic_and_mem_seq);
}

struct Foo;
impl Foo {
    thread_local! {
        static FOO: RefCell<usize> = RefCell::new(0);
    }
}

thread_local! {
    static FOO: RefCell<usize> = RefCell::new(0);
}
struct Bar {
    foo: &'static LocalKey<RefCell<usize>>,
}
impl Bar {
    fn constructor() -> Self {
        Self { foo: &FOO }
    }
}
static mut VAL: usize = 0;
static INIT: Once = Once::new();

fn multi_thread() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("spawn thread {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 1..5 {
        println!("main thread {}", i);
        thread::sleep(Duration::from_millis(1));
    }
    handle.join().unwrap();

    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("here's a vector: {:?}", v);
    });
    handle.join().unwrap();

    let new_thread = thread::spawn(move || {
        thread::spawn(move || {
            // loop {
            //     println!("I am a new thread.");
            // }
        })
    });
    new_thread.join().unwrap();
    println!("Child thread is finish!");
    thread::sleep(Duration::from_millis(100));

    let mut ht = Arc::new(Mutex::new(HashMap::<u32, u32>::new()));
    let num_threads = 5;
    let adds_per_threads = 10;
    let mut handles: Vec<JoinHandle<()>> = vec![];
    for _i in 0..num_threads {
        let ht = Arc::clone(&ht);
        let handle = thread::spawn(move || {
            for _j in 0..adds_per_threads {
                let k = thread_rng().gen::<u32>();
                let v = thread_rng().gen::<u32>();
                ht.lock().unwrap().insert(k, v);
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }

    let capacity = 6;
    let mut handles = Vec::with_capacity(capacity);
    let barrier = Arc::new(Barrier::new(capacity));
    for _ in 0..capacity {
        let b = Arc::clone(&barrier);
        handles.push(thread::spawn(move || {
            println!("before wait");
            b.wait();
            println!("after wait");
        }));
    }
    for handle in handles {
        handle.join().unwrap();
    }
    thread_local!(static FOO: RefCell<u32> = RefCell::new(1));
    FOO.with(|f| {
        assert_eq!(*f.borrow(), 1);
        *f.borrow_mut() = 2;
    });
    let t = thread::spawn(move || {
        FOO.with(|f| {
            assert_eq!(*f.borrow(), 1);
            *f.borrow_mut() = 3;
        })
    });
    t.join().unwrap();

    FOO.with(|f| {
        assert_eq!(*f.borrow(), 2);
    });
    Foo::FOO.with(|x| println!("{:?}", x));

    let tls = Arc::new(ThreadLocal::new());
    let mut v = vec![];
    for _ in 0..5 {
        let tls2 = tls.clone();
        let handle = thread::spawn(move || {
            let cell = tls2.get_or(|| Cell::new(0));
            cell.set(cell.get() + 1);
        });
        v.push(handle);
    }
    for handle in v {
        handle.join().unwrap();
    }
    let tls = Arc::try_unwrap(tls).unwrap();
    let total = tls.into_iter().fold(0, |x, y| {
        println!("x: {}, y: {}", x, y.get());
        x + y.get()
    });
    assert_eq!(total, 5);

    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = pair.clone();
    thread::spawn(move || {
        let (lock, cvar) = &*pair2;
        let mut started = lock.lock().unwrap();
        println!("changing started");
        *started = true;
        cvar.notify_one();
    });
    let (lock, cvar) = &*pair;
    let mut started = lock.lock().unwrap();
    while !*started {
        started = cvar.wait(started).unwrap();
    }
    println!("started changed");

    let handle1 = thread::spawn(move || {
        INIT.call_once(|| unsafe {
            VAL = 1;
        });
    });
    let handle2 = thread::spawn(move || {
        INIT.call_once(|| unsafe {
            VAL = 2;
        });
    });
    handle1.join().unwrap();
    handle2.join().unwrap();
    println!("{}", unsafe { VAL });
}

enum Fruit {
    Apple(u8),
    Orange(String),
}

fn msg_transmit() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        tx.send(1).unwrap();
    });
    println!("receive {}", rx.recv().unwrap());

    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        tx.send(2).unwrap();
    });
    println!("receive {:?}", rx.try_recv());
    println!("receive {:?}", rx.try_recv());
    std::thread::sleep(Duration::from_millis(1));
    println!("receive {:?}", rx.try_recv());
    std::thread::sleep(Duration::from_millis(1));
    println!("receive {:?}", rx.try_recv());

    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();
    thread::spawn(move || {
        let s = "send_string".to_string();
        let v = vec![
            "h".to_string(),
            "e".to_string(),
            "a".to_string(),
            "b".to_string(),
        ];
        tx1.send(s).unwrap();
        for e in v {
            tx1.send(e).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
        // s moved because of without Copy
        // println!("s={}", s);
    });
    thread::spawn(move || {
        let v = vec![
            "hh".to_string(),
            "ee".to_string(),
            "aa".to_string(),
            "bb".to_string(),
        ];
        for e in v {
            tx.send(e).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });
    for recv in rx {
        println!("recv s={}", recv);
    }
    // 异步通道
    let (tx, rx) = mpsc::sync_channel(0);
    let handle = thread::spawn(move || {
        println!("发送之前");
        tx.send(1).unwrap();
        println!("发送之后");
    });

    println!("睡眠之前");
    thread::sleep(Duration::from_secs(3));
    println!("睡眠之后");

    println!("receive {}", rx.recv().unwrap());
    handle.join().unwrap();

    let (tx, rx): (Sender<Fruit>, Receiver<Fruit>) = mpsc::channel();
    tx.send(Fruit::Orange("h".to_string())).unwrap();
    tx.send(Fruit::Apple(8)).unwrap();
    drop(tx);
    for fruit in rx {
        match fruit {
            Fruit::Apple(v) => println!("apple={}", v),
            Fruit::Orange(v) => println!("orange={}", v),
        }
    }
    println!("hhhhh");

    let (send, recv) = mpsc::channel();
    let num_threads = 3;
    for i in 0..num_threads {
        let local_send = send.clone();
        thread::spawn(move || {
            local_send.send(i).unwrap();
            println!("thread {:?} finished", i);
        });
    }
    drop(send);
    for x in recv {
        println!("Got: {}", x);
    }
    println!("finished iterating");
}

use lazy_static::lazy_static;
lazy_static! {
    static ref MUTEX1: Mutex<i64> = Mutex::new(0);
    static ref MUTEX2: Mutex<i64> = Mutex::new(0);
}

#[tokio::main]
async fn lock_condvar_semaphore() {
    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }
    println!("m = {:?}", m);

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..10 {
        let local_counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = local_counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Result: {}", *counter.lock().unwrap());

    let mut children = vec![];
    for i_thread in 0..2 {
        children.push(thread::spawn(move || {
            for _ in 0..1 {
                if i_thread % 2 == 0 {
                    let guard = MUTEX1.lock().unwrap();
                    println!("thread {} locked MUTEX1, try lock MUTEX2", i_thread);
                    thread::sleep(Duration::from_millis(10));
                    let guard = MUTEX2.try_lock().unwrap();
                    println!("thread {} locked MUTEX2, value = {:?}", i_thread, guard);
                } else {
                    let guard = MUTEX2.lock();
                    println!("thread {} locked MUTEX2, try lock MUTEX1", i_thread);
                    thread::sleep(Duration::from_millis(10));
                    let guard = MUTEX1.try_lock();
                    println!("thread {} locked MUTEX1, value = {:?}", i_thread, guard);
                }
            }
        }));
    }
    for child in children {
        let _ = child.join();
    }

    let lock = RwLock::new(5);
    {
        let r1 = lock.read().unwrap();
        let r2 = lock.read().unwrap();
        assert_eq!(*r1, 5);
        assert_eq!(*r2, 5);
    }
    {
        let mut w = lock.write().unwrap();
        *w += 1;
        assert_eq!(*w, 6);
    }

    let flag = Arc::new(Mutex::new(false));
    let cond = Arc::new(Condvar::new());
    let cflag = flag.clone();
    let ccond = cond.clone();
    let hdl = thread::spawn(move || {
        let mut lock = cflag.lock().unwrap();
        let mut counter = 0;
        while counter < 3 {
            while !*lock {
                lock = ccond.wait(lock).unwrap();
            }
            *lock = false;
            counter += 1;
            println!("inner counter: {}", counter);
        }
    });
    let mut counter = 0;
    loop {
        thread::sleep(Duration::from_millis(1000));
        *flag.lock().unwrap() = true;
        counter += 1;
        if counter > 3 {
            break;
        }
        println!("outside counter: {}", counter);
        cond.notify_one();
    }
    hdl.join().unwrap();
    println!("{:?}", flag);

    let semaphore = Arc::new(Semaphore::new(3));
    let mut join_handles = Vec::new();
    for i in 0..5 {
        let permit = semaphore.clone().acquire_owned().await.unwrap();
        join_handles.push(tokio::spawn(async move {
            println!("tokio thread {}", i);
            drop(permit);
        }));
    }
    for handle in join_handles {
        handle.await.unwrap();
    }
}

const N_TIMES: u64 = 10000000;
const N_THREADS: usize = 10;
static R: AtomicU64 = AtomicU64::new(0);
fn add_n_times(n: u64) -> JoinHandle<()> {
    thread::spawn(move || {
        for _ in 0..n {
            R.fetch_add(1, Ordering::Relaxed);
        }
    })
}

static mut DATA: u64 = 0;
static READY: AtomicBool = AtomicBool::new(false);
fn reset() {
    unsafe {
        DATA = 0;
    }
    READY.store(false, Ordering::Relaxed);
}
fn producer() -> JoinHandle<()> {
    thread::spawn(move || {
        unsafe {
            DATA = 100;
        }
        READY.store(true, Ordering::Release);
    })
}
fn consumer() -> JoinHandle<()> {
    thread::spawn(move || {
        while !READY.load(Ordering::Acquire) {}
        assert_eq!(100, unsafe { DATA });
    })
}

#[derive(Debug)]
struct MyBox(*mut u8);
unsafe impl Send for MyBox {}

#[derive(Debug)]
struct MyBoxV2(*const u8);
unsafe impl Send for MyBoxV2 {}
unsafe impl Sync for MyBoxV2 {}
fn atomic_and_mem_seq() {
    let s = Instant::now();
    let mut threads = Vec::with_capacity(N_THREADS);
    for _ in 0..N_THREADS {
        threads.push(add_n_times(N_TIMES));
    }
    for thread in threads {
        thread.join().unwrap();
    }
    assert_eq!(N_TIMES * N_THREADS as u64, R.load(Ordering::Relaxed));
    println!("{:?}", Instant::now().sub(s));

    for _ in 0..100 {
        reset();
        let t_producer = producer();
        let t_consumer = consumer();
        t_producer.join().unwrap();
        t_consumer.join().unwrap();
    }

    let p = MyBox( 5 as *mut u8);
    let t = thread::spawn(move || {
        println!("{:?}", p);
    });
    t.join().unwrap();

    let b = &MyBoxV2(5 as *const u8);
    let v = Arc::new(Mutex::new(b));
    let t = thread::spawn(move || {
        let _v1 = v.lock().unwrap();
    });
    t.join().unwrap();

}
