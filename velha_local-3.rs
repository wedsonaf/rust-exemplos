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

fn vez(r: &mut impl BufRead, w: &mut impl Write, t: &mut usize, x: usize) -> Result<bool> {
    *t |= x << (ler_jogada(r, w, *t)? - 1) * 2;
    let cmp = *t >> (x - 1);
    for v in [0x15, 0x540, 0x15000, 0x1041, 0x4104, 0x10410, 0x10101, 0x1110] {
        if cmp & v == v {
            mostrar_status(w, *t, "Parabéns, você ganhou!\n")?;
            return Ok(true);
        }
    }
    Ok(false)
}

fn main() -> Result<()> {
    let mut breader = BufReader::new(stdin());
    let mut tabuleiro = 0;
    for i in [1, 2].iter().cycle().take(9) {
        if vez(&mut breader, &mut stdout(), &mut tabuleiro, *i)? {
            return Ok(());
        }
    }
    writeln!(stdout(), "Jogo empatado!")
}
