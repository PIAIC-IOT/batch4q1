use std::io;
fn main() {
    let age = [22,24,26];
    //index    0  1  2  3
    //length 3
    println!("Complete Array {:?}",age);
    println!("One Element {:?}",age[1]);
    println!("One Element {:?}",age.get(100));

    let mut salary : Vec<u32> = Vec::new();

    println!("Complete Vector {:?}",salary);
    salary.push(12000);
    println!("Complete Vector {:?}",salary);
    let mut data :u32 = 5000;

    while data > 0 {
        println!("Enter your salary");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("");
        data = input.trim().parse().expect("You are in trouble");
        //data = input.trim().parse().unwrap();
        salary.push(data);
    }

    let name = "Zeeshan Noor".to_string();
    let length = name.len();

    println!("Complete Vector {:?}",salary);
    salary.pop();
    salary.pop();
    salary.pop();
    println!("Complete Vector {:?}",salary);

    let mut price = vec![100,150,250];
    println!("Price Vector {:?}",price);
    price.push(300);
    println!("Price Vector {:?}",price);
    price.pop();
    println!("Price Vector {:?}",price);

    println!("Price Vector {:?}",price.get(250));
    println!("Price Vector {:?}",price.get(1));

    let name = "Abid".to_string();
    let name1 = String::from("Mehmood");
    let name2 = String::new();


}
