fn main() {
    let value1: i32 = 1234;
    let value2: f32 = 1234.44;
    let sum1 = value1 + value2 as i32;
    let sum2 = value1 as f32 + value2 ;
    println!("Sum of {} and {} is {}",value1,value2,sum1);
    println!("Sum of {} and {} is {}",value1,value2,sum2);
}
