fn main() {
    let mut age : u8 = 33;
    age =100;
    println!("age {}",age);
    let myage = age;
    println!("age {}",age);
    println!("my age {}",myage);
    //string literal
    let city = "Faisalabad";
    println!("city {}",city);
    let mycity = city;
    println!("city {}",city);
    println!("my city {}",mycity);

    //String Type
    let mut kcity = "Karachi is very beautiful city".to_string();
    //               0123456
    kcity.push_str("clean");
    //              78901
    println!("city {}",kcity);
    let mykcity = kcity;
    println!("city {}",kcity);
    println!("my city {}",mykcity);
}
//push()
//pop()
