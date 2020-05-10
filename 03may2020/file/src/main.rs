use std::fs::File;
fn main() {
    let mydata :f32 = File::open("faisalabad.txt");
    println!("{:#?}",mydata);
}
