fn main() {
    // let mut fruit = ["Water Melon","Berries Falsa"];
    //index            0              1               2
    // println!("{}",fruit[1]);
    // fruit[1]="Melon";
    // fruit[3]="Banana";
    // println!("{}",fruit[2]);

    // let mut fruit1 = ("Water Melon",40,"Kg",2.5);
    //              0           1   2    3
    let mut data: Vec<String> = Vec::new();
    println!("{:?}",data);
    data.push("Mango".to_string());
    data.push("Apple".to_string());
    println!("{:?}",data);
    data.pop();
    println!("{:?}",data);
    let mut myincome: Vec<u32> = Vec::new();
    myincome.push(100);
    myincome.push(50);
    myincome.push(200);
    myincome.push(150);
    let mut sum:u32 = 0;
    for val in 0..myincome.len() {
        sum = sum+ myincome[val];
    }
    let mut sum2:u32 = 0;
    for val in myincome.iter(){
        sum2 = sum2 + val;
    }

    println!("Complete Vector{:?}",myincome);
    println!("Sum : {}",sum);
    println!("Sum2 : {}",sum2);

}
