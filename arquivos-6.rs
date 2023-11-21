fn main() -> std::io::Result<()> {
    for item in std::fs::read_dir(".")? {
        println!("{}", item?.path().display());
    }
    Ok(())
}
