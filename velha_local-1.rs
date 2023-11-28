use std::io::{stdout, Result, Write};

fn mostrar_status(out: &mut impl Write, tabuleiro: usize, status: &str) -> Result<()> {
    for i in 0..9 {
        write!(out, " {} ", b" xo"[(tabuleiro >> i * 2) & 3] as char)?;
        if i % 3 != 2 {
            write!(out, "|")?;
        } else {
            writeln!(out, "{}", ["\n---+---+---", ""][i >> 3])?;
        }
    }
    write!(out, "{}", status)?;
    out.flush()
}

fn main() -> Result<()> {
    mostrar_status(&mut stdout(), 0, "Testando 1\n")?;
    mostrar_status(&mut stdout(), 1, "Testando 2\n")?;
    mostrar_status(&mut stdout(), 2, "Testando 3\n")?;
    Ok(())
}
