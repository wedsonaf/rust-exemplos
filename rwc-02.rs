use std::io::{stdin, stdout, Read, Result, Stdin, Write};

fn eco(r: Stdin) -> Result<()> {
    for b in r.bytes() {
        stdout().write(&[b?])?;
    }
    Ok(())
}

fn main() -> Result<()> {
    eco(stdin())
}
