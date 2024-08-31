use std::fmt;
use std::io;
use std::io::Result as IoResult;

use rand::Rng;

fn function1() -> fmt::Result {
   fmt::Result::Ok(())
}
fn function2() -> io::Result<()> {
    io::Result::Ok(())
}
fn function3() -> IoResult<()> {
    IoResult::Ok(())
}
fn main() {

    println!("Hello, world!");
}
