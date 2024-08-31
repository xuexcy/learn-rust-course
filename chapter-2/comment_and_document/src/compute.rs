//! some compute function
//!
/// # Panics
///
/// The function panics if the second argument is zero.
///
/// ```rust,should_panic
/// // panics on division by zero
/// comment_and_document::compute::div(10, 0);
/// ```
pub fn div(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("Divide-by-zero error");
    }
    a / b
}

/// ```rust,should_panic
/// # // 隐藏
/// # fn try_main() -> Result<(), String> {
/// let res = comment_and_document::compute::try_div(10, 0)?;
/// # Ok(())
/// # }
/// # fn main() {
/// # try_main().unwrap();
/// # }
/// ```
pub fn try_div(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Divide-by-zero"))
    } else {
        Ok(a / b)
    }
}

/// `add_one` 返回 [`Option`]
pub fn add_one(x: i32) -> Option<i32> {
    Some(x + 1)
}
