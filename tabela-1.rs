fn main() {
    let mut celsius = 0;

    loop {
        let fahrenheit = celsius * 9 / 5 + 32;
        println!("{celsius}° C = {fahrenheit} F");
        celsius = celsius + 10;

        if celsius > 100 {
            break;
        }
    }
}
