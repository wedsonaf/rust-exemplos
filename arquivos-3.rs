fn main() -> std::io::Result<()> {
    match std::fs::read_dir(".") {
        Err(e) => return Err(e),
        Ok(itens) => {
            for item in itens {
                println!("{}", item.unwrap().path().display());
            }
        }
    }
    Ok(())
}
