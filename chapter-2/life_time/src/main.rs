fn main() {
    let r;
    let x = 5;
    r = &x;
    println!("r: {}", r);

    let s1 = String::from("abcd");
    {
        let s2 = "xyz";
        let result = longest(s1.as_str(), s2);
        println!("the longest string is {}", result);
    }

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        return x;
    } else {
        return y;
    }
}
struct ImportantExcerpt<'a> {
    part: &'a str,
}
impl<'a: 'b, 'b> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
    fn announce_and_return_part(&'a self, announcement: &'b str) -> &'b str {
        println!("attention please: {}", announcement);
        self.part
    }
}
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    return &s[..];
}
