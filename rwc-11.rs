use std::io::{stdin, Read, Result};

trait Contador {
    fn contar(&mut self, byte: u8);
    fn contador(&self) -> u64;
}

struct Bytes {
    contador: u64,
}

impl Bytes {
    fn new() -> Bytes {
        Bytes { contador: 0 }
    }
}

impl Contador for Bytes {
    fn contador(&self) -> u64 {
        return self.contador;
    }

    fn contar(&mut self, _byte: u8) {
        self.contador += 1;
    }
}

struct Linhas {
    contador: u64,
}

impl Linhas {
    fn new() -> Linhas {
        Linhas { contador: 0 }
    }
}

impl Contador for Linhas {
    fn contador(&self) -> u64 {
        return self.contador;
    }

    fn contar(&mut self, byte: u8) {
        if byte == b'\n' {
            self.contador += 1;
        }
    }
}

struct Palavras {
    contador: u64,
    palavra: bool,
}

impl Palavras {
    fn new() -> Palavras {
        Palavras {
            contador: 0,
            palavra: false,
        }
    }
}

impl Contador for Palavras {
    fn contador(&self) -> u64 {
        return self.contador;
    }

    fn contar(&mut self, b: u8) {
        let novo_estado = b != b' ' && b != b'\n' && b != b'\t';
        if !self.palavra && novo_estado {
            self.contador += 1;
        }
        self.palavra = novo_estado;
    }
}

fn contar(r: impl Read, contadores: &mut [&mut dyn Contador]) -> Result<()> {
    for b in r.bytes() {
        let b = b?;
        for c in contadores.iter_mut() {
            c.contar(b);
        }
    }
    Ok(())
}

fn mostrar(contadores: &[&mut dyn Contador]) {
    for c in contadores {
        print!("{} ", c.contador());
    }
    println!("");
}

fn main() -> Result<()> {
    let contadores: &mut [&mut dyn Contador] =
        &mut [&mut Linhas::new(), &mut Palavras::new(), &mut Bytes::new()];
    contar(stdin(), contadores)?;
    mostrar(contadores);
    Ok(())
}
