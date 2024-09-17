fn main() {
    let n1 = 1;
    {
        let n2 = 2;
        let n3 = max(&n1, &n2);
        println!("max = {}", n3);
    }
}

fn max<'a>(a: &'a i32, b: &'a i32) -> &'a i32 {
    if a > b {
        a
    } else {
        b
    }
}