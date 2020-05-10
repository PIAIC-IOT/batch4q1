use std::io;
fn main() {
    println!("Enter your lucky number");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("");
    
    //let data:u8 = input.trim().parse().expect("We are in trouble");   
    let data:u8 = input.trim().parse().unwrap();   

    let data:u8 = match input.trim().parse() {
        Ok(a) => a,
        Err(_)   => 0
    };
    
    // match variable{
    //     1 => a
    //     2 => b
    // }

    if data == 99{
        panic!("We are in trouble at {}",data);    
    }

    println!("{}",data);
}
