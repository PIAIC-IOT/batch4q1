fn main() {
    let mut one = "Ola ".to_string();
    println!("length of {} is {}",one,one.len());
    one.push_str("saday kolon");
    println!("length of {} is {}",one,one.len());
    one.push('z');
    println!("length of {} is {}",one,one.len());

    let two = "Olá".to_string();
    println!("length of {} is {}",two,two.len());
    
    let three = "ते".to_string();
    println!("length of {} is {}",three,three.len());
    
    let name1 = "Faisal".to_string();
    let name2 = "Abad".to_string();
    let city = name1 + &name2;
    println!("My beautiful city name is {}",city);
    //println!("My beautiful city name is {}",name1);
    let d1 = "Banana ".to_string();
    let d2 = "Pakoray".to_string();
    let d3 = "Lassi".to_string();
    let iftari = format!("{} {} {}",d1,d2,d3);
    println!("Iftari of Zeeshan Noor is : {}",iftari);
    println!("Iftari of Zeeshan Noor is : {}",d1);


}
