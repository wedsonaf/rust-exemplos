use std::io::{BufReader, Result, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::Mutex;

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
