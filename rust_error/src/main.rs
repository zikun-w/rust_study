use std::num::ParseIntError;

fn divide(x: i32, y: i32) -> Result<f64, String> {
    if y == 0 {
        Err(String::from("divided by zero"))
    } else {
        Ok(x as f64 / y as f64)
    }
}

fn find_element(vec: &Vec<i32>, target: i32) -> Result<i32, String> {
    for (i, e) in vec.iter().enumerate() {
        if *e == target {
            return Ok(i as i32);
        }
    }
    Err(String::from("not found"))
}

fn find_element_2(vec: &Vec<i32>, target: i32) -> Option<usize> {
    let mut ret = None;
    for (i, e) in vec.iter().enumerate() {
        if *e == target {
            ret = Some(i);
        }
    }
    ret
}

fn parse_int(s: &str) -> Result<i32, ParseIntError> {
    let num = s.parse::<i32>()?;
    Ok(num)
}

#[derive(Debug)]
struct MyError {
    detail: String,
}

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MyError: {}", self.detail)
    }
}

impl std::error::Error for MyError {
    fn description(&self) -> &str {
        &self.detail
    }
}

fn func() -> Result<(), MyError> {
    Err(MyError {
        detail: String::from("some error") ,
    })
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    match divide(10, 2) {
        Ok(v) => println!("result is: {}", v),
        Err(e) => println!("error: {}", e),
    }
    match divide(10, 0) {
        Ok(v) => println!("result is: {}", v),
        Err(e) => println!("error: {}", e),
    }

    let vec = vec![1, 2, 3, 4, 5];
    match find_element(&vec, 3) {
        Ok(v) => println!("found at: {}", v),
        Err(e) => println!("error: {}", e),
    }
    match find_element(&vec, 6) {
        Ok(v) => println!("found at: {}", v),
        Err(e) => println!("error: {}", e),
    }

    match find_element_2(&vec, 3) {
        Some(v) => println!("found at: {}", v),
        None => println!("not found"), 
    }
    match find_element_2(&vec, 6) {
        Some(v) => println!("found at: {}", v),
        None => println!("not found"),
    }

    // panic 
    // let d: i32 = (-100..=100).sum();
    // let t = 100 / d;
    // println!("{}", t);

    let num = parse_int("101010");
    match num {
        Ok(v) => println!("parsed: {}", v),
        Err(e) => println!("error: {}", e),
    }
    let num = parse_int("101010abc");
    match num {
        Ok(v) => println!("parsed: {}", v),
        Err(e) => println!("error: {}", e),
    }

    match func() {
        Ok(_) => println!("ok"),
        Err(e) => println!("error: {}", e),
    }
    func()?;
    println!("ok");
    Ok(())
}
