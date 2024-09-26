#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

#[derive(Debug)]
struct Point2<T, U> {
    x: T,
    y: U,
}

impl <T> Point<T>{
    fn new(x: T, y: T) -> Self {
        Point { x, y }
    }
    fn get_x(&self) -> &T {
        &self.x
    }
    fn get_y(&self) -> &T {
        &self.y
    }
    fn get_coordinates(&self) -> (&T, &T) {
        (&self.x, &self.y)
    }
}

fn swap<T>(x: T, y: T) -> (T, T) {
    (y, x)
}

fn main() {
    let p1 = Point { x: 5.0, y: 10.4 };
    let p2 = Point { x: 5, y: 10 };
    let p3 = Point2 { x: 5, y: 10.4 };
    let p4 = Point2 { x: 'a', y: 10 };
    println!("p1: {:?}, p2: {:?}, p3: {:?}, p4: {:?}", p1, p2, p3, p4);
    let res = swap(0, 1);
    println!("res: {:?}", res);
    let res = swap('a', 'b');
    println!("res: {:?}", res);
    let p5 = Point::new(5, 10);
    println!("p5: {:?}", p5.get_coordinates());
    
}
