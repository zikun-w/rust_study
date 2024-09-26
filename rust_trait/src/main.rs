trait Fire{
    fn fired(&self);
}

struct Employee{
    name: String,
    age: u32,
    salary: u32,
}

impl Fire for Employee{
    fn fired(&self){
        println!("Employee {} is fired", self.name);
    }
}

impl Employee {
    fn new(name: String, age: u32, salary: u32) -> Employee {
        Employee { name, age, salary }
    }
}

fn main() {
    let p1 = Employee::new("John".to_string(), 30, 5000);
    p1.fired();
}
