use std::{fs::read_dir, io::Result};

fn main() -> Result<()> {
    for item in read_dir(".")? {
        println!("{}", item?.path().display());
    }
    Ok(())
}
