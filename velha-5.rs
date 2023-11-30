use std::io::{BufRead, BufReader, Result, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::Mutex;

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
    mostrar_status(w, *t, "Esperando o outro jogador.\n")?;
    Ok(false)
}

fn novo_cliente(mut esse: TcpStream) -> Result<()> {
    static ESPERANDO: Mutex<Option<TcpStream>> = Mutex::new(None);

    writeln!(esse, "Procurando por outro jogador")?;
    let outro = {
        let mut guard = ESPERANDO.lock().unwrap();
        if let Some(outro) = guard.take() {
            outro
        } else {
            *guard = Some(esse);
            return Ok(());
        }
    };

    struct Jogador<R> {
        reader: BufReader<R>,
        writer: TcpStream,
        valor: usize,
    }

    let jogadores = &mut [
        Jogador {
            reader: BufReader::new(esse.try_clone()?),
            writer: esse,
            valor: 1,
        },
        Jogador {
            reader: BufReader::new(outro.try_clone()?),
            writer: outro,
            valor: 2,
        },
    ];

    mostrar_status(&mut jogadores[1].writer, 0, "Esperando o outro jogador começar\n")?;

    let mut t = 0;
    for i in 0..9 {
        let j = &mut jogadores[i % 2];
        if vez(&mut j.reader, &mut j.writer, &mut t, j.valor)? {
            mostrar_status(&mut jogadores[(i + 1) % 2].writer, t, "Você perdeu!\n")?;
            return Ok(());
        }
    }

    mostrar_status(&mut jogadores[0].writer, t, "Jogo empatado!\n")?;
    mostrar_status(&mut jogadores[1].writer, t, "Jogo empatado!\n")?;

    Ok(())
}

fn main() -> Result<()> {
    let listener = TcpListener::bind("0.0.0.0:5560")?;
    for stream in listener.incoming() {
        if let Ok(s) = stream {
            std::thread::spawn(|| {
                let _ = novo_cliente(s);
            });
        }
    }
    Ok(())
}
