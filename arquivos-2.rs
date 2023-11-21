fn main() {
    match std::fs::read_dir(".") {
        Err(e) => {
            println!("Erro ao tentar ler arquivos: {e}");
        }
        Ok(itens) => {
            for item in itens {
                println!("{}", item.unwrap().path().display());
            }
        }
    }
}
