fn menor<'a>(x: &'a mut i32, y: &'a mut i32) -> &'a mut i32 {
    if *x < *y {
        x
    } else {
        y
    }
}

fn main() {
    let mut i = 30;
    let mut j = 40;

    let p = menor(&mut i, &mut j);
    println!("{}", *p);

    *p = 50;
    println!("{j}");
}
