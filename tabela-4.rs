fn main() {
    let intervalo = 0..100;
    for celsius in intervalo {
        let fahrenheit = celsius * 9 / 5 + 32;
        println!("{celsius}° C = {fahrenheit} F");
    }
}
