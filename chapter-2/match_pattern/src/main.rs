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
    call(match_lef_if);
    call(option);
    call(pattern_scene);
    call(all_pattern);
}

#[derive(Debug)]
enum Direction {
    East,
    West,
    North,
    South,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

enum IpAddr {
    Ipv4,
    Ipv6
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}
enum CoinV2 {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents_v2(coin: CoinV2) -> u8 {
    match coin {
        CoinV2::Penny => 1,
        CoinV2::Nickel => 5,
        CoinV2::Dime => 10,
        CoinV2::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        }
    }
}

enum Action {
    Say(String),
    MoveTo(i32, i32),
    ChangeColorRGB(u16, u16, u16),
}

#[derive(Debug)]
enum  MyEnum {
    Foo,
    Bar,
}

fn match_lef_if() {
    let dire = Direction::South;
    match dire {
        Direction::East => println!("East"),
        Direction::North | Direction::South => {
            println!("South or North");
        },
        _ => println!("West"),
    }

    let ip1 = IpAddr::Ipv6;
    let ip_str = match ip1 {
        IpAddr::Ipv4 => "127.0.0.1",
        _ => "::1",
    };
    println!("{}", ip_str);

    let actions = [
        Action::Say("Hello Rust".to_string()),
        Action::MoveTo(1, 2),
        Action::ChangeColorRGB(255,  255, 0),
    ];
    for action in actions {
        match action {
            Action::Say(s) => {
                println!("{}", s);
            },
            Action::MoveTo(x, y) => {
                println!("point from (0, 0) move to ({}, {})", x, y);
            },
            Action::ChangeColorRGB(r, g, _) => {
                println!(
                    "change color into '(r:{}, g:{}, b:0)', 'b' has been ignored",
                    r, g);
            }
        }
    }
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }

    let dire = Direction::South;
    match dire {
        Direction::East => println!("East"),
        other => println!("other direction: {:?}", other),
    };
    let v = Some(3u8);
    match v {
        Some(3) => println!("three"),
        _ => (),
    }
    if let Some(3) = v {
        println!("three");
    }
    let v = vec![MyEnum::Foo, MyEnum::Bar, MyEnum::Foo];
    println!("{:?}", v);
    let v = v.iter().filter(|x| matches!(x, MyEnum::Foo));
    println!("{:?}", v);

    let foo = 'f';
    assert!(matches!(foo, 'A'..='Z' | 'a'..='z'));
    let bar = Some(4);
    assert!(matches!(bar, Some(x) if x > 2));

    let age  = Some(30);
    println!("在匹配前，age是{:?}", age);
    if let Some(age) = age {
        println!("匹配出来的age是{}", age);
    }
    println!("在匹配后，age是{:?}", age);

}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
fn option() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}
fn pattern_scene() {
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
    let v = vec!['a', 'b', 'c'];
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }
    let point = (3, 5);
    print_coordinates(&point);
}

fn all_pattern() {
    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }
    println!("at the end: x = {:?}, y = {:?}", x, y);

    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }
    let x = 'c';
    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }
    let p = Point { x: 0, y: 7 };
    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);
    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);
    match p {
        Point { x, y: 0 } => println!("One the x axis at {}", x),
        Point { x: 0, y } => println!("One the y axis at {}", y),
        Point { x, y } => println!("On neight axis: ({}, {})", x, y),
    }
    let arr: [u16; 2] = [114, 514];
    let [x, y] = arr;
    assert_eq!(114, x);
    assert_eq!(514, y);
    let arr: &[u16] = &[114, 514];
    if let [x, ..] = arr {
        assert_eq!(x, &114);
    }
    if let &[.., y] = arr {
        assert_eq!(y, 514);
    }
    let arr: &[u16] = &[];
    assert!(matches!(arr, [..]));
    //assert!(matches!(arr, [x, ..]));
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);
    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        },
        _ => {
            setting_value = new_setting_value;
        }
    }
    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {}, {}, {}", first, third, fifth)
        },
    }

    let _x = 5;
    let s = Some(String::from("Hello!"));
    if let Some(_s) = s {
        println!("found a string");
    }
    // s moved to Some(_s)
    // println!("{:?}", s);
    let s = Some(String::from("hello!"));
    if let Some(_) = s {
        println!("found a string");
    }
    println!("{:?}", s);

    let origin = Point { x: 0, y: 0};
    match origin {
        Point { x, .. } => println!("x is {}", x),
    }

    let num = Some(4);
    // match guard
    match num {
        Some(x) if x < 5 => print!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }
    enum Message {
        Hello { id: i32 },
    }
    let msg = Message::Hello { id: 5 };
    // @ 绑定
    match msg {
        Message::Hello { id: id_variable @ 3..=7 } => {
            println!("Found an id in range: {}", id_variable);
        },
        Message::Hello { id: 10..=12  } => {
            println!("Found an id in another");
        },
        Message::Hello { id } => {
            println!("Found some other id: {}", id);
        }
    }
    let p @ Point { x: px, y: py } = Point { x: 10, y: 23};
    println!("x: {}, y: {}", px, py);
    println!("{:?}", p);
}
