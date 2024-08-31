/*! lib contains compute module */
pub mod compute;
/// `add_one` 将指定值加 1
/// # Example
///
/// ```
/// let arg = 5;
/// let answer = comment_and_document::add_one(arg);
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

/** `add_two` 将指定值加 2
```
let arg = 5;
let answer = comment_and_document::add_two(arg);
assert_eq!(7, answer);
```
 */
pub fn add_two(x: i32) -> i32 {
    x + 2
}

pub mod a {
    /// [`crate::MyStruct`]
    pub fn add_one(x: i32) -> Option<i32> {
        Some(x + 1)
    }
}
pub struct MyStruct;

/// [`Foo`](struct@Foo)
pub struct Bar;
/// [`Foo`](fn@Foo2)
pub struct Foo;
/// [`foo!`]
pub fn Foo2() {}

#[macro_export]
macro_rules! foo {
    () => {

    };
}

#[doc(alias = "x")]
#[doc(alias = "big")]
pub struct BigX;
#[doc(alias("y", "big"))]
pub struct BigY;

