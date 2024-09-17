fn main() {
    let n1 = 1;
    {
        let n2 = 2;
        let n3 = max(&n1, &n2);
        println!("max = {}", n3);
    }
}

fn max<'a, 'b, 'out>(a: &'a i32, b: &'b i32) -> &'out i32 
    where 'a: 'out, 'b: 'out
{
    if a > b {
        a
    } else {
        b
    }
}