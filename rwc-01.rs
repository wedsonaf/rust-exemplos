use std::io::{stdin, stdout, Read, Result, Write};

fn main() -> Result<()> {
    for b in stdin().bytes() {
        stdout().write(&[b?])?;
    }
    Ok(())
}
