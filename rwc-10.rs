use std::io::{stdin, Read, Result};

fn main() -> Result<()> {
    let mut contador: u64 = 0;
    let mut palavra = false;

    for b in stdin().bytes() {
        let b = b?;
        let novo_estado = b != b' ' && b != b'\n' && b != b'\t';
        if !palavra && novo_estado {
            contador += 1;
        }
        palavra = novo_estado;
    }
    println!("{contador}");
    Ok(())
}
