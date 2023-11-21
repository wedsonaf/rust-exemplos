use std::{fs::read_dir, io::Result};

fn main() -> Result<()> {
    for item in read_dir(".")? {
        match item {
            Err(_) => {}
            Ok(i) => println!("{}", i.path().display()),
        }
    }
    Ok(())
}
