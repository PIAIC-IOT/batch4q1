use std::fs::File;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("lockdown.txt")?;
    println!("{:?}",f);
    Ok(())
}
