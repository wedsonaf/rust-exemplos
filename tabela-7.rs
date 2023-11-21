fn main() {
    for celsius in (0..=100).step_by(10) {
        let fahrenheit = celsius * 9 / 5 + 32;
        println!("{celsius}° C = {fahrenheit} F");
    }
}
