fn divide(x: i32, y: i32) -> Result<f64, String> {
    if y == 0 {
        Err(String::from("divided by zero"))
    } else {
        Ok(x as f64 / y as f64)
    }
}

fn find_element(vec: &Vec<i32>, target: i32) -> Result<i32, String> {
    let mut ret = Err(String::from("not found"));
    for (i, e) in vec.iter().enumerate() {
        if *e == target {
            ret = Ok(i as i32);
        }
    }
    ret
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

fn main() {
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
}
