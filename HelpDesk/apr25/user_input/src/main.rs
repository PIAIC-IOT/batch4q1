use std::io;

fn main() {
    println!("Enter you name :");
    let mut get_input = String::new();
    io::stdin().read_line(&mut get_input).expect("Failed to read line");
    println!("You entered {}",get_input);
    println!("Enter you age :");
    let mut get_age = String::new();
    io::stdin().read_line(&mut get_age).expect("Failed to read line");
    println!("You entered {:#?}",get_age);
    let age : i32 = get_age.trim().parse().expect("We are in trouble");
    let age2 : i32 = get_age.trim().parse().unwrap();

    // let length = get_input.len();
    // let arr = [22,33,44]'
    // let index = arr.get(2);

    println!("You entered {:#?}",age);

    
}
