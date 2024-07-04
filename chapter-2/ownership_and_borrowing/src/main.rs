fn main() {
    call(ownership);
    call(reference_and_borrowing);
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

fn ownership() {
    let mut s = String::from("hello");
    s.push_str(", world");
    println!("{}", s);

    let x = 5;
    // 基本类型存在栈上，rust自动拷贝
    let y = x;  // 基本类型固定大小，自动拷贝
    let s1 = String::from("hello");
    // String:
    //  栈上: 堆指针、字符串长度(已使用)、字符串容量(已分配)
    //  堆上: 具体字符
    let s2 = s1;
    // 浅拷贝(shallow copy): s2 从 s1 拷贝堆指针、容量、长度，之后s1、s2皆可用
    // 移动(move): 将s1的堆指针、长度、容量复制到s2，并让s1失效无法使
    // rust使用了移动

    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    // trait Copy: 一个旧变量在被赋值给其他变量后仍然可用

    takes_ownership(s2);
    // s2 has been token
    // println!("{s2}");
    let x = 1;
    make_copy(x);
    println!("{x}")

}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}
fn make_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn reference_and_borrowing() {
    let x: i32 = 5;
    let y: &i32= &x;
    assert_eq!(5, x);
    assert_eq!(5, *y);

    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    let mut s = String::from("hello");
    change(&mut s);
}
fn calculate_length(s: &String) -> usize {
    s.len()
}
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
