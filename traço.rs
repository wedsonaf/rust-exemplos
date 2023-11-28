trait MeuTraço {
    const N: usize;
    type Retorno;

    fn exemplo() -> Self::Retorno;

    fn opcional() -> u32 {
        0
    }
}

impl MeuTraço for char {
    const N: usize = 50;
    type Retorno = i32;

    fn exemplo() -> i32 {
        -1
    }
}

fn mostrar_exemplo<T: MeuTraço>(_v: T) {
    println!("{} {}", T::N, T::opcional());
}

fn main() {
    mostrar_exemplo('a');
}
