use std::fs::File;
use std::io::ErrorKind;

fn main() {
    // let data = File::open("eid.txt");
    // let myfile = match data {
    //     Ok(yourfile) => yourfile,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("eid.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file: {:?}", e),
    //         },
    //         other_error => panic!("Problem opening the file: {:?}", other_error)
    //     }
    // };
    // println!("We got {:#?}",myfile);
    let myfile = File::open("hello.txt").unwrap_or_else(|myerror| {
        if myerror.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|zerror| {
                panic!("Problem creating the file: {:?}", zerror);
            })
        } else {
            panic!("Problem opening the file: {:?}", myerror);
        }
    });
    println!("We got {:#?}",myfile);
}
