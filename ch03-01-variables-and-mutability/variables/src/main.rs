fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    const SUBSCRIBER_COUNT: u32 = 100_000;
    println!("The value of SUBSCRIBER_COUNT is: {}", SUBSCRIBER_COUNT);

    let tup = (500, 6.4, 1, "hello");
    let (a, b, c, d) = tup;
    println!("The value of a is: {}", a);
    println!("The value of b is: {}", b);
    println!("The value of c is: {}", c);
    println!("The value of d is: {}", d);

    let sum = another_function(11, 33);
    println!("The value of sum is: {}", sum);

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number);

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter;
        }
    };
    println!("The value of result is: {}", result);

    let mut number = 3;
    while number != 0 {
        println!("The value of number is: {}", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("The value of element is: {}", element);
    }

    for number in (1..4).rev() {
        println!("The value of number is: {}", number);
    }

    // Line comment
    /*
        Block comment
    */
}

fn another_function(x: i32, y: i32) -> i32 {
    println!("Another function.");
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    x + y
}
