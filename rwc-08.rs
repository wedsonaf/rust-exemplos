use std::io::{stdin, Read, Result};

fn main() -> Result<()> {
    let mut contador: u64 = 0;
    for _ in stdin().bytes() {
        contador += 1;
    }
    println!("{contador}");
    Ok(())
}
