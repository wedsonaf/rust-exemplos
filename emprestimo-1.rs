fn main() {
    let i = 10;

    let ptr1 = &i;
    let ptr2 = &i;

    println!("{} {}", *ptr1, *ptr2);
}
