fn get_length_1(s: String) -> usize {
    s.len()
}

fn get_length_2(s: &String) -> usize {
    s.len()
}

// fn hello() -> &str{
//     "Hello, world!"
// }

fn hello() -> String {
    "hello".to_owned()
}

fn hello_static() -> &'static str{
    "hello"
}

// fn get_first_word(s: &str) -> &str {
//     return &s[..s.find(' ').unwrap_or(s.len())];
// }

fn get_first_word(s: &str) -> &str {
    for (idx, item) in s.char_indices() {
        if item == ' ' {
            return &s[..idx];
        }
    }
    &s[..]
}

fn main() {
    // copy and move
    // copy
    let a: i32 = 1;
    let b: i32 = a;
    println!("a = {}, b = {}", a, b);
    // move
    let s1: String = String::from("hello");
    let s3: String = s1.clone(); // clone
    let s2: String = s1;
    // println!("s1 = {}, s2 = {}", s1, s2); // error: borrow of moved value: `s1`
    println!("s2 = {}, s3 = {}", s2, s3);
    println!("{}", get_length_1(s3)); // s3 is moved to get_length_1, so s3 is invalid after this line
    // println!("{}", s3); // error: use of moved value: `s3`
    println!("{}", get_length_2(&s2)); // s2 is borrowed to get_length_2, so s2 is still valid after this line
    println!("{}", s2);
    println!("{}", hello());
    println!("{}", hello_static());
    let test_str = "world hello";
    println!("{}", get_first_word(test_str));
}