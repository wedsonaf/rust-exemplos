use std::io::Result;
use std::net::{TcpListener, TcpStream};

fn novo_cliente(_esse: TcpStream) -> Result<()> {
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
