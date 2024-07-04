#[allow(unused_variables)]

fn main() {
    let a = 10; // i32
    let b: i32 = 20;
    let mut c = 30i32;
    c += 3;
    let d = 30_i32;
    let e = add(add(a, b), add(c, d));

    println!("(a+b)+(c+d)={}", e);

    call(variables);
    call(basic_types);
    call(char_bool_unit);
    call(statement_and_expression);
    call(function);
}
fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}
//fn call(f: fn()) {
fn call<T>(f: T)
where
    T: Fn(),
{
    println!("Start {}", type_of(&f));
    f();
    println!("End {}", type_of(&f));
}

fn add(i: i32, j: i32) -> i32 {
    i + j
}

struct Struct {
    e: i32,
}
fn variables() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let _x = 5; // unused
    let (a, mut b): (bool, bool) = (true, false);
    println!("a = {:?}, b = {:?}", a, b);
    b = true;
    assert_eq!(a, b);

    let (a, b, c, d, e);
    (a, b) = (1, 2);
    [c, .., d, _] = [1, 2, 3, 4, 5];
    Struct { e, .. } = Struct { e: 5 };
    assert_eq!([1, 2, 1, 4, 5], [a, b, c, d, e]);

    const MAX_POINTS: u32 = 100_000;
    println!("{MAX_POINTS}");

    let x = 5;
    let x = x + 1; // 6
    {
        let x = x * 2; // 12
        println!("The value of x in the inner scope is: {}", x);
    }
    // 6
    println!("The value of x is: {}", x);

    let spaces = "    ";
    let spaces = spaces.len(); // 4
    println!("{spaces}");
}

fn basic_types() {
    // let guess = "42".parse().expect("Not a number");
    let guess = "42".parse::<i32>().expect("Not a number");
    println!("{guess}");

    let _i = 1;  // default to i32

    // saturating 不超过或低于目标类型MAX和MIN
    assert_eq!(100u8.saturating_add(1), 101);
    assert_eq!(u8::MAX.saturating_add(17), u8::MAX);

    let a: u8 = 255;
    let b = a.wrapping_add(20);
    // (255 + 20) % 256
    assert_eq!(19, b);

    // default to f64 in rust
    let x = 2.0;  // f64
    let y: f32 = 3.0;

    let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
    let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3);
    println!("abc (f32)");
    println!("    0.1 + 0.2: {:x}", (abc.0 + abc.1).to_bits());
    println!("          0.3: {:x}", (abc.2).to_bits());
    println!();
    assert_eq!(abc.0 + abc.1, abc.2);

    println!("xcy (f64)");
    println!("    0.1 + 0.2: {:x}", (xyz.0 + xyz.1).to_bits());
    println!("          0.3: {:x}", (xyz.2).to_bits());
    // assert_eq!(xyz.0 + xyz.1, xyz.2);
    assert!((xyz.0 + xyz.1 - xyz.2) < 0.00001);

    let x: f32 = (-42.0_f32).sqrt();
    // x: NaN,  Nan不能用来比较
    // assert_eq!(x, x);
    if x.is_nan() {
        println!("undefined, x is NaN");
    }

    // 同类型才能相加，twenty被推导为成i32
    let twenty = 20;
    let twenty_one: i32 = 21;
    let twenty_two = 22i32;

    let addition = twenty + twenty_one + twenty_two;
    println!("{} + {} + {} = {}", twenty, twenty_one, twenty_two, addition);

    let one_million: i64 = 1_000_000;
    println!("{}", one_million.pow(2));

    let forty_twos = [
        42.0, 42f32, -42.0_f32
    ];
    println!("{:.2}", forty_twos[0]);
    let a: i32 = 2; // 010
    let b: i32 = 3; // 011
    assert_eq!(2, a & b);
    assert_eq!(3, a | b);
    assert_eq!(1, a ^ b);
    // b = 3
    // -b = !b + 1;
    // !b = -b - 1 = -3 -1 =-4
    assert_eq!(-4, !b);
    assert_eq!(16, a << b);
    assert_eq!(0, a >> b);
    // [1, 2, 3, 4, 5]
    for i in 1..=5 {
        println!("{}", i);
    }
    // [1, 2, 3, 4]
    for i in 1..5 {
        println!("{}", i);
    }
}

fn char_bool_unit() {
    // unicode 4 字节
    let x = '中';
    println!("size of x: {}", std::mem::size_of_val(&x));

}

fn statement_and_expression() {
    // statement 语句，完成操作
    let x = 1;
    // expression 表达式，返回值
    let y = {
        let z = 3;
        // expression 表达式，返回值
        z + 1
    };
    let y = if x % 2 == 1 {
        "odd"
    } else {
        "even"
    };
    let z = if x % 2 == 1 { "odd" } else { "even" };
}

// 返回类型为 ! 为永不返回
fn function_dead_end() -> ! {
    panic!("go ahead and dead");
}
// 返回类型为 ! 为永不返回
fn function_loop() -> ! {
    loop {}
}
fn function() {
    // function_dead_end();
    // function_loop();
}
