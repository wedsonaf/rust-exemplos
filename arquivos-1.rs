fn main() {
    for item in std::fs::read_dir(".").unwrap() {
        println!("{}", item.unwrap().path().display());
    }
}
