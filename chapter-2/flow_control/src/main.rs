fn main() {
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };
    println!("The value of number is: {}", number);

    let n = 6;
    if n % 4 == 0 {
        println!("number is divisible by 4");
    } else if n % 3 == 0 {
        println!("number is divisible by 3");
    } else if n % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    for i in 1..=5 {
        println!("{}", i);
    }
    let a = [4, 3, 2, 1];
    for (i, v) in a.iter().enumerate() {
        println!("第{}个元素是{}", i + 1, v);
    }
    for i in 1..4 {
        if i == 2 {
            continue;
        }
        println!("{}", i);
    }
    for i in 1..4 {
        if i == 2 {
            break;
        }
        println!("{}", i);
    }

    let mut n = 0;
    while  n <= 5 {
        println!("{}!", n);
        n += 1;
    }

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }
    for element in a.iter() {
        println!("the value is: {}", element);
    }

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);
}
