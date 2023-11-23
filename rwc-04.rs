use std::io::{stdin, stdout, Read, Result, Write};

fn eco(r: impl Read) -> Result<()> {
    for b in r.bytes() {
        stdout().write(&[b?])?;
    }
    Ok(())
}

fn main() -> Result<()> {
    eco(stdin())
}
