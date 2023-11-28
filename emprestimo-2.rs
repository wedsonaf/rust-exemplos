fn main() {
    let mut i = 10;

    let ptr1 = &mut i;
    let ptr2 = &i;

    println!("{} {}", *ptr1, *ptr2);
}
