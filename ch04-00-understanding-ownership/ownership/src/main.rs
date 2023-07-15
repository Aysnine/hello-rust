fn main() {
    let s = String::from("hello");
    takes_ownership(s);
    // println!("{}", s); // error: value borrowed here after move

    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
    // println!("{}", s2); // error: value borrowed here after move

    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    let mut s = String::from("hello");
    change(&mut s);

    let r1 = &mut s;
    // let r2 = &mut s; // error: cannot borrow `s` as mutable more than once at a time
    // let r3 = &s; // error: cannot borrow `s` as immutable because it is also borrowed as mutable
    println!("{}", r1);

    let reference_to_nothing = dangle();

    let mut s = String::from("hello world");
    let word = first_word(&s);

    let s2  = "hello world";
    let word2 = first_word_x(s2);
    println!("{}", word2);
}

fn first_word_x(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() { // iter() returns each element in a collection
        if item == b' ' { // b' ' is byte literal
            return &s[0..i];
        }
    }
    &s[..]
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() { // iter() returns each element in a collection
        if item == b' ' { // b' ' is byte literal
            return i;
        }
    }
    s.len()
}

fn dangle() -> String {
    let s = String::from("hello");
    s
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn calculate_length(s: &String) -> usize {
    let length = s.len();
    length
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}
