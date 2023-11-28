fn main() {
    let mut i = 10;

    let ptr1 = &i;
    let ptr2 = &mut i;

    println!("{} {}", *ptr1, *ptr2);
}
