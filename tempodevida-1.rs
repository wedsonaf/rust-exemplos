fn main() {
    let ptr;
    {
        let v = 30;
        ptr = &v;
        println!("{}", *ptr);
    }
}
