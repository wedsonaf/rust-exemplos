use std::rc::Rc;

fn myfunc(v: Rc<i32>) {
    println!("Valor: {v:?}");
}

fn main() {
    let c = Rc::new(10);
    std::thread::scope(|s| {
        s.spawn(|| myfunc(c));
    });
}
