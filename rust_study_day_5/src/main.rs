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

    println!("{}", match grade {
        x if x < 58 => "You failed",
        x if x > 61 => "You passed",
        58 | 59 => "Bad luck",
        60 | 61 => "Good luck",
        _ => "Illegal grade!!!",
    });

    // loop & while
    // loop
    let mut cnt = 1;
    loop {
        println!("loop {}", cnt);
        cnt += 1;
        if cnt > 10 {
            break;
        }
    }

    cnt = 1;
    while cnt <= 10 {
        println!("while {}", cnt);
        cnt += 1;
    }

    let arr = [1, 2, 3, 4, 5];
    for num in arr {
        println!("{}", num);
    }

    for i in 1..=10 {
        println!("{}", i);
    }

    // break
    let mut i = 1;
    'outer: loop {
        println!("outer loop");
        loop {
            println!("inner loop {}", i);
            i += 1;
            if i > 10 {
                break 'outer;
            }
        }
    }

    // iter 
    let arr = [1, 2, 3, 4, 5].to_vec();
    let new_arr: Vec<_> = arr.iter().map(|x| x * x).collect();
    println!("{:?}", new_arr);
}
