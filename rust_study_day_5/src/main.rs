fn main() {
    // if else & match
    let grade = 60;
    if grade < 60 {
        println!("You failed");
    } else {
        println!("You passed");
    }

    let message = if grade < 60 {"You failed"} else {"You passed"};

    println!("{message}");

    println!("{}", match grade {
        60..=100 => "You passed",
        0..=59 => "You failed",
        _ => "Illegal grade!!!",
    });
}
