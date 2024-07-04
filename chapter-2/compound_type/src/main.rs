#![allow(unused_variables)]

use std::str;

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
    call(compound_type);
    call(string_and_slice);
}

type File = String;
fn open(f: &mut File) -> bool {
    true
}
fn close(f: &mut File) -> bool {
    false
}
#[allow(dead_code)]
fn read(f: &mut File, save_to: &mut Vec<u8>) -> ! {
    unimplemented!()
}

fn compound_type() {
    let mut f1 = File::from("f1.txt");
    open(&mut f1);
    // read(&mut f1, &mut vec![]);
    close(&mut f1);
}

fn string_and_slice() {
    let my_name: &str = "Pascal";
    // greet(my_name);
    greet(my_name.to_string());

    let s = String::from("hello world");
    // slice
    let hello = &s[0..5];
    let hello = &s[..5];
    let world = &s[6..11];
    let world = &s[6..];
    let hello_world = &s[..];

    let mut s = String::from("hello world");
    let word = first_word(&s);
    println!("the first word is: {}", word);
    s.clear();
    // println!("the first word is: {}", word);

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);

    let s = String::from("Hello, world!");
    say_hello(&s);
    say_hello(&s[..]);
    say_hello(s.as_str());

    let mut s = String::from("Hello");
    s.push_str("rust");
    println!("{}", s);
    s.push('!');
    println!("{}", s);

    let mut s= String::from("hello rust!");
    s.insert(5, ',');
    println!("{}", s);
    s.insert_str(6, " I like");
    println!("{}", s);

    let string_replace = String::from("I like rust. Learning rust is my favorite!");
    let new_string_replace = string_replace.replace("rust", "Rust");
    dbg!(new_string_replace);
    let new_string_replacen = string_replace.replacen("rust", "Rust", 1);
    dbg!(new_string_replacen);

    let mut string_replace_range = String::from("I like rust!");
    string_replace_range.replace_range(7..8, "R");
    dbg!(string_replace_range);

    let mut string_pop = String::from("rust pop 中文!");
    let p1 = string_pop.pop();
    let p2 = string_pop.pop();
    dbg!(p1);
    dbg!(p2);
    dbg!(string_pop);

    let mut string_remove = String::from("测试remove方法");
    // 18字节 = 汉字3字节 * 4 + 英文1字节 * 6
    println!("string_remove 占 {} 个字节", std::mem::size_of_val(string_remove.as_str()));
    string_remove.remove(0);
    // string_remove.remove(3);
    dbg!(string_remove);

    let mut string_truncate = String::from("测试truncate");
    // delete from index to end
    string_truncate.truncate(3);
    dbg!(string_truncate);

    let mut string_clear = String::from("string clear");
    string_clear.clear();
    dbg!(string_clear);

    let string_append = String::from("hello ");
    let string_rust = String::from("rust");
    // fn add(self, s: &str) -> String
    // 这里是 (self), 转移了所有权, 所以 string_append 后面不能再用
    let result = string_append + &string_rust;
    println!("{}", result);
    let mut result = result + "!";
    result += "!!!";
    println!("{}", result);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("{}", s);

    let s1 = "hello";
    let s2 = String::from("rust");
    println!("{}", format!("{} {}", s1, s2));

    let byte_escape = "I'm writing \x52\x75\x73\x74";
    println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);

    let unicode_codepoint = "\u{211D}";
    let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";
    println!(
        "Unicode character {} (U+211D) is called {}",
        unicode_codepoint, character_name);

    let long_string = "String literals
                                can span multiple lines.
                                The linebreak and indentation here \
                                <- can be escaped too!";
    println!("{}", long_string);

    let quotes = r#"And the I said: "There is no escaped!""#;
    let longer_delimiter = r###"A string with "# in it. And even "##!"###;
    println!("{}", longer_delimiter);

    for c in "中国人".chars() {
        println!("{}", c);
    }
    for b in "中国人".bytes() {
        println!("{}", b);
    }

    let bytestring: &[u8; 21] = b"this is a byte string";
    println!("A byte string: {:?}", bytestring);

    let escaped = b"\x52\x75\x73\x74 as bytes";
    println!("some escaped bytes: {:?}", escaped);

    let raw_bytestring = br"\u{211D} is not escaped here";
    println!("{:?}", raw_bytestring);

    if let Ok(my_str) = str::from_utf8(raw_bytestring) {
        println!("And the same as text: '{}'", my_str);
    }
    let shift_jis = b"\x82\xe6\x82\xa8\x82\xb1\x82\xbb"; // "ようこそ" in SHIFT-JIS
    match str::from_utf8(shift_jis) {
        Ok(my_str) => println!("Conversion successful: '{}'", my_str),
        Err(e) => println!("Conversion failed: {:?}", e),
    }


}
fn first_word(s: &String) -> &str {
    &s[..1]
}
fn say_hello(s: &str) {
    println!("{}", s);
}

fn greet(name: String) {
    println!("Hello, {}!", name);
}

