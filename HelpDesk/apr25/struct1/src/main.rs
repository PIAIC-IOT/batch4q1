#[derive(Debug)]
struct Aftari {
    Fruit : String,
    Dates : u8,
    breakfast : bool,
}

fn main() {
    let age:u8 = 33;

    let data = ("Zaiba",12345,'A');
    //index      0       1     2
    println!("Name : {}",data.0);
    let zaiba = Aftari {
        breakfast : false,
        Fruit : "Banana".to_string(),
        Dates : 3,
    };
    println!("{:#?}",zaiba);
    println!("{:#?}",zaiba.Fruit);

}
