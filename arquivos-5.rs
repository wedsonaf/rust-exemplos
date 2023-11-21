fn main() -> std::io::Result<()> {
    let itens = std::fs::read_dir(".")?;
    for item in itens {
        println!("{}", item?.path().display());
    }
    Ok(())
}
