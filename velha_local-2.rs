use std::io::{stdin, stdout, BufRead, BufReader, Result, Write};

fn ler_jogada(r: &mut impl BufRead, w: &mut impl Write, t: usize) -> Result<usize> {
    let mut linha = String::new();
    mostrar_status(w, t, "É a sua vez, escolha onde jogar: ")?;
    loop {
        r.read_line(&mut linha)?;
        if let Ok(indice) = linha.trim().parse::<usize>() {
            if indice >= 1 && indice <= 9 && (t >> (indice - 1) * 2) & 3 == 0 {
                return Ok(indice);
            }
        }
        write!(w, "Opção invalida, escolha novamente: ")?;
        w.flush()?;
        linha.clear();
    }
}

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
    ler_jogada(&mut BufReader::new(stdin()), &mut stdout(), 0)?;
    Ok(())
}
