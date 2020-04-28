//use std::io;
fn main() {
    let mut salary : Vec<u32> = Vec::new();

    println!("Complete Vector {:?}",salary);
    salary.push(12000);
    println!("Complete Vector {:?}",salary);
    let mut data :u32 = 5000;

    while data > 0 {
        println!("Enter your salary");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("");
        data = input.trim().parse().expect("You are in trouble");
        //data = input.trim().parse().unwrap();
        salary.push(data);
    }

    for data in &salary {
        println!(" {}",data);
    }

    let age = [22,33,44];
    for abcd in &age {
        println!("{}",abcd);
    }
    println!("End of Loop");

    for data in &mut salary {
        * data += 1000;
    }

    println!("Complete Vector {:?}",salary);
    println!("End of Program");
}
