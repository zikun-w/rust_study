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

fn process_n_times(x: i32, f: fn(i32) -> i32, n: i32) -> i32 {
    let mut result = x;
    for _ in 0..n {
        result = f(result);
    }
    result
}

fn add_two(x: i32) -> i32 {
    x + 2
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

    let result = process_n_times(1, add_two, 5);
    println!("The result is {}", result);

    let vec1 = vec![1, 2, 3, 4, 5];
    let vec2 = vec1.iter().map(|x| x + 1).collect::<Vec<i32>>();
    let vec3 = vec1.clone().into_iter().filter(|x| x % 2 == 0).collect::<Vec<_>>();
    let sum_of_vec1 = vec1.iter().fold(0, |acc, &x| acc + x);
    println!("{:?}", vec2);
    println!("{:?}", vec3);
    println!("The sum of vec1 is {}", sum_of_vec1);
}