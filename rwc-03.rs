use std::io::{stdin, stdout, Read, Result, Write};

fn eco<T: Read>(r: T) -> Result<()> {
    for b in r.bytes() {
        stdout().write(&[b?])?;
    }
    Ok(())
}

fn main() -> Result<()> {
    eco(stdin())
}
