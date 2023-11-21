use std::{fs::read_dir, io::Result};

fn main() -> Result<()> {
    for item in read_dir(".")? {
        if let Ok(i) = item {
            println!("{}", i.path().display());
        }
    }
    Ok(())
}
