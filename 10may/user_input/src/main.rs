use std::io;
fn main() {
    loop {
    println!("Type your Age");
    let mut data = String::new();
    
    io::stdin().read_line(&mut data).expect("");
    let age :u8 = match data.trim().parse() {
        Ok(value) => value,
        Err(_) => {println!("Only Integer Valid 0-255");
                    continue}
    };
    println!("age : {}",age);
    if age < 1 ||  age > 100  {
        println!("Out of range 1 and 100.");
        continue;
        } 
    println!("age : {}",age);
    break
    }
    


}
