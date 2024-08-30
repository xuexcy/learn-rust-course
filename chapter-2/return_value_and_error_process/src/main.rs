use std::fs::File;
use std::io::{self, ErrorKind, Read};
use std::net::IpAddr;

fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}
fn call<T>(f: T)
where T: Fn() {
    println!("Start {}", type_of(&f));
    f();
    println!("End {}", type_of(&f));
}
fn main() {
    println!("Hello, world!");
    call(return_value_and_question_mark);
    call(panic);
}

fn panic() {
    let home: IpAddr = "127.0.0.1".parse().unwrap();
    panic!("crash and burn");
}
fn return_value_and_question_mark() {
    let file_name = "hello.txt";
    let f: Result<File, std::io::Error> = File::open(file_name);
    let f = match f {
        Ok(file) => file,
        // Err(error) => {
        //     panic!("problems opening the file: {:?}", error)
        // },
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(file_name) {
                Ok(fc) => fc,
                Err(e) => panic!("problems creating the file: {:?}", e),
            },
            other_error => panic!("problems opening the file: {:?}", other_error),
        }
    };
    // let f = File::open("hello.txt").unwrap();
    // let f = File::open("hello.txt").expect("Failed to open hello.txt");

}
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
fn open_file() -> Result<File, Box<dyn std::error::Error>> {
    let mut f = File::open("hello.txt")?;
    Ok(f)
}
fn read_username_from_file_v2() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("h.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

