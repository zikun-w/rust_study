enum Sex{
    Male,
    Female,
}

fn print_sex(sex: &Sex){
    match sex {
        Sex::Male => println!("This is a male"),
        Sex::Female => println!("This is a female"),
    }
}

enum Student{
    ID(usize),
    Name(String),
    Unknown,
}

impl Student {
    fn print_student_info(&self){
        match self {
            Student::ID(id) => println!("Student ID: {}", id),
            Student::Name(name) => println!("Student Name: {}", name),
            Student::Unknown => println!("Unknown student"),
        }
    }
    fn print_student_info_base(student: &Student){
        match student {
            Student::ID(id) => println!("Student ID: {}", id),
            Student::Name(name) => println!("Student Name: {}", name),
            Student::Unknown => println!("Unknown student"),
        }
    }
}

fn main() {
    let mut sex = Sex::Female;
    print_sex(&sex);
    sex = Sex::Male;
    print_sex(&sex);
    let student1 = Student::ID(123);
    let student2 = Student::Name(String::from("Alice"));
    let student3 = Student::Unknown;
    student1.print_student_info();
    student2.print_student_info();
    student3.print_student_info();
    Student::print_student_info_base(&student1);
    Student::print_student_info_base(&student2);
    Student::print_student_info_base(&student3);
}
