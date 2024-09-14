// static value
static STATIC_VAL_A: i32 = 1;
static mut STATIC_VAL_B: i32 = 2;

fn main() {
    println!("Hello, world!");
    // variable
    let a: i32 = 1;
    let b: &str = "abcdefg";
    let c: String = String::from(b);
    let d: char = '🍜';
    let e: i64 = a as i64;
    // constant
    const B: i32 = 1;
    // tuple
    let tup_a: (i32, i32, i32, i32) = (1, 2, 3, 4);
    let tup_b: () = ();
    // array
    let arr_a: [i32; 4] = [1, 2, 3, 4];
    let arr_b: [i32; 100] = [1; 100];
    println!("{}", a);
    println!("{}", B);
    println!("{}", STATIC_VAL_A);
    println!("{:?}", tup_a);
    println!("{:?}", tup_b);
    println!("{:?}", arr_a);
    println!("{:?}", arr_b);
    println!("{}", b);
    println!("{}", c);
    println!("{}", d);
    println!("{}", e);
    // modify a static value is unsafe
    unsafe {
        STATIC_VAL_B = 123;
        println!("{}", STATIC_VAL_B);
    }
    println!("{}, {}", i32::MAX, i32::MIN);
    println!("{}, {}", i64::MAX, i64::MIN);
    for item in arr_b {
        println!("{item}");
    }
    for i in 0..4{
        println!("{}", arr_a[i]);
    }
}
