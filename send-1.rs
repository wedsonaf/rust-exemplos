use std::cell::Cell;

fn myfunc(v: Cell<i32>) {
    println!("Valor: {v:?}");
}

fn main() {
    let c = Cell::new(10);
    std::thread::scope(|s| {
        s.spawn(|| myfunc(c));
    });
}
