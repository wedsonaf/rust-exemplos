use std::io::{stdin, Read, Result};

fn main() -> Result<()> {
    let mut contador: u64 = 0;
    for b in stdin().bytes() {
        if b? == b'\n' {
            contador += 1;
        }
    }
    println!("{contador}");
    Ok(())
}
