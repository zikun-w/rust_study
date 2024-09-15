fn add(x:i32, y:i32) -> i32 {
    x + y
}

fn change_i32(x: &mut i32) {
    *x += 10;
}

#[derive(Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

fn print_point(p: Point) {
    println!("Point: x = {}, y = {}", p.x, p.y);
}

fn move_func(c1: i32, c2: String) {
    println!("c1 = {}, c2 = {}", c1, c2);
}

fn func1(c1: &i32, c2: &String) {
    println!("c1 = {}, c2 = {}", c1, (*c2).to_uppercase());
}

fn func2(c1: &mut i32, c2: &mut String) {
    *c1 = 9;
    *c2 = String::from("qwq");
    println!("c1 = {}, c2 = {}", c1, (*c2).to_uppercase());
}

fn main() {
    let a = 5;
    let b = 7;
    let c = add(a, b);
    println!("The sum of {} and {} is {}", a, b, c);

    let mut d = 0;
    change_i32(&mut d);
    println!("The value of d after change is {}", d);

    let point = Point { x: 5, y: 10 };
    print_point(point);
    println!("The value of point.x is {}", point.x);

    let c1 = 10;
    let c2 = String::from("Hello");
    move_func(c1, c2);
    
    let mut c1 = c1;
    let mut c2 = String::from("World");
    func1(&c1, &c2);
    println!("c1 = {}, c2 = {}", c1, c2);

    func2(&mut c1, &mut c2);
    println!("c1 = {}, c2 = {}", c1, c2);
}