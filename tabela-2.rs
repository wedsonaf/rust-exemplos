fn main() {
    let mut celsius = 0;

    while celsius <= 100 {
        let fahrenheit = celsius * 9 / 5 + 32;
        println!("{celsius}° C = {fahrenheit} F");
        celsius = celsius + 10;
    }
}
