fn main() {
    let name = "Olá".to_string();
    println!("1. Lenth of {} is {}",name,name.len());
    let name = "ते".to_string();
    //let abcd = name.len();
    println!("2. Lenth of {} is {}",name,name.len());

    // let name1 = "Haseeb".to_string();
    // let name2 = "Hassan".to_string();
    // let fullname = name1 + &name2;
    // println!("{}",fullname);
    // println!("name1 {}",name1);
    // println!("name2 {}",name2);

    let name1 = "Haseeb".to_string();
    //           012345
    let name2 = "Hassan".to_string();
    //format!("{} {}",name1,name2);
    let mut fullname = format!("{} {}",name1,name2);
    println!("{}",fullname);
    println!("name1 {}",name1);
    println!("name2 {}",name2);
    //let first_char  = &fullname[0];
    let first_char  = &fullname[0..1];
    println!("{}",first_char);
    let first_5  = &fullname[3..5];
    println!("{}",first_5);
    
    fullname.clear();
    println!("{}",fullname);
    

}
