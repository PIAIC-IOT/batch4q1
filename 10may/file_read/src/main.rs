use std::fs::File;
use std::io;
use std::io::Read;

fn main() {
    //let reading = my_file_reading();
    let reading = read_username_from_file();
    println!("We got {:?}",reading);
}


// fn my_file_reading() -> Result<String, io::Error> {
    
//     let f = File::open("lockdown.txt");

//     let mut readdata = match f {
//         Ok(file) => file,
//         Err(e) => return Err(e),
//     };

//     let mut data = String::new();

//     match readdata.read_to_string(&mut data) {
//         Ok(_) => Ok(data),
//         Err(e) => Err(e),
//     }
// }

fn read_username_from_file() -> Result<String, io::Error> {
    // let mut f = File::open("lockdown.txt")?;
    // let mut mydata = String::new();
    // f.read_to_string(&mut mydata)?;
    // Ok(mydata)

    let mut s = String::new();

    File::open("lockdown.txt")?.read_to_string(&mut s)?;

    Ok(s)
}