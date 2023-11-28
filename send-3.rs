fn myfunc(v: &i32) {
    println!("Valor: {v:?}");
}

fn main() {
    let c = 10;
    std::thread::scope(|s| {
        s.spawn(|| myfunc(&c));
    });
}
