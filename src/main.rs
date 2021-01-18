use std::io;

fn main() {
    println!("Enter a temperature.");

    let mut temperature = String::new();

    io::stdin()
        .read_line(&mut temperature)
        .expect("Failed to read temperature");

    let (value, unit) = temperature.split_at(temperature.len() - 2);

    let value: f32 = match value.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("{} is not a number", value),
    };
    let unit = unit.trim().to_lowercase();

    print!("The input temperature is: {}", temperature);

    if unit == "c" {
        c_to_f(value);
    } else if unit == "f" {
        f_to_c(value);
    } else {
        println!("Unsupported unit");
    }
}

fn c_to_f(c: f32) {
    output(c * 1.8 + 32.0, "F".to_string());
}

fn f_to_c(f: f32) {
    output((f - 32.0) / 1.8, "C".to_string());
}

fn output(t: f32, u: String) {
    println!("The output temperature is: {}{}", t, u);
}
