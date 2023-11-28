use std::rc::Rc;

fn exemplo(v: Rc<i32>) {
    println!("Valor: {v:?}");
}

fn main() {
    let c = Rc::new(10);
    exemplo(c);
    println!("Valor: {c:?}");
}
