fn main() {
    let age = [22,33,44,55,66];
    //index    0  1  2  3  4
    let name = "Butt Batair Karahi".to_string();
    //inde      012345678901234567
    let somedata = &name[12..13];
    //                  12..<13
    println!("Complet name: {} ",name);
    println!("some data : {}",somedata);
    let somedata = &name[5..11];
    println!("some data : {}",somedata);
    
    for data in name.bytes() {
        println!("{}",data);
    }

    for data in name.chars() {
        println!("{}",data);
    }

}
